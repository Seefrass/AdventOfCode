use std::{
    fs::File,
    io::{BufReader, Read},
};

use regex::Regex;

const FILE_NAME: &str = "input.txt";

//constanst for the amount of tiles (change this depending on input/example)
const SPACE_WIDTH: i32 = 101;
const SPACE_HEIGHT: i32 = 103;

fn main() {
    let file = File::open(FILE_NAME).unwrap();
    let mut reader = BufReader::new(file);
    let mut input = String::new();

    reader
        .read_to_string(&mut input)
        .expect("could not read from file");

    let input = input.trim();

    //--- Actual Task starts here ---//
    let robots = input.split("\n").map(|s| {
        let mut r = Robot::try_from(s).unwrap();
        r.step(100);
        r.position
    });
    // .for_each(|r| println!("{:?}", r));
    let (w, e): (Vec<_>, Vec<_>) = robots
        .clone()
        .filter(|p| p.0 != SPACE_WIDTH / 2 && p.1 != SPACE_HEIGHT / 2)
        .partition(|p| p.0 < SPACE_WIDTH / 2);

    let (nw, sw): (Vec<_>, Vec<_>) = w.into_iter().partition(|p| p.1 < SPACE_HEIGHT / 2);
    let (ne, se): (Vec<_>, Vec<_>) = e.into_iter().partition(|p| p.1 < SPACE_HEIGHT / 2);

    println!(
        "total safety factor is: {}",
        nw.len() * sw.len() * ne.len() * se.len()
    );
}

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn step(&mut self, n: usize) -> () {
        // potentially optimize this by removing the for loop through finding a formula
        // to directly calculate the final position.
        for _ in 0..n {
            self.position.0 = (self.position.0 + self.velocity.0 + SPACE_WIDTH) % SPACE_WIDTH;
            self.position.1 = (self.position.1 + self.velocity.1 + SPACE_HEIGHT) % SPACE_HEIGHT;
        }
    }
}

impl TryFrom<&str> for Robot {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
        if let Some(c) = re.captures(s) {
            let (_, [p_x, p_y, v_x, v_y]) = c.extract();
            Ok(Robot {
                position: (p_x.parse::<i32>().unwrap(), p_y.parse::<i32>().unwrap()),
                velocity: (v_x.parse::<i32>().unwrap(), v_y.parse::<i32>().unwrap()),
            })
        } else {
            Err(())
        }
    }
}
