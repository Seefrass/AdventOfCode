use std::{
    cmp,
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

    //let result = input.split(",").map(to_range);
    let result: u64 = input
        .split(",")
        .map(|s| s.split_once("-").unwrap())
        .map(|(s, e)| generate_invalid_ids(s.parse::<u64>().unwrap(), e.parse::<u64>().unwrap()))
        .flatten()
        .sum();
    /*
    let result1: u64 = result
        .clone()
        .map(|i| find_invalid_ids(i, is_invalid_id))
        .flatten()
        .sum();

    let result2: u64 = result
        .map(|i| find_invalid_ids(i, is_invalid_id_part_two))
        .flatten()
        .sum();
    */

    println!("Sum of invalid IDs: {}", result);
    //println!("Sum of invalid IDs with new rules: {}", result2);
}

fn generate_invalid_ids(range_start: u64, range_end: u64) -> Vec<u64> {
    let mut result = Vec::new();

    for length in range_start.to_string().len()..=range_end.to_string().len() {
        if length % 2 == 1 {
            continue;
        }

        // 1111
        let start_num = cmp::max(range_start, min(length));
        let end_num = cmp::min(range_end, max(length));

        // "1111"
        let start_str = start_num.to_string();
        let end_str = end_num.to_string();

        // "11"
        let start_half_num = start_str.split_at(length / 2).0.parse::<u64>().unwrap();
        let end_half_num = end_str.split_at(length / 2).0.parse::<u64>().unwrap();

        for i in start_half_num..=end_half_num {
            let invalid_id = i * 10u64.pow(length as u32 / 2) + i;
            if invalid_id < range_start || invalid_id > range_end {
                continue;
            }
            result.push(invalid_id);
        }
    }

    result
}

fn min(length: usize) -> u64 {
    10u64.pow((length - 1) as u32)
}

fn max(length: usize) -> u64 {
    let mut result = 9;
    for _ in 0..length - 1 {
        result = result * 10 + 9;
    }
    result
}

/*
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

    if !is_feasibly_invalid(&id_str) {
        return false;
    }

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

fn is_feasibly_invalid(id_str: &String) -> bool {
    let mut buckets = vec![0; 10];
    id_str.chars().for_each(|c| {
        let idx = c.to_digit(10).unwrap();
        buckets[idx as usize] += 1;
    });
    !buckets.iter().any(|&d| d == 1)
}
*/
