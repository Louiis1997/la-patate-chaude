use rand::Rng;
use shared::challenges::hash_cash::MD5HashCash as MD5HashCashChallenge;
use shared::challenges::monstrous_maze::MonstrousMaze as MonstrousMazeChallenge;
use shared::challenges::{
    Challenge, Challenges, Challenges::MD5HashCash as MD5HashCashChallengeEnum,
    Challenges::MonstrousMaze as MonstrousMazeChallengeEnum,
};
use shared::MD5HashCashInput;
use shared::Message;
use shared::MonstrousMazeInput;
use shared::PublicLeaderBoard;
use shared::PublicPlayer;
use shared::SubscribeError;
use shared::SubscribeResult;
use shared::Welcome;
use shared::{
    BadResult, ChallengeAnswer, ChallengeResult, ChallengeValue, EndOfGame, Ok,
    ReportedChallengeResult, RoundSummary,
};
use shared::{Challenge::MD5HashCash, Challenge::MonstrousMaze};
use std::net::{TcpListener, TcpStream};
use std::{process, thread};

struct PublicPlayerTCPStream {
    player: PublicPlayer,
    stream: TcpStream,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878");
    let listener = match listener {
        Ok(l) => l,
        Err(_err) => panic!("Cannot bind: {_err}"),
    };
    static mut PUBLIC_PLAYERS_TCP_STREAM: Vec<PublicPlayerTCPStream> = Vec::new();
    static mut PUBLIC_PLAYERS: Vec<PublicPlayer> = Vec::new();
    static mut NB_PLAYED_CHALLENGES: i32 = 0;
    static mut CURRENT_CHALLENGE: Challenges = Challenges::None();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => unsafe {
                println!("{}", stream.peer_addr().unwrap());
                thread::spawn(move || loop {
                    handle_client(&stream);
                });
            },
            Err(_err) => {
                panic!("Connection failed: {}", _err);
            }
        }
    }

    unsafe fn handle_client(stream: &TcpStream) {
        let response = shared::read_message(&stream);
        let reported_challenges: Vec<ReportedChallengeResult> = vec![];
        match serde_json::from_str(&response) {
            Ok(response) => match response {
                Message::Hello => {
                    shared::send_message(&stream, Message::Welcome(Welcome { version: 1 }));
                }
                Message::Subscribe(subscribe) => {
                    let public_player = create_player(
                        subscribe.name,
                        &mut PUBLIC_PLAYERS,
                        stream.try_clone().unwrap(),
                        &mut PUBLIC_PLAYERS_TCP_STREAM,
                    );
                    shared::send_message(&stream, public_player);
                    if PUBLIC_PLAYERS.len() >= 2 {
                        println!("{}", " ==== Starting game ==== ");
                        send_to_all_players(Message::PublicLeaderBoard(PublicLeaderBoard(
                            PUBLIC_PLAYERS.clone(),
                        )));
                        let random_player = get_random_next_player(PUBLIC_PLAYERS.clone());
                        let random_player_stream = PUBLIC_PLAYERS_TCP_STREAM
                            .iter()
                            .find(|player| player.player.stream_id == random_player.stream_id);
                        let random_player_stream =
                            random_player_stream.unwrap().stream.try_clone().unwrap();
                        CURRENT_CHALLENGE = launch_game(get_random_game(), random_player_stream);
                    }
                }
                Message::ChallengeResult(challenge_result) => {
                    let current_player_address: String;
                    match stream.try_clone().unwrap().peer_addr() {
                        Ok(address) => current_player_address = address.to_string(),
                        Err(_err) => panic!("Failed to get peer address"),
                    };
                    let current_player: PublicPlayer;
                    match get_current_player(current_player_address.clone()) {
                        Some(player) => {
                            current_player = player;
                        }
                        None => {
                            panic!("Failed to get current player")
                        }
                    };

                    match CURRENT_CHALLENGE {
                        Challenges::None(..) => {
                            panic!("Current challenge is None !");
                        }
                        _ => {}
                    }
                    let next_player_stream = handle_client_challenge_response(
                        &stream,
                        current_player,
                        CURRENT_CHALLENGE.clone(),
                        challenge_result.clone(),
                        reported_challenges,
                    );
                    NB_PLAYED_CHALLENGES += 1;
                    if NB_PLAYED_CHALLENGES >= 3 {
                        send_to_all_players(Message::EndOfGame(EndOfGame {
                            leader_board: PublicLeaderBoard(PUBLIC_PLAYERS.clone()),
                        }));
                        process::exit(0);
                    }
                    CURRENT_CHALLENGE = launch_game(get_random_game(), next_player_stream);
                }
                _ => {}
            },
            Err(err) => {
                panic!("Failed to deserialize the message received: {}", err)
            }
        }
    }

    unsafe fn get_current_player(address: String) -> Option<PublicPlayer> {
        let current_player = PUBLIC_PLAYERS
            .iter()
            .find(|player| player.stream_id == address)?;
        return Some(current_player.clone());
    }
    fn generate_challenge_value(
        success: bool,
        used_time: f64,
        current_player_name: String,
    ) -> ChallengeValue {
        if success {
            ChallengeValue::Ok {
                0: Ok {
                    used_time,
                    next_target: current_player_name,
                },
            }
        } else {
            ChallengeValue::BadResult {
                0: BadResult {
                    used_time,
                    next_target: current_player_name,
                },
            }
        }
    }

    fn create_player(
        name: String,
        public_players: &mut Vec<PublicPlayer>,
        stream: TcpStream,
        public_players_stream: &mut Vec<PublicPlayerTCPStream>,
    ) -> Message {
        if !name.is_ascii() {
            return Message::SubscribeResult(SubscribeResult::Err(SubscribeError::InvalidName));
        }
        for player in public_players.clone() {
            if player.name.eq(&name) {
                return Message::SubscribeResult(SubscribeResult::Err(
                    SubscribeError::AlreadyRegistered,
                ));
            }
        }
        let player = PublicPlayer {
            name,
            stream_id: stream.peer_addr().unwrap().to_string(),
            score: 0,
            steps: 0,
            is_active: true,
            total_used_time: 0.0,
        };
        public_players.push(player.clone());
        public_players_stream.push(PublicPlayerTCPStream { player, stream });

        return Message::SubscribeResult(SubscribeResult::Ok);
    }

    fn update_player_score(player: &mut PublicPlayer, won: bool) {
        player.score += if won { 0 } else { -1 };
    }

    fn increment_player_steps(player: &mut PublicPlayer) {
        player.steps += 1;
    }

    unsafe fn replace_player_in_players(player: &PublicPlayer) {
        for i in 0..PUBLIC_PLAYERS.len() {
            if PUBLIC_PLAYERS[i].stream_id == player.stream_id {
                PUBLIC_PLAYERS[i] = player.clone();
            }
        }
    }

    unsafe fn update_player_in_player_list(
        success: bool,
        addr: String,
        used_time: f64,
    ) -> Option<PublicPlayer> {
        let current_player = PUBLIC_PLAYERS
            .iter()
            .find(|player| player.stream_id == addr)?;
        let mut current_player = current_player.clone();
        update_player_score(&mut current_player, success);
        increment_player_steps(&mut current_player);
        replace_player_in_players(&current_player);
        current_player.total_used_time += used_time;
        Some(current_player)
    }

    fn get_next_player(
        challenge_result: ChallengeResult,
        public_players: Vec<PublicPlayer>,
    ) -> Option<PublicPlayer> {
        if challenge_result.next_target == "" {
            return Some(get_random_next_player(public_players));
        }
        let next_player_result = public_players
            .iter()
            .find(|player| player.name == challenge_result.next_target)?;
        Some(next_player_result.clone())
    }

    fn get_random_next_player(public_players: Vec<PublicPlayer>) -> PublicPlayer {
        let mut rng = rand::thread_rng();
        let active_players: Vec<&PublicPlayer> = public_players
            .iter()
            .filter(|player| player.is_active)
            .collect();
        let random_index: usize = rng.gen_range(0..active_players.len());
        return active_players[random_index].clone();
    }

    unsafe fn send_to_all_players(message: Message) {
        PUBLIC_PLAYERS_TCP_STREAM
            .iter_mut()
            .for_each(|public_player_tcp_stream| {
                shared::send_message(&public_player_tcp_stream.stream, message.clone());
            });
    }

    unsafe fn send_round_summarize(
        challenge_name: String,
        reported_challenges: Vec<ReportedChallengeResult>,
    ) {
        send_to_all_players(Message::RoundSummary(RoundSummary {
            challenge: challenge_name,
            chain: reported_challenges.clone(),
        }));
    }

    unsafe fn handle_client_challenge_response(
        stream: &TcpStream,
        current_player: PublicPlayer,
        current_challenge: Challenges,
        challenge_result: ChallengeResult,
        mut reported_challenges: Vec<ReportedChallengeResult>,
    ) -> TcpStream {
        let challenge_timer = std::time::Instant::now();

        let current_challenge_used_time = challenge_timer.elapsed().as_secs() as f64;
        match challenge_result.clone().answer {
            ChallengeAnswer::MD5HashCash {
                0: hash_cash_answer,
            } => {
                let challenge_input;
                match current_challenge {
                    MD5HashCashChallengeEnum(challenge) => challenge_input = challenge.input,
                    _ => {
                        panic!("Error in handle_player_challenge client response")
                    }
                }
                let current_challenge = MD5HashCashChallenge::new(challenge_input.clone());
                let success = current_challenge.verify(&hash_cash_answer);
                reported_challenges.push(ReportedChallengeResult {
                    name: MD5HashCashChallenge::name(),
                    value: generate_challenge_value(
                        success,
                        current_challenge_used_time,
                        current_player.name.clone(),
                    ),
                });
                match update_player_in_player_list(
                    success,
                    stream.peer_addr().unwrap().to_string(),
                    current_challenge_used_time,
                ) {
                    Some(_current_player) => {
                        match get_next_player(challenge_result.clone(), PUBLIC_PLAYERS.clone()) {
                            Some(next_player) => {
                                send_round_summarize(
                                    MD5HashCashChallenge::name(),
                                    reported_challenges.clone(),
                                );
                                let next_player_stream =
                                    PUBLIC_PLAYERS_TCP_STREAM.iter().find(|player| {
                                        player.player.stream_id == next_player.stream_id
                                    });
                                return next_player_stream.unwrap().stream.try_clone().unwrap();
                            }
                            None => {
                                panic!("No more players ???");
                            }
                        }
                    }
                    None => {
                        panic!("Failed to update current player in player list");
                    }
                }
            }
            ChallengeAnswer::MonstrousMaze {
                0: monstrous_maze_answer,
            } => {
                let challenge_input;
                match current_challenge {
                    MonstrousMazeChallengeEnum(challenge) => challenge_input = challenge.input,
                    _ => {
                        panic!("Error in handle_player_challenge client response")
                    }
                }
                let current_challenge = MonstrousMazeChallenge::new(challenge_input.clone());
                let success = current_challenge.verify(&monstrous_maze_answer);
                reported_challenges.push(ReportedChallengeResult {
                    name: MonstrousMazeChallenge::name(),
                    value: generate_challenge_value(
                        success,
                        current_challenge_used_time,
                        current_player.name.clone(),
                    ),
                });
                match update_player_in_player_list(
                    success,
                    stream.peer_addr().unwrap().to_string(),
                    current_challenge_used_time,
                ) {
                    Some(_current_player) => {
                        match get_next_player(challenge_result.clone(), PUBLIC_PLAYERS.clone()) {
                            Some(next_player) => {
                                send_round_summarize(
                                    MonstrousMazeChallenge::name(),
                                    reported_challenges.clone(),
                                );
                                let next_player_stream =
                                    PUBLIC_PLAYERS_TCP_STREAM.iter().find(|player| {
                                        player.player.stream_id == next_player.stream_id
                                    });
                                return next_player_stream.unwrap().stream.try_clone().unwrap();
                            }
                            None => {
                                panic!("No more players ???");
                            }
                        }
                    }
                    None => {
                        panic!("Failed to update current player in player list");
                    }
                }
            }
        }
    }

    fn get_random_game() -> Challenges {
        let mut rng = rand::thread_rng();
        let challenge_index: usize = rng.gen_range(0..=1);
        match challenge_index {
            0 => {
                let challenge_input = MD5HashCashInput {
                    complexity: 9,
                    message: "hello".to_string(),
                };
                let challenge =
                    MD5HashCashChallengeEnum(MD5HashCashChallenge::new(challenge_input.clone()));
                return challenge;
            }
            1 => {
                let challenge_input = MonstrousMazeInput {
                    endurance: 10,
                    grid: "|I   X|".to_string(),
                };
                let challenge = MonstrousMazeChallengeEnum(MonstrousMazeChallenge::new(
                    challenge_input.clone(),
                ));
                return challenge;
            }
            _ => panic!("Not implemented"),
        };
    }

    fn launch_game(challenge: Challenges, stream: TcpStream) -> Challenges {
        return match challenge {
            Challenges::MD5HashCash(challenge) => {
                shared::send_message(
                    &stream,
                    Message::Challenge(MD5HashCash(challenge.clone().input)),
                );
                Challenges::MD5HashCash(challenge)
            }
            Challenges::MonstrousMaze(challenge) => {
                shared::send_message(
                    &stream,
                    Message::Challenge(MonstrousMaze(challenge.clone().input)),
                );
                Challenges::MonstrousMaze(challenge)
            }
            _ => Challenges::None(),
        };
    }
}
