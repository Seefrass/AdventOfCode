use std::{
    fs::File,
    io::{BufReader, Read},
};

use microlp::Problem;
use regex::Regex;

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

    let result: u64 = input
        .split("\n\n")
        .map(|s| ClawMachine::try_from(s).unwrap().solve())
        .flatten()
        .sum();
    println!(
        "total tokens required to beat all claw machine games: {}",
        result
    );
    let result: u64 = input
        .split("\n\n")
        .map(|s| ClawMachine::try_from(s).unwrap().solve_adjusted())
        .flatten()
        .sum();
    println!(
        "adjusted total tokens required to beat all claw machine games: {}",
        result
    );
}

#[derive(Debug)]
struct ClawMachine {
    x_a: u64,
    y_a: u64,
    x_b: u64,
    y_b: u64,
    x_dest: u64,
    y_dest: u64,
}

impl ClawMachine {
    fn solve(&self) -> Option<u64> {
        let mut problem = Problem::new(microlp::OptimizationDirection::Minimize);
        let a = problem.add_integer_var(3.0, (0, 100));
        let b = problem.add_integer_var(1.0, (0, 100));
        problem.add_constraint(
            &[(a, self.x_a as f64), (b, self.x_b as f64)],
            microlp::ComparisonOp::Eq,
            self.x_dest as f64,
        );
        problem.add_constraint(
            &[(a, self.y_a as f64), (b, self.y_b as f64)],
            microlp::ComparisonOp::Eq,
            self.y_dest as f64,
        );
        if let Ok(solution) = problem.solve() {
            let solution = solution.into_solution().unwrap();
            Some(solution.objective() as u64)
        } else {
            None
        }
    }
    fn solve_adjusted(&self) -> Option<u64> {
        let mut problem = Problem::new(microlp::OptimizationDirection::Minimize);
        let a = problem.add_integer_var(3.0, (0, i32::MAX));
        let b = problem.add_integer_var(1.0, (0, i32::MAX));
        problem.add_constraint(
            &[(a, self.x_a as f64), (b, self.x_b as f64)],
            microlp::ComparisonOp::Eq,
            (self.x_dest + 10000000000000) as f64,
        );
        problem.add_constraint(
            &[(a, self.y_a as f64), (b, self.y_b as f64)],
            microlp::ComparisonOp::Eq,
            (self.y_dest + 10000000000000) as f64,
        );
        if let Ok(solution) = problem.solve() {
            let solution = solution.into_solution().unwrap();
            Some(solution.objective() as u64)
        } else {
            None
        }
    }
}

impl TryFrom<&str> for ClawMachine {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"Button A: X\+([0-9]+), Y\+([0-9]+)\nButton B: X\+([0-9]+), Y\+([0-9]+)\nPrize: X=([0-9]+), Y=([0-9]+)",
        )
        .unwrap();
        if let Some(c) = re.captures(s) {
            // println!("{:?}", c);
            let (_, [x_a, y_a, x_b, y_b, x_dest, y_dest]) = c.extract();
            Ok(ClawMachine {
                x_a: x_a.parse::<u64>().unwrap(),
                y_a: y_a.parse::<u64>().unwrap(),
                x_b: x_b.parse::<u64>().unwrap(),
                y_b: y_b.parse::<u64>().unwrap(),
                x_dest: x_dest.parse::<u64>().unwrap(),
                y_dest: y_dest.parse::<u64>().unwrap(),
            })
        } else {
            Err(())
        }
    }
}
