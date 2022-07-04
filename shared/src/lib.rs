use std::io::{Read, Write};
use std::net::TcpStream;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Welcome {
    pub version: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe {
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicLeaderBoard (
    pub Vec<PublicPlayer>
);

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: i32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub enum  Challenge {
    MD5HashCash(MD5HashCashInput),
    MonstrousMaze(MonstrousMazeInput),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashInput {
    pub complexity: u32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonstrousMazeInput {
    pub grid: String,
    pub endurance: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput),
    MonstrousMaze(MonstrousMazeOutput),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashOutput {
    pub seed: u64,
    pub hashcode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonstrousMazeOutput {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundSummary {
    pub challenge: String,
    pub chain: Vec<ReportedChallengeResult>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedChallengeResult {
    pub name: String,
    pub value: ChallengeValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult(BadResult),
    Ok(Ok),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BadResult {
    pub used_time: f64,
    pub next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ok {
    pub used_time: f64,
    pub next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndOfGame {
    pub leader_board: PublicLeaderBoard,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    StartGame,
    PublicLeaderBoard(PublicLeaderBoard),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),
}

fn serialize_message(message : Message) -> String {
    let serialized = serde_json::to_string(&message);
    return serialized.unwrap();
}

pub fn write_message(stream: &mut TcpStream, message : Message){
    let serialized = serialize_message(message);
    let size = serialized.len() as u32;
    let size = size.to_be_bytes();
    stream.write_all(&size).unwrap();
    stream.write_all(&serialized.as_bytes()).unwrap();
}

pub fn read_message(stream: &mut TcpStream) -> Vec<u8> {
    let mut data = [0 as u8; 4];
    match stream.read_exact(&mut data) {
        Ok(_) => {
            let size = u32::from_be_bytes(data) as usize;
            let mut data : Vec<u8> = vec![0u8; size];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    return data;
                },
                Err(e) => {
                    panic!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            panic!("Failed to receive data: {}", e);
        }
    }
}
