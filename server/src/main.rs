use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;
use shared::{BadResult, ChallengeAnswer, ChallengeResult, ChallengeValue, EndOfGame, Ok, ReportedChallengeResult, RoundSummary};
use shared::MD5HashCashInput;
use shared::Message;
use shared::MonstrousMazeInput;
use shared::PublicLeaderBoard;
use shared::PublicPlayer;
use shared::SubscribeError;
use shared::SubscribeResult;
use shared::Welcome;
use shared::{Challenge::MD5HashCash, Challenge::MonstrousMaze};
use shared::challenges::{Challenge, Challenges, MD5HashCash as MD5HashCashChallenge, MonstrousMaze as MonstrousMazeChallenge, Challenges::MonstrousMaze as MonstrousMazeChallengeEnum, Challenges::MD5HashCash as MD5HashCashChallengeEnum};
use rand::Rng;

struct PublicPlayerTCPStream {
    player: PublicPlayer,
    stream: TcpStream,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878");
    let listener = match listener {
        Ok(l) => l,
        Err(_err) => panic!("Cannot bind: {_err}")
    };
    static mut PUBLIC_PLAYERS_TCP_STREAM: Vec<PublicPlayerTCPStream> = Vec::new();
    static mut PUBLIC_PLAYERS: Vec<PublicPlayer> = Vec::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("{}", stream.peer_addr().unwrap());
        thread::spawn(move || {
            handle_client(stream);
        });
    }

    fn handle_client(stream: TcpStream) {
        loop {
            let response = shared::read_message(&stream);
            let response = from_utf8((&response).as_ref()).unwrap();
            let response = serde_json::from_str(response).unwrap();
            match response {
                Message::Hello => {
                    shared::write_message(&stream, Message::Welcome(Welcome { version: 1 })).expect("Failed to send welcome message");
                }
                Message::Subscribe(subscribe) => unsafe {
                    let public_player = create_player(subscribe.name, &mut PUBLIC_PLAYERS, stream.try_clone().unwrap(), &mut PUBLIC_PLAYERS_TCP_STREAM);
                    shared::write_message(&stream, public_player).expect("Failed to send SubscribeResult message to client");
                    if PUBLIC_PLAYERS.len() >= 2 {
                        send_to_all_players(Message::PublicLeaderBoard(PublicLeaderBoard(PUBLIC_PLAYERS.clone())));
                        let random_player = get_random_next_player(PUBLIC_PLAYERS.clone());
                        let random_player_stream = PUBLIC_PLAYERS_TCP_STREAM.iter().find(|player| player.player.stream_id == random_player.stream_id);
                        let random_player_stream = random_player_stream.unwrap().stream.try_clone().unwrap();
                        handle_player_challenge(random_player_stream, 0, vec![]);
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    unsafe fn handle_player_challenge(stream: TcpStream, mut played_challenges: usize, mut reported_challenges: Vec<ReportedChallengeResult>) {
        println!("played_challenges : {}", played_challenges);
        println!("Stream address: {}", stream.peer_addr().unwrap());
        if played_challenges >= 3 {
            send_to_all_players(Message::EndOfGame(EndOfGame { leader_board: PublicLeaderBoard(PUBLIC_PLAYERS.clone()) }));
            return;
        }

        let current_player_address: String;
        match stream.peer_addr() {
            Ok(address) => current_player_address = address.to_string(),
            Err(_err) => panic!("Failed to get peer address")
        };
        let current_player: PublicPlayer;
        match get_current_player(current_player_address.clone()) {
            Some(player) => { current_player = player;}
            None => { panic!("Failed to get current player") }
        };
        let current_challenge: Challenges;
        let mut rng = rand::thread_rng();
        let challenge_index: usize = rng.gen_range(0..1);
        match challenge_index {
            0 => {
                let challenge_input = MD5HashCashInput {
                    complexity: 9,
                    message: "hello".to_string(),
                };
                current_challenge = MD5HashCashChallengeEnum(MD5HashCashChallenge::new(challenge_input.clone()));
                shared::write_message(&stream, Message::Challenge(MD5HashCash(challenge_input))).expect("Failed to send Challenge message to client");
            }
            1 => {
                let challenge_input = MonstrousMazeInput {
                    endurance: 10,
                    grid: "|I   X|".to_string(),
                };
                current_challenge = MonstrousMazeChallengeEnum(MonstrousMazeChallenge::new(challenge_input.clone()));
                shared::write_message(&stream, Message::Challenge(MonstrousMaze(challenge_input))).expect("Failed to send Challenge message to client");
            }
            _ => panic!("Not implemented"),
        };
        let challenge_timer = std::time::Instant::now();

        // Listen with read_message -> ChallengeResult(...)
        let response = shared::read_message(&stream);
        let current_challenge_used_time = challenge_timer.elapsed().as_secs() as f64;
        let response = from_utf8((&response).as_ref()).unwrap();
        let response = serde_json::from_str(response).unwrap();
        match response {
            Message::ChallengeResult(challenge_result) => match challenge_result.clone().answer {
                ChallengeAnswer::MD5HashCash { 0: hash_cash_answer } => {
                    let challenge_input;
                    match current_challenge {
                        MD5HashCashChallengeEnum(challenge) => {
                            challenge_input = challenge.input
                        }
                        _ => {
                            panic!("Error in handle_player_challenge client response")
                        }
                    }
                    let current_challenge = MD5HashCashChallenge::new(challenge_input.clone());
                    let success = current_challenge.verify(&hash_cash_answer);
                    reported_challenges.push(ReportedChallengeResult {
                        name: MD5HashCashChallenge::name(),
                        value: generate_challenge_value(success, current_challenge_used_time, current_player.name.clone()),
                    });
                    match update_player_in_player_list(success, stream.peer_addr().unwrap().to_string(), current_challenge_used_time) {
                        Some(current_player) => {
                            match get_next_player(challenge_result.clone(), PUBLIC_PLAYERS.clone()) {
                                Some(next_player) => {
                                    println!("Current player name : {}\n Next player name : {}", current_player.name, next_player.name);
                                    send_round_summarize(MD5HashCashChallenge::name(), reported_challenges.clone());
                                    played_challenges += 1;
                                    let next_player_stream = PUBLIC_PLAYERS_TCP_STREAM.iter().find(|player| player.player.stream_id == next_player.stream_id);
                                    let next_player_stream = next_player_stream.unwrap().stream.try_clone().unwrap();
                                    println!("Next player stream address: {}", next_player_stream.peer_addr().unwrap());
                                    return handle_player_challenge(next_player_stream, played_challenges, reported_challenges.clone());
                                }
                                None => {
                                    println!("No more players ???");
                                }
                            }
                        }
                        None => {
                            println!("Failed to update current player in player list");
                            return;
                        }
                    }
                },
                ChallengeAnswer::MonstrousMaze { 0: monstrous_maze_answer } => {
                    let challenge_input;
                    match current_challenge {
                        MonstrousMazeChallengeEnum(challenge) => {
                            challenge_input = challenge.input
                        }
                        _ => {
                            panic!("Error in handle_player_challenge client response")
                        }
                    }
                    let current_challenge = MonstrousMazeChallenge::new(challenge_input.clone());
                    let success = current_challenge.verify(&monstrous_maze_answer);
                    reported_challenges.push(ReportedChallengeResult {
                        name: MD5HashCashChallenge::name(),
                        value: generate_challenge_value(success, current_challenge_used_time, current_player.name.clone()),
                    });
                    match update_player_in_player_list(success, stream.peer_addr().unwrap().to_string(), current_challenge_used_time) {
                        Some(current_player) => {
                            match get_next_player(challenge_result.clone(), PUBLIC_PLAYERS.clone()) {
                                Some(next_player) => {
                                    println!("Current player name : {}\n Next player name : {}", current_player.name, next_player.name);
                                    send_round_summarize(MonstrousMazeChallenge::name(), reported_challenges.clone());
                                    played_challenges += 1;
                                    let next_player_stream = PUBLIC_PLAYERS_TCP_STREAM.iter().find(|player| player.player.stream_id == next_player.stream_id);
                                    let next_player_stream = next_player_stream.unwrap().stream.try_clone().unwrap();
                                    return handle_player_challenge(next_player_stream, played_challenges, reported_challenges.clone());
                                }
                                None => {
                                    println!("No more players ???");
                                }
                            }
                        }
                        None => {
                            println!("Failed to update current player in player list");
                            return;
                        }
                    }
                }
            },
            _ => {}
        }
    }
    unsafe fn get_current_player(address: String) -> Option<PublicPlayer> {
        let current_player = PUBLIC_PLAYERS.iter().find(|player| player.stream_id == address)?;
        return Some(current_player.clone());
    }
    fn generate_challenge_value(success: bool, used_time: f64, current_player_name: String) -> ChallengeValue {
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

    fn create_player(name: String, public_players: &mut Vec<PublicPlayer>, stream: TcpStream, public_players_stream: &mut Vec<PublicPlayerTCPStream>) -> Message {
        if !name.is_ascii() {
            return Message::SubscribeResult(SubscribeResult::Err(SubscribeError::InvalidName));
        }
        for player in public_players.clone() {
            if player.name.eq(&name) {
                return Message::SubscribeResult(SubscribeResult::Err(SubscribeError::AlreadyRegistered));
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

    unsafe fn update_player_in_player_list(success: bool, addr: String, used_time: f64) -> Option<PublicPlayer> {
        let current_player = PUBLIC_PLAYERS.iter().find(|player| player.stream_id == addr)?;
        let mut current_player = current_player.clone();
        update_player_score(&mut current_player, success);
        increment_player_steps(&mut current_player);
        replace_player_in_players(&current_player);
        current_player.total_used_time += used_time;
        Some(current_player)
    }

    fn get_next_player(challenge_result: ChallengeResult, public_players: Vec<PublicPlayer>) -> Option<PublicPlayer> {
        if challenge_result.next_target == "" {
            return Some(get_random_next_player(public_players))
        }
        let next_player_result = public_players.iter().find(|player| player.name == challenge_result.next_target)?;
        Some(next_player_result.clone())
    }

    fn get_random_next_player(public_players: Vec<PublicPlayer>) -> PublicPlayer {
        let mut rng = rand::thread_rng();
        let active_players: Vec<&PublicPlayer> = public_players.iter().filter(|player| player.is_active).collect();
        let random_index: usize = rng.gen_range(0..active_players.len());
        return active_players[random_index].clone();
    }

    unsafe fn send_to_all_players(message: Message) {
        PUBLIC_PLAYERS_TCP_STREAM.iter_mut().for_each(|public_player_tcp_stream| {
            shared::write_message(&public_player_tcp_stream.stream, message.clone()).expect("Failed to send PublicLeaderBoard message to clients");
        });
    }

    unsafe fn send_round_summarize(challenge_name: String, reported_challenges: Vec<ReportedChallengeResult>) {
        send_to_all_players(Message::RoundSummary(RoundSummary {
            challenge: challenge_name,
            chain: reported_challenges.clone()
        }));
    }
}
