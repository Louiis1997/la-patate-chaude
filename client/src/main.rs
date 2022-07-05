mod hash_cash;
mod monstrous_maze;

use clap::Parser;
use shared::{ChallengeAnswer, ChallengeResult, Message, Subscribe, SubscribeResult};
use shared::Challenge::{MD5HashCash, MonstrousMaze};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'n', long, required = false, default_value = "free_potato", value_parser)]
    name: String,
    #[clap(short = 'p', long, required = false, default_value = "localhost:7878",value_parser)]
    port: String,
}

fn main() {
    let args = Args::parse();
    let stream= std::net::TcpStream::connect(args.port);
    match stream {
        Ok(mut stream) => {
            shared::send_message(&mut stream, Message::Hello);
            shared::read_message(&mut stream);
            shared::send_message(&mut stream, Message::Subscribe(Subscribe { name: args.name }));
            loop {
                let response = shared::read_message(&mut stream);
                match serde_json::from_str(&response) {
                    Ok(message) => {
                        match message {
                            Message::SubscribeResult(res) => {
                                match res {
                                    SubscribeResult::Ok => {}
                                    SubscribeResult::Err(..) => {
                                        panic!("Please restart the client with a new name :)")
                                    }
                                }
                            }
                            Message::Challenge(response) => {
                                match response {
                                    MD5HashCash(md5_hash_cash_input) => {
                                        shared::send_message(&mut stream, Message::ChallengeResult(
                                            ChallengeResult {
                                                answer: ChallengeAnswer::MD5HashCash {
                                                    0: hash_cash::solve_md5(md5_hash_cash_input),
                                                },
                                                next_target: "".to_string()
                                            }
                                        ));
                                    },
                                    MonstrousMaze(monstrous_maze_input) => {
                                        shared::send_message(&mut stream, Message::ChallengeResult(
                                            ChallengeResult {
                                                answer: ChallengeAnswer::MonstrousMaze {
                                                    0: monstrous_maze::solve_monstrous_maze(monstrous_maze_input),
                                                },
                                                next_target: "".to_string()
                                            }
                                        ));
                                    }
                                };
                            }
                            Message::EndOfGame(..) => {
                                break;
                            }
                            _ => {}
                        }
                    }
                    Err(err) => {
                        panic!("Failed to deserialize the message received: {}", err)
                    }
                }
            }
        }
        Err(err) => panic!("Cannot connect: {err}")
    }
}



