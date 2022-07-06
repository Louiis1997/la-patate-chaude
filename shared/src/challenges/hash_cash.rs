pub fn generate_seed(already_generated_seed: &Vec<u64>) -> u64 {
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

pub fn complete_hexadecimal_seed_with_zero(input: String) -> String {
    if input.len() < 16 {
        return complete_hexadecimal_seed_with_zero(format!("0{}", input).to_string());
    }
    return input.to_uppercase();
}
