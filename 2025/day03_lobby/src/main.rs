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

    let result: u32 = input
        .split("\n")
        .map(parse_banks)
        .map(find_max_joltage)
        .sum();

    println!("Sum of max joltages: {}", result);
}

fn parse_banks(bank_str: &str) -> Vec<u8> {
    bank_str
        .chars()
        .map(|c| char::to_digit(c, 10).unwrap() as u8)
        .collect::<Vec<u8>>()
}

fn find_max_joltage(bank: Vec<u8>) -> u32 {
    let (first_digit, idx) = highest_digit(&bank[0..bank.len() - 1]);
    let (second_digit, _) = highest_digit(&bank[idx + 1..bank.len()]);
    first_digit as u32 * 10 + second_digit as u32
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
