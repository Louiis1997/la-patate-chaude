mod hash_cash;

use std::io;
use std::str::from_utf8;
use shared::{ChallengeAnswer, ChallengeResult, MD5HashCashInput, MD5HashCashOutput, Message, Subscribe, SubscribeResult};
use shared::Challenge::{MD5HashCash, MonstrousMaze};

fn main() {
    let stream = std::net::TcpStream::connect("localhost:7878");
    match stream {
        Ok(mut stream) => {
            shared::write_message(&mut stream, Message::Hello);
            let response = shared::read_message(& mut stream);
            println!("{}", from_utf8(&response).unwrap());
            //let mut buffer = String::new();
            //io::stdin().read_line(&mut buffer).unwrap();
            //shared::write_message(&mut stream, Message::Subscribe(Subscribe { name: buffer.trim_end().to_string() }));
            shared::write_message(&mut stream, Message::Subscribe(Subscribe { name: "free_patatoo".to_string() }));
            let response = shared::read_message(& mut stream);
            println!("{}", from_utf8(&response).unwrap());
            loop {
                let response = shared::read_message(&mut stream);
                let response= from_utf8(&response).unwrap();
                println!("{}", response);
                let response = serde_json::from_str(response).unwrap();
                match response {
                    Message::EndOfGame(..) => {
                        break;
                    }
                    Message::Challenge(response) => {
                        match response {
                            MD5HashCash(md5_hash_cash_input) => {
                                shared::write_message(&mut stream, Message::ChallengeResult(
                                    ChallengeResult {
                                        answer: ChallengeAnswer::MD5HashCash {
                                            0: hash_cash::solve_md5(md5_hash_cash_input),
                                        },
                                        next_target: "".to_string()
                                    }
                                ));
                                let response = shared::read_message(& mut stream);
                                println!("{}", from_utf8(&response).unwrap());
                            },
                            MonstrousMaze(MonstrousMazeInput) => {
                                shared::write_message(&mut stream, Message::ChallengeResult(
                                    ChallengeResult {
                                        answer: ChallengeAnswer::MonstrousMaze {
                                            0: monstrous_maze::solve_monstrous_maze(MonstrousMazeInput),
                                        },
                                        next_target: "".to_string()
                                    }
                                ))
                            }
                        }
                    }
                    Message::SubscribeResult(res) => {
                        match res {
                            SubscribeResult::Ok => {}
                            SubscribeResult::Err(..) => {
                                break;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Err(err) => panic!("Cannot connect: {err}")
    }
}
