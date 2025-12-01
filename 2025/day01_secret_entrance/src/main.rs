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

    let mut input = input.trim();

    //--- Actual Task starts here ---//

    let mut result: u16 = 0;

    input
        .split("\n")
        .map(|l| decode_line(l))
        .fold(50, |acc, num| {
            let res = acc + num;

            if res % 100 == 0 {
                result += 1
            };
            res
        });

    println!("{}", result)
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
