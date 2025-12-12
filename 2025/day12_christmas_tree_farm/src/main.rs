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

    let mut shapes = input.split("\n\n").collect::<Vec<_>>();
    let data = shapes.remove(shapes.len() - 1);

    let data = data
        .split("\n")
        .map(|l| {
            let (dim_str, shape_idxs) = l.split_once(":").unwrap();

            let (x, y) = dim_str.split_once("x").unwrap();
            let (x, y) = (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap());

            let shape_counts = shape_idxs
                .trim()
                .split(" ")
                .map(|idx| idx.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            ((x, y), shape_counts)
        })
        .collect::<Vec<_>>();

    let result = data
        .iter()
        .filter(|((x, y), v)| x * y >= v.iter().sum::<u32>() * 9)
        .count();

    println!("Number of plausible gifts: {}", result);
}
