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

    let result1: u64 = input
        .split(",")
        .map(|s| s.split_once("-").unwrap())
        .map(|(s, e)| {
            generate_invalid_ids(s.parse::<u64>().unwrap(), e.parse::<u64>().unwrap(), false)
        })
        .flatten()
        .sum();

    let result2: u64 = input
        .split(",")
        .map(|s| s.split_once("-").unwrap())
        .map(|(s, e)| {
            generate_invalid_ids(s.parse::<u64>().unwrap(), e.parse::<u64>().unwrap(), true)
        })
        .flatten()
        .sum();

    println!("Sum of invalid IDs: {}", result1);
    println!("Sum of invalid IDs with new rules: {}", result2);
}

fn generate_invalid_ids(range_start: u64, range_end: u64, part_two: bool) -> Vec<u64> {
    let mut result = Vec::new();

    for length in range_start.to_string().len()..=range_end.to_string().len() {
        let divisors = if part_two {
            get_unique_divisors(length)
        } else {
            if length % 2 == 1 { vec![] } else { vec![2] }
        };

        for d in divisors {
            // 1111
            let start_num = cmp::max(range_start, min(length));
            let end_num = cmp::min(range_end, max(length));

            // "1111"
            let start_str = start_num.to_string();
            let end_str = end_num.to_string();

            // 11
            let start_frac_num = start_str.split_at(length / d).0.parse::<u64>().unwrap();
            let end_frac_num = end_str.split_at(length / d).0.parse::<u64>().unwrap();

            for i in start_frac_num..=end_frac_num {
                let invalid_id = i.to_string().repeat(d).parse::<u64>().unwrap();
                if invalid_id < range_start || invalid_id > range_end {
                    continue;
                }
                result.push(invalid_id);
            }
        }
    }

    result.sort();
    result.dedup();

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

fn get_unique_divisors(n: usize) -> Vec<usize> {
    let mut result = Vec::new();

    for i in 2..=n {
        if n % i == 0 {
            result.push(i);
        }
    }

    result
}
