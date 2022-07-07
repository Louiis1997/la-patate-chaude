use clap::Parser;
use rand::Rng;
use shared::challenges::hash_cash::MD5HashCash as MD5HashCashChallenge;
use shared::challenges::monstrous_maze::MonstrousMaze as MonstrousMazeChallenge;
use shared::challenges::Challenge;
use shared::Challenge::{MD5HashCash, MonstrousMaze};
use shared::{ChallengeAnswer, ChallengeResult, Message, PublicPlayer, Subscribe, SubscribeResult};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(
        short = 'n',
        long,
        required = false,
        default_value = "free_potato",
        value_parser
    )]
    name: String,
    #[clap(
        short = 'a',
        long,
        required = false,
        default_value = "localhost:7878",
        value_parser
    )]
    addr: String,
}

fn main() {
    let args = Args::parse();
    let stream = std::net::TcpStream::connect(args.addr);
    match stream {
        Ok(mut stream) => {
            shared::send_message(&mut stream, Message::Hello);
            shared::read_message(&mut stream);
            shared::send_message(
                &mut stream,
                Message::Subscribe(Subscribe { name: args.name }),
            );
            let mut public_leader_board = Vec::new();
            loop {
                let response = shared::read_message(&mut stream);
                match serde_json::from_str(&response) {
                    Ok(message) => match message {
                        Message::SubscribeResult(res) => match res {
                            SubscribeResult::Ok => {}
                            SubscribeResult::Err(..) => {
                                panic!("Please restart the client with a new name :)")
                            }
                        },
                        Message::PublicLeaderBoard(leader_board) => {
                            public_leader_board = leader_board.0
                        }
                        Message::Challenge(response) => {
                            match response {
                                MD5HashCash(md5_hash_cash_input) => {
                                    let challenge = MD5HashCashChallenge::new(md5_hash_cash_input);
                                    shared::send_message(
                                        &mut stream,
                                        Message::ChallengeResult(ChallengeResult {
                                            answer: ChallengeAnswer::MD5HashCash(MD5HashCashChallenge::solve(&challenge)),
                                            next_target: next_target(&public_leader_board),
                                        }),
                                    );
                                }
                                MonstrousMaze(monstrous_maze_input) => {
                                    let challenge =
                                        MonstrousMazeChallenge::new(monstrous_maze_input);
                                    shared::send_message(
                                        &mut stream,
                                        Message::ChallengeResult(ChallengeResult {
                                            answer: ChallengeAnswer::MonstrousMaze(MonstrousMazeChallenge::solve(&challenge)),
                                            next_target: next_target(&public_leader_board),
                                        }),
                                    );
                                }
                            };
                        }
                        Message::EndOfGame(..) => {
                            break;
                        }
                        _ => {}
                    },
                    Err(err) => {
                        panic!("Failed to deserialize the message received: {}", err)
                    }
                }
            }
        }
        Err(_err) => panic!("Cannot connect: {_err}"),
    }
}

fn next_target(public_leader_board: &Vec<PublicPlayer>) -> String {
    let mut rng = rand::thread_rng();
    public_leader_board[rng.gen_range(0..public_leader_board.len())]
        .name
        .to_string()
}
