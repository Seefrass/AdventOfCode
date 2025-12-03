use std::{
    fs::File,
    io::{BufReader, Read},
};

const FILE_NAME: &str = "input.txt";

fn main() {
    let file = File::open(FILE_NAME).unwrap();
    let mut reader = BufReader::new(file);
    let mut input = String::new();

    reader
        .read_to_string(&mut input)
        .expect("could not read from file");

    let input = input.trim();

    //--- Actual Task starts here ---//

    let result = input.split("\n").map(parse_banks);

    let result1: u64 = result.clone().map(|b| find_max_joltage(b, 2)).sum();
    let result2: u64 = result.map(|b| find_max_joltage(b, 12)).sum();

    println!(
        "Sum of max joltages when turning on exactly two batteries: {}",
        result1
    );
    println!(
        "Sum of max joltages when turning on exactly twelve batteries: {}",
        result2
    );
}

fn parse_banks(bank_str: &str) -> Vec<u8> {
    bank_str
        .chars()
        .map(|c| char::to_digit(c, 10).unwrap() as u8)
        .collect::<Vec<u8>>()
}

fn find_max_joltage(bank: Vec<u8>, battery_count: u32) -> u64 {
    let mut result = 0;

    let mut idx = 0;
    for i in (1..=battery_count).rev() {
        let (digit, new_idx) = highest_digit(&bank[idx..bank.len() - (i - 1) as usize]);
        idx += new_idx + 1; // note: new_idx is only the local index from within the slice thus "+="

        result = result * 10 + digit as u64
    }

    result
}

fn highest_digit(digits: &[u8]) -> (u8, usize) {
    let mut result = (0, 0);
    let mut max = 0;
    for i in 0..digits.len() {
        if digits[i] > max {
            result = (digits[i], i);
            max = digits[i]
        }
    }
    result
}
