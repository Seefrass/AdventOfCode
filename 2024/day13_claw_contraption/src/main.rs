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
    x_a: i64,
    y_a: i64,
    x_b: i64,
    y_b: i64,
    x_dest: i64,
    y_dest: i64,
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
    //Probem: microlp does not support variable sizes greater than i32:MAX which
    // is definitely required here!
    //
    // ---
    //
    // "it is tempting, if the only tool you have is a hammer, to treat everything
    // as if it were a nail." - Abraham Maslow
    //
    // after solving Year 2025 day 10 using linear programming i was amazed, how
    // elegant a lp solver solution can be. So of course when hearing the terms
    // "minimum" and other constraints I didn't even think about the fact that this
    // is a linear problem with 2 terms and 2 variables and thus solvable (with
    // exactly one solution) by hand...
    // after some simple linear transformations I made on a piece of paper the solution
    // is stupidly simple... (and also faster)
    // still leaving part 1 in though as a lesson, also it is easier to use linear
    // programming interfaces like good_lp and rooc instead of microlp directy.
    fn solve_adjusted(&self) -> Option<u64> {
        let b_num =
            (self.y_dest + 10000000000000) * self.x_a - (self.x_dest + 10000000000000) * self.y_a;
        let b_denom = self.y_b * self.x_a - self.y_a * self.x_b;
        if b_num % b_denom != 0 {
            return None;
        }
        let b = b_num / b_denom;
        let a_num = (self.x_dest + 10000000000000) - self.x_b * b;
        let a_denom = self.x_a;
        if a_num % a_denom != 0 {
            return None;
        }
        let a = a_num / a_denom;
        if a < 0 || b < 0 {
            return None;
        }
        Some((3 * a + b) as u64)
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
                x_a: x_a.parse::<i64>().unwrap(),
                y_a: y_a.parse::<i64>().unwrap(),
                x_b: x_b.parse::<i64>().unwrap(),
                y_b: y_b.parse::<i64>().unwrap(),
                x_dest: x_dest.parse::<i64>().unwrap(),
                y_dest: y_dest.parse::<i64>().unwrap(),
            })
        } else {
            Err(())
        }
    }
}
