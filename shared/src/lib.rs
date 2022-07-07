pub mod challenges;

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Welcome {
    pub version: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subscribe {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublicLeaderBoard(pub Vec<PublicPlayer>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: i32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Challenge {
    MD5HashCash(MD5HashCashInput),
    MonstrousMaze(MonstrousMazeInput),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MD5HashCashInput {
    pub complexity: u32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonstrousMazeInput {
    pub grid: String,
    pub endurance: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput),
    MonstrousMaze(MonstrousMazeOutput),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MD5HashCashOutput {
    pub seed: u64,
    pub hashcode: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonstrousMazeOutput {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoundSummary {
    pub challenge: String,
    pub chain: Vec<ReportedChallengeResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReportedChallengeResult {
    pub name: String,
    pub value: ChallengeValue,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult(BadResult),
    Ok(Ok),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BadResult {
    pub used_time: f64,
    pub next_target: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ok {
    pub used_time: f64,
    pub next_target: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EndOfGame {
    pub leader_board: PublicLeaderBoard,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

pub fn send_message(stream: &TcpStream, message: Message) {
    match write_message(stream, message) {
        Ok(_) => {}
        Err(err) => {
            panic!("Enable to send message to the server: {}", err)
        }
    }
}

pub fn write_message(mut stream: &TcpStream, message: Message) -> std::io::Result<()> {
    match serialize_message(message) {
        Ok(serialized) => {
            let size = serialized.len() as u32;
            let size = size.to_be_bytes();
            stream.write_all(&size)?;
            stream.write_all(serialized.as_bytes())?;
            Ok(())
        }
        Err(err) => panic!("Serialization failed: {}", err),
    }
}

fn serialize_message(message: Message) -> serde_json::Result<String> {
    let serialized = serde_json::to_string(&message)?;
    Ok(serialized)
}

pub fn read_message(mut stream: &TcpStream) -> String {
    let mut data = [0_u8; 4];
    match stream.read_exact(&mut data) {
        Ok(_) => read_message_data(stream, data),
        Err(e) => {
            panic!("Failed to read message size: {}", e);
        }
    }
}

fn read_message_data(mut stream: &TcpStream, data: [u8; 4]) -> String {
    let size = u32::from_be_bytes(data) as usize;
    let mut data: Vec<u8> = vec![0u8; size];
    match stream.read_exact(&mut data) {
        Ok(_) => {
            vec_to_string(data)
        }
        Err(e) => {
            panic!("Failed to read message data: {}", e);
        }
    }
}

fn vec_to_string(data: Vec<u8>) -> String {
    match from_utf8(&data) {
        Ok(response) => {
            println!("{}", response);
            response.to_string()
        }
        Err(e) => {
            panic!("Failed to convert message data to string: {}", e);
        }
    }
}
