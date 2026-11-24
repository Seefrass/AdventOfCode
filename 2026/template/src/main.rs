use std::{fs::File, io::{BufReader, Read}};

fn main() {
}

fn part_one() -> u64 {
    0
}

fn part_two() -> u64 {
    0
}

fn read_file() -> String {
    return "".to_string();
}

fn parse_file() {}

#[cfg(test)]
mod tests {

use super::*;

    #[test]
    fn test_example_part_one() {
        // todo: name this lhs/rhs?
        const EXPECTED : u64 = 0; // cannot be const due to todo macro! const or todo()?
        println!("expecting result: {}", EXPECTED); // 

        const FILE_NAME : &str = "example.txt";

        // TODO:
        // 1) open example file
        // 2) read contents
        // 3) pass to part one
        // 4) return results / assert
        
        let file = File::open(FILE_NAME).unwrap();
        let mut reader = BufReader::new(file);
        let mut data = String::new();

        reader.read_to_string(&mut data).unwrap();

        let data = data.trim(); // remove trailing newline. Is this the best way to do this?

        let result = part_one();
        
        //how do I print the wrong result in case it is wrong?
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn test_example_part_two() {
        // this is kind of annoying because i don't want to test part two while doing part one...
        println!("test");
        const EXPECTED : u64 = 0;
        assert!(part_two() == 0);
    }
}
