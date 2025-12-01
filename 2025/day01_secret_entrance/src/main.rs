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

    let mut result1: u16 = 0;
    let mut result2: u16 = 0;

    input
        .split("\n")
        .map(|l| decode_line(l))
        .fold(50, |acc, mut num| {
            while num >= 100 {
                num -= 100;
                result2 += 1;
            }

            while num <= -100 {
                num += 100;
                result2 += 1;
            }

            let mut res = acc + num; //this could be < 0 or >= 100, acc is assumed to be \in [0, 99]

            if res < 0 {
                if acc != 0 {
                    result2 += 1;
                }
                res += 100
            } else if res > 100 {
                result2 += 1;
                res -= 100
            } else if res == 0 || res == 100 {
                res = 0;
                result1 += 1;
                result2 += 1;
            }

            res
        });

    println!("{}, {}", result1, result2)
}

fn decode_line(line: &str) -> i32 {
    let (sign, num_str) = line.split_at(1);
    let num = num_str.parse::<i32>().unwrap();
    match sign {
        "R" => num,
        "L" => -num,
        _ => panic!("invalid file format"),
    }
}
