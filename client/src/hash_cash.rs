use md5;
use shared::{MD5HashCashInput, MD5HashCashOutput};

pub fn solve_md5(input: MD5HashCashInput) -> MD5HashCashOutput {
    let mut solved = false;
    let mut generated_seeds: Vec<u64> = Vec::new();
    let mut final_output: MD5HashCashOutput = MD5HashCashOutput {
        hashcode: "".to_string(),
        seed: 0,
    };

    while !solved {
        let seed = generate_seed(&generated_seeds);// TODO generate seed
        generated_seeds.push(seed);
        // println!("seed: {}", seed);

        let seed_as_hexadecimal_string = complete_hexadecimal_seed_with_zero(format!("{:x}", seed));
        // println!("New seed as hexadecimal string: {}", seed_as_hexadecimal_string);

        let concatenated = format!("{}{}", seed_as_hexadecimal_string, input.message.as_str());

        let hashcode = md5::compute(concatenated);

        let hashcode_string = format!("{:x}", hashcode).to_uppercase();
        // println!("Digest string: {}", hashcode_string);

        let hashcode_binary = convert_string_to_binary(hashcode_string.clone());
        // println!("Digest bits: {:?}", digest_binary);

        if check_number_of_zero(hashcode_binary, input.complexity) {
            final_output.seed = seed;
            final_output.hashcode = hashcode_string.to_string();
            solved = true;
        }
        // println!("================================================================")
    }
    return final_output;
}

fn generate_seed(already_generated_seed: &Vec<u64>) -> u64 {
    if already_generated_seed.len() == 0 {
        return 1;
    }

    let last_generated_seed = already_generated_seed.last().unwrap();
    let generated_seed = last_generated_seed + 1;
    if already_generated_seed.contains(&generated_seed) {
        generate_seed(already_generated_seed)
    } else {
        return generated_seed;
    }
}

fn complete_hexadecimal_seed_with_zero(input: String) -> String {
    if input.len() < 16 {
        return complete_hexadecimal_seed_with_zero(format!("0{}", input).to_string());
    }
    return input.to_uppercase();
}

fn convert_string_to_binary(input: String) -> String {
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

fn check_number_of_zero(input: String, complexity: u32) -> bool {
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
