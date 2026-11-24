use std::{error::Error, fs::File, io::Read};

fn part_one() -> u64 {
    1
}

fn part_two() -> u64 {
    0
}

fn parse_file(filename: &str) -> Result<(), Box<dyn Error>> {
    // TODO:
    // - is returning Result<T,E> a good idea here? why not just panic?
    // - how to make user define Ok(T) type T? problem dependent.
    let mut file = File::open(filename)?;
    let mut data = String::new();

    file.read_to_string(&mut data)?;

    let data = data.trim(); // remove trailing newline. Is this the best way to do this?
    Ok(())
}

/* -------------------------------------------------------------------------- */

fn main() {
    const FILENAME: &str = "input.txt";
    let _ = parse_file(FILENAME);
    let result_one = part_one();
    println!("result one: {}", result_one);
    let result_two = part_two();
    println!("result two: {}", result_two);
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part_one() {
        const FILENAME: &str = "example.txt";
        const EXPECTED: u64 = 0; // cannot be const due to todo macro! const or todo()?

        // let data = parse_file(FILENAME).unwrap();

        // TODO: rephrase this! maybe replace with default assert!()
        assert_eq!(EXPECTED, part_one(), "correct answer is {}", EXPECTED);
    }

    #[test]
    fn test_example_part_two() {
        // this is kind of annoying because i don't want to test part two while doing part one...
        println!("test");
        const EXPECTED: u64 = 0;
        assert!(part_two() == 0);
    }
}
