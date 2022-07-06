pub mod hash_cash;
pub mod monstrous_maze;
use hash_cash::MD5HashCash;
use monstrous_maze::MonstrousMaze;

#[derive(Debug, Clone)]
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
