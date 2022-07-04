use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;
use shared::{MD5HashCashInput, Message, PublicLeaderBoard, PublicPlayer, SubscribeError, SubscribeResult, Welcome};
use shared::Challenge::MD5HashCash;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7676");
    let listener = match listener {
        Ok(l) => l,
        Err(err) => panic!("Cannot bind: {err}")
    };
    static mut PUBLIC_PLAYER: Vec<PublicPlayer> = Vec::new();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("{}", stream.peer_addr().unwrap());
        thread::spawn(move|| {
            handle_client(&mut stream);
        });
    }

    fn handle_client(mut stream :  &mut TcpStream) {
        loop {
            let addr = stream.peer_addr().unwrap().to_string();
            let response = shared::read_message(&mut stream);
            let response= from_utf8(&response).unwrap();
            println!("{}", response);
            let response = serde_json::from_str(response).unwrap();
            match response {
                Message::Hello => {
                    shared::write_message(&mut stream, Message::Welcome(Welcome { version: 1 }));
                }
                Message::Subscribe(subscribe) => unsafe {
                    shared::write_message(&mut stream, create_player(subscribe.name, &mut PUBLIC_PLAYER, addr));
                }
                Message::StartGame => unsafe {
                    shared::write_message(&mut stream, Message::PublicLeaderBoard(PublicLeaderBoard(PUBLIC_PLAYER)));
                    shared::write_message(&mut stream, Message::Challenge(
                        MD5HashCash(
                            MD5HashCashInput {
                                complexity: 9,
                                message: "Hello".to_string()
                            }
                        )
                    ));
                }
                _ => {}
            }
        }
    }


    fn create_player(name: String, public_player: &mut Vec<PublicPlayer>, stream: String) -> Message {
        if !name.is_ascii() {
            return Message::SubscribeResult(SubscribeResult::Err(SubscribeError::InvalidName))
        }
        if public_player.len() == 0 {
            public_player.push(
                PublicPlayer {
                    name,
                    stream_id: stream,
                    score: 0,
                    steps: 0,
                    is_active: true,
                    total_used_time: 0.0
                }
            );
            return Message::SubscribeResult(SubscribeResult::Ok);
        }
        for player in public_player {
            if player.name.eq(&name) {
                return Message::SubscribeResult(SubscribeResult::Err(SubscribeError::AlreadyRegistered))
            }
        }
        return Message::SubscribeResult(SubscribeResult::Ok)
    }
}
