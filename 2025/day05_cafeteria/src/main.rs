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

    let (id_ranges, ids) = input.split_once("\n\n").unwrap();

    let id_ranges: Vec<RangeInclusive<u64>> = id_ranges.split("\n").map(to_range).collect();
    let id_ranges = optimize_ranges(id_ranges);

    let ids = ids.split("\n").map(|s| s.parse::<u64>().unwrap());

    let result1: usize = ids.filter(|id| is_fresh(id, &id_ranges)).count();
    let result2: usize = id_ranges.iter().map(|r| r.clone().count()).sum();

    println!("A total of {} ingredients are still fresh", result1);
    println!("The total number of unique fresh ids is {}", result2);
}

fn to_range(s: &str) -> RangeInclusive<u64> {
    let (start, end) = s.split_once("-").unwrap();
    start.parse().unwrap()..=end.parse().unwrap()
}

fn is_fresh(id: &u64, id_ranges: &Vec<RangeInclusive<u64>>) -> bool {
    for range in id_ranges {
        if range.contains(id) {
            return true;
        }
    }
    false
}

fn optimize_ranges(ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut ranges = ranges.clone();
    let mut result = Vec::new();

    while !ranges.is_empty() {
        let mut current = ranges.remove(0);

        loop {
            let (mut overlaps, remainder) = remove_overlapping_with(&current, ranges);
            ranges = remainder;

            if overlaps.len() > 0 {
                overlaps.push(current);
                current = merge_ranges(overlaps);
            } else {
                break;
            }
        }

        result.push(current);
    }

    result
}

fn remove_overlapping_with(
    range: &RangeInclusive<u64>,
    others: Vec<RangeInclusive<u64>>,
) -> (Vec<RangeInclusive<u64>>, Vec<RangeInclusive<u64>>) {
    let mut remove = Vec::new();
    let mut keep = Vec::new();

    for r in others {
        if range.start() <= r.end() && r.start() <= range.end() {
            remove.push(r);
        } else {
            keep.push(r);
        }
    }

    (remove, keep)
}

fn merge_ranges(ranges: Vec<RangeInclusive<u64>>) -> RangeInclusive<u64> {
    let bounds = ranges.iter().map(|r| [*r.start(), *r.end()]).flatten();
    bounds.clone().min().unwrap()..=bounds.max().unwrap()
}
