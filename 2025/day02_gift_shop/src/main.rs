use std::{
    fs::File,
    io::{BufReader, Read},
    ops::RangeInclusive,
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

    let result = input.split(",").map(to_range);

    let result1: u64 = result
        .clone()
        .map(|i| find_invalid_ids(i, is_invalid_id))
        .flatten()
        .sum();

    let result2: u64 = result
        .map(|i| find_invalid_ids(i, is_invalid_id_part_two))
        .flatten()
        .sum();

    println!("Sum of invalid IDs: {}", result1);
    println!("Sum of invalid IDs with new rules: {}", result2);
}

fn to_range(range_str: &str) -> RangeInclusive<u64> {
    let (start, end) = range_str.split_once("-").unwrap();
    let (start, end) = (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap());
    start..=end
}

fn find_invalid_ids<F: Fn(u64) -> bool>(range: RangeInclusive<u64>, is_invalid_id: F) -> Vec<u64> {
    let mut result = Vec::new();

    for id in range {
        if is_invalid_id(id) {
            result.push(id);
        }
    }

    result
}

fn is_invalid_id(id: u64) -> bool {
    let id_str = id.to_string();
    if id_str.len() % 2 == 1 {
        return false;
    }
    let (id_str1, id_str2) = id_str.split_at(id_str.len() / 2);
    if id_str1 == id_str2 {
        return true;
    }
    return false;
}

fn is_invalid_id_part_two(id: u64) -> bool {
    let id_str = id.to_string();
    let divisors = get_unique_divisors(id_str.len());
    for d in divisors {
        let chunks = id_str
            .as_bytes()
            .chunks(id_str.len() / d)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();
        let first = chunks.first().unwrap();
        if chunks.iter().all(|c| c == first) {
            return true;
        }
    }
    false
}

fn get_unique_divisors(n: usize) -> Vec<usize> {
    let mut result = Vec::new();

    for i in 2..=n {
        if n % i == 0 {
            result.push(i);
        }
    }

    result
}
