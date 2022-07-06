use md5;
use crate::challenges::Challenge;
use crate::{challenges, MD5HashCashInput, MD5HashCashOutput};

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
            match generate_seed(&generated_seeds) {
                Some(seed) => {
                    generated_seeds.push(seed);
                    // println!("seed: {}", seed);

                    let seed_as_hexadecimal_string = complete_hexadecimal_seed_with_zero(format!("{:x}", seed));
                    // println!("New seed as hexadecimal string: {}", seed_as_hexadecimal_string);

                    let concatenated = format!("{}{}", seed_as_hexadecimal_string, self.input.message.as_str());

                    let hashcode = md5::compute(concatenated);

                    let hashcode_string = format!("{:x}", hashcode).to_uppercase();
                    // println!("Digest string: {}", hashcode_string);

                    let hashcode_binary = challenges::convert_string_to_binary(hashcode_string.clone());
                    // println!("Digest bits: {:?}", digest_binary);

                    if challenges::check_number_of_zero(hashcode_binary, self.input.complexity) {
                        final_output.seed = seed;
                        final_output.hashcode = hashcode_string.to_string();
                        solved = true;
                    }
                }
                None => {
                    panic!("Error while generating seed");
                }
            }
            // println!("================================================================")
        }
        return final_output;
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        let hash_cash_client_answer_in_binary = challenges::convert_string_to_binary(answer.hashcode.clone());
        return challenges::check_number_of_zero(hash_cash_client_answer_in_binary, self.input.complexity.clone());
    }
}

pub fn generate_seed(already_generated_seed: &Vec<u64>) -> Option<u64> {
    if already_generated_seed.len() == 0 {
        return Some(1);
    }

    let last_generated_seed = already_generated_seed.last()?;
    let generated_seed = last_generated_seed + 1;
    return if already_generated_seed.contains(&generated_seed) {
        generate_seed(already_generated_seed)
    } else {
        Some(generated_seed)
    }
}

pub fn complete_hexadecimal_seed_with_zero(input: String) -> String {
    if input.len() < 16 {
        return complete_hexadecimal_seed_with_zero(format!("0{}", input).to_string());
    }
    return input.to_uppercase();
}
