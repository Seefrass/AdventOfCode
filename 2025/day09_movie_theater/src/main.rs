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

    let corners: Vec<(u64, u64)> = input
        .split("\n")
        .map(|l| l.split_once(",").unwrap())
        .map(|(x_str, y_str)| (x_str.parse::<u64>().unwrap(), y_str.parse::<u64>().unwrap()))
        .collect();

    let mut pairs: Vec<((u64, u64), (u64, u64))> = Vec::new();
    for i in 0..corners.len() {
        for j in i + 1..corners.len() {
            pairs.push((corners[i], corners[j]));
        }
    }

    let result = pairs.iter().fold(0, |acc, &((x1, y1), (x2, y2))| {
        let area = (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1);
        area.max(acc)
    });

    println!("Largest Area is {}", result);
}
