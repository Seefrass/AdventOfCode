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

    let result: u64 = input
        .split(",")
        .map(to_range)
        .map(find_invalid_ids)
        .flatten()
        .sum();

    println!("Sum of invalid IDs: {}", result)
}

fn to_range(range_str: &str) -> RangeInclusive<u64> {
    let (start, end) = range_str.split_once("-").unwrap();
    let (start, end) = (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap());
    start..=end
}

fn find_invalid_ids(range: RangeInclusive<u64>) -> Vec<u64> {
    let mut result = Vec::new();

    for id in range {
        let id_str = id.to_string();
        if id_str.len() % 2 == 1 {
            continue;
        }
        let (id_str1, id_str2) = id_str.split_at(id_str.len() / 2);
        if id_str1 == id_str2 {
            result.push(id);
        }
    }

    result
}
