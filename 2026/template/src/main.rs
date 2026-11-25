use std::{fs::File, io::Read};

/// TODO:
/// - documentation for type T alias
/// - find a suitable name
type T = (); // TODO: set correct type

/// TODO: documentation...
fn parse_file(filename: &str) -> T {
    let mut file = File::open(filename).unwrap();
    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let data = data.trim(); // remove trailing newline. Is this the best way to do this?

    todo!()
}

fn part_one(data : &mut T) -> u64 {
    let _ = data;
    todo!() //TODO: message?
}

fn part_two(data : &mut T) -> u64 {
    let _ = data;
    todo!()
}

/* -------------------------------------------------------------------------- */

fn main() {
    const FILENAME: &str = "input.txt";
    let mut data = parse_file(FILENAME);

    // shared data for both parts goes here!

    let result = part_one(&mut data.clone());
    println!("result one: {}", result);

    let result = part_two(&mut data);
    println!("result two: {}", result);
}

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_one() {
        const FILENAME: &str = "example.txt";
        const EXPECTED: u64 = 0; // TODO: transfer solution from problem stmt

        let mut data = parse_file(FILENAME);
        let result = part_one(&mut data);

        assert!(EXPECTED == result, "expected {} but got {}", EXPECTED, result);
    }

    #[test]
    #[ignore = "part_two() not yet implemented."]
    fn test_example_part_two() {
        const FILENAME: &str = "example.txt";
        const EXPECTED: u64 = 0; // TODO: transfer solution from problem stmt

        let mut data = parse_file(FILENAME);
        let result = part_one(&mut data);

        assert!(EXPECTED == result, "expected {} but got {}", EXPECTED, result);
    }
}
