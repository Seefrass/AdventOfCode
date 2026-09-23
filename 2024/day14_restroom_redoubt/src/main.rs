use std::{
    fs::File,
    io::{BufReader, Read},
};

use colored::Colorize;

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
    let mut robots: Vec<_> = input
        .split("\n")
        .map(|s| Robot::try_from(s).unwrap())
        .collect();
    let (w, e): (Vec<_>, Vec<_>) = robots
        .clone()
        .into_iter()
        .map(|mut r| {
            r.step(100);
            r.position
        })
        .filter(|p| p.0 != SPACE_WIDTH / 2 && p.1 != SPACE_HEIGHT / 2)
        .partition(|p| p.0 < SPACE_WIDTH / 2);

    let (nw, sw): (Vec<_>, Vec<_>) = w.into_iter().partition(|p| p.1 < SPACE_HEIGHT / 2);
    let (ne, se): (Vec<_>, Vec<_>) = e.into_iter().partition(|p| p.1 < SPACE_HEIGHT / 2);

    println!(
        "total safety factor is: {}",
        nw.len() * sw.len() * ne.len() * se.len()
    );

    // part 2: this problem is kind of frustrating, because the term "christmas
    // tree" is rather ambiguous.
    // first attempt: a christmas tree must have a trunk so we assum that there
    // is a vertical line perfectly in the middle of the board.
    // -> doesn't seem to work, there are not enough 1 spaces to for such a tree
    // second attempt: a christmas tree has no square with >= 1 robot
    // -> this seems to work consistently at least, if I find more motivation maybe
    // solve this in a nicer way (for example find some metric that shows how clustered
    // the robots are etc.)

    let mut steps = 1; //apparently we start at second 1?!?!?
    loop {
        for i in 0..robots.len() {
            robots[i].step(1);
        }
        let mut space = vec![vec![0; SPACE_WIDTH as usize]; SPACE_HEIGHT as usize];
        robots.iter().for_each(|r| {
            space[r.position.1 as usize][r.position.0 as usize] += 1;
            // print_space(&space);
            // println!();
        });
        if !space.iter().flatten().fold(false, |acc, n| acc || *n > 1) {
            print_space(&space);
            println!("this took {} seconds", steps);
            break;
        }
        steps += 1;
    }
}

fn print_space(space: &Vec<Vec<usize>>) {
    space.iter().for_each(|row| {
        row.iter().for_each(|n| {
            if *n > 0 {
                print!("{}", n.to_string().green().bold());
            } else {
                print!("{}", n);
            }
        });

        println!()
    });
}

#[derive(Debug, Clone)]
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
