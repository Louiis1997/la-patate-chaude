pub mod hash_cash;
pub mod monstrous_maze;

use md5;
use crate::{MD5HashCashInput, MD5HashCashOutput, MonstrousMazeInput, MonstrousMazeOutput};
use crate::challenges::hash_cash::{complete_hexadecimal_seed_with_zero, generate_seed};
use crate::challenges::monstrous_maze::{find_paths, get_best_path, Grid, GridPossibleSolution};

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
        let mut solved = false;
        let mut generated_seeds: Vec<u64> = Vec::new();
        let mut final_output: MD5HashCashOutput = MD5HashCashOutput {
            hashcode: "".to_string(),
            seed: 0,
        };

        while !solved {
            let seed = generate_seed(&generated_seeds);
            generated_seeds.push(seed);
            // println!("seed: {}", seed);

            let seed_as_hexadecimal_string = complete_hexadecimal_seed_with_zero(format!("{:x}", seed));
            // println!("New seed as hexadecimal string: {}", seed_as_hexadecimal_string);

            let concatenated = format!("{}{}", seed_as_hexadecimal_string, self.input.message.as_str());

            let hashcode = md5::compute(concatenated);

            let hashcode_string = format!("{:x}", hashcode).to_uppercase();
            // println!("Digest string: {}", hashcode_string);

            let hashcode_binary = convert_string_to_binary(hashcode_string.clone());
            // println!("Digest bits: {:?}", digest_binary);

            if check_number_of_zero(hashcode_binary, self.input.complexity) {
                final_output.seed = seed;
                final_output.hashcode = hashcode_string.to_string();
                solved = true;
            }
            // println!("================================================================")
        }
        return final_output;
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
        let mut final_output = MonstrousMazeOutput {
            path: "".to_string(),
        };

        let mut grid: Grid = Grid::new(self.input.clone());
        println!("Grid start: {:?}", grid.start);
        println!("Grid end: {:?}", grid.end);

        let grid_possible_solution: GridPossibleSolution = GridPossibleSolution {
            path_taken: "".to_string(),
            current_coordinates: (grid.start.0 as i64, grid.start.1 as i64),
            visited_coordinates: vec![],
            success: false,
            endurance_left: grid.endurance as i8,
        };

        let possible_solutions = find_paths(&mut grid, grid_possible_solution);

        let no_solution_because_died = possible_solutions.iter().all(|solution| solution.endurance_left <= 0);
        if no_solution_because_died {
            println!("/!\\ No solution found because '☠️ YOU DIED ☠️' /!\\");
            return final_output;
        }

        // Filter successful & not empty paths
        let successful_paths: Vec<&GridPossibleSolution> = possible_solutions
            .iter()
            .filter(|path| path.success && !path.path_taken.is_empty())
            .collect::<Vec<&GridPossibleSolution>>();

        if successful_paths.len() == 0 {
            println!("/!\\ No solution because no path found in Monstrous Maze ☹️ /!\\");
            return final_output;
        }

        // Display found paths
        // println!("Found paths:");
        // for path in &successful_paths {
        //     println!("path {:?} - endurance: {} - success: {}", &path.path_taken, &path.endurance_left, &path.success);
        // }

        let best_path: &GridPossibleSolution = get_best_path(successful_paths);
        final_output.path = best_path.path_taken.clone();

        return final_output;
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
