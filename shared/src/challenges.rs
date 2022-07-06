use crate::{MD5HashCashInput, MD5HashCashOutput, MonstrousMazeInput, MonstrousMazeOutput};

pub enum Challenges {
    MD5HashCash(MD5HashCash),
    MonstrousMaze(MonstrousMaze),
}

pub trait Challenge {
    /// Données en entrée du challenge
    type Input;
    /// Données en sortie du challenge
    type Output;
    /// Nom du challenge
    fn name() -> String;
    /// Create a challenge from the specific input
    fn new(input: Self::Input) -> Self;
    /// Résout le challenge
    fn solve(&self) -> Self::Output;
    /// Vérifie qu'une sortie est valide pour le challenge
    fn verify(&self, answer: &Self::Output) -> bool;
}

#[derive(Debug, Clone)]
pub struct MD5HashCash {
    pub input: MD5HashCashInput,
}

impl Challenge for MD5HashCash {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        return "MD5HashCash".to_string();
    }

    fn new(input: Self::Input) -> Self {
        return Self {input};
    }

    fn solve(&self) -> Self::Output {
        todo!()
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        let hash_cash_client_answer_in_binary = convert_string_to_binary(answer.hashcode.clone());
        return check_number_of_zero(hash_cash_client_answer_in_binary, self.input.complexity.clone());
    }
}

#[derive(Debug, Clone)]
pub struct MonstrousMaze {
    pub input: MonstrousMazeInput,
}

impl Challenge for MonstrousMaze {
    type Input = MonstrousMazeInput;
    type Output = MonstrousMazeOutput;

    fn name() -> String {
        return "MonstrousMaze".to_string();
    }

    fn new(input: Self::Input) -> Self {
        return Self { input};
    }

    fn solve(&self) -> Self::Output {
        todo!()
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        println!("{:?}", answer);
        return true;
    }
}

pub fn convert_string_to_binary(input: String) -> String {
    let mut name_in_binary = "".to_string();
    // Call into_bytes() which returns a Vec<u8>, and iterate accordingly
    // I only called clone() because this for loop takes ownership
    for character in input.clone().chars() {
        name_in_binary += to_binary(character);
    }
    return name_in_binary;
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("Invalid character in hexadecimal string"),
    }
}

pub fn check_number_of_zero(input: String, complexity: u32) -> bool {
    let mut number_of_zero = 0;
    for character in input.chars() {
        if character == '0' {
            number_of_zero += 1;
        } else {
            return number_of_zero == complexity;
        }
    }
    return false;
}
