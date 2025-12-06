use core::panic;
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Read, Write},
};

const FILE_NAME: &str = "example.txt";

fn main() {
    let file = File::open(FILE_NAME).unwrap();
    let mut reader = BufReader::new(file);
    let mut input = String::new();

    reader
        .read_to_string(&mut input)
        .expect("could not read from file");

    let input = input.trim();

    //--- Actual Task starts here ---//

    let mut input: Vec<&str> = input.split("\n").collect();
    let operations = input.remove(input.len() - 1);

    let data1: Vec<Vec<u64>> = input
        .iter()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();

    let operations: Vec<&str> = operations.split_ascii_whitespace().collect();

    let mut result = 0;

    for j in 0..operations.len() {
        match operations[j] {
            "+" => {
                let mut tmp = 0;
                for i in 0..data1.len() {
                    tmp += data1[i][j];
                }
                result += tmp;
            }
            "*" => {
                let mut tmp = 1;
                for i in 0..data1.len() {
                    tmp *= data1[i][j];
                }
                result += tmp;
            }
            _ => {
                panic!("invalid operation")
            }
        }
    }

    println!("{}", result);

    fs::remove_file("input_formatted.txt").unwrap();
    let file2 = File::create_new("input_formatted.txt").unwrap();
    let mut writer = BufWriter::new(file2);

    for j in 0..input[0].len() {
        let mut current_line = String::new();
        for i in 0..input.len() {
            current_line.push_str(&(input[i])[(input[0].len() - 1 - j)..=(input[0].len() - 1 - j)]);
        }
        current_line.push('\n');
        writer.write(current_line.as_bytes()).unwrap();
    }
    writer.flush().unwrap();

    let file2 = File::open("input_formatted.txt").unwrap();
    let mut reader = BufReader::new(file2);
    let mut input2 = String::new();

    reader
        .read_to_string(&mut input2)
        .expect("could not read from file");

    let input2 = input2.trim();

    let mut split_str = String::from(" ").repeat(data1.len());
    split_str.push('\n');

    let data2: Vec<Vec<u64>> = input2
        .split(&split_str)
        .map(|s| {
            s.trim()
                .split("\n")
                .map(|str| str.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();

    let mut result2: u64 = 0;

    let mut data_idx = 0;
    for op in operations.iter().rev() {
        match *op {
            "+" => result2 += data2[data_idx].iter().sum::<u64>(),
            "*" => result2 += data2[data_idx].iter().product::<u64>(),
            _ => panic!("invalid operation"),
        }
        data_idx += 1;
    }

    println!("{}", result2);
}
