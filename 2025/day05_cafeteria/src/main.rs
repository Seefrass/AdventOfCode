use std::{
    cmp,
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
    let id_ranges = remove_overlaps(id_ranges);

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

// Note:
// The idea of this solution is not my own! It actually comes from a friend of mine, so all credit
// goes to him: https://github.com/xA1ph/AdventOfCode2025/blob/main/src/day05/Aufgabe_5.java
// I chose to implement this idea as an excercise, because 1) it is more intuitive and 2) is is
// also much faster than my original solution.

fn remove_overlaps(ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut ranges = ranges.clone();
    let mut result: Vec<RangeInclusive<u64>> = Vec::new();

    ranges.sort_by(|r1, r2| r1.start().cmp(r2.start()));
    let tmp = ranges
        .into_iter()
        .reduce(|r1: RangeInclusive<u64>, r2: RangeInclusive<u64>| {
            if r1.end() >= r2.start() {
                RangeInclusive::new(*r1.start(), cmp::max(*r1.end(), *r2.end()))
            } else {
                result.push(r1);
                r2
            }
        })
        .unwrap();
    result.push(tmp); // don't forget the remaining range

    result
}
