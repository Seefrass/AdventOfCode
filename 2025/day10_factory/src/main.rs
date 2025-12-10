use std::{
    fs::File,
    io::{BufReader, Read},
};

const FILE_NAME: &str = "input.txt";

struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u32>,
}

impl From<&str> for Machine {
    fn from(line: &str) -> Self {
        let mut line: Vec<&str> = line.split(" ").collect();
        let light_str = line.remove(0);
        let joltage_str = line.remove(line.len() - 1);

        let mut lights = Vec::new();
        light_str.chars().for_each(|c| match c {
            '.' => lights.push(false),
            '#' => lights.push(true),
            _ => (),
        });

        let joltages: Vec<u32> = joltage_str
            .strip_prefix("{")
            .unwrap()
            .strip_suffix("}")
            .unwrap()
            .split(',')
            .map(|j| j.parse::<u32>().unwrap())
            .collect();

        let buttons = line
            .iter()
            .map(|b| {
                b.strip_prefix("(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .split(",")
                    .map(|b| b.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect();

        Machine {
            lights: lights,
            buttons: buttons,
            joltages: joltages,
        }
    }
}

impl Machine {
    fn min_button_presses(&self) -> u32 {
        for n in 1..self.buttons.len() {
            let combinations = permutations_of_len(&self.buttons, n);
            let fulfills_combination = combinations
                .iter()
                .map(|buttons| {
                    let mut light = vec![false; self.lights.len()];
                    buttons
                        .iter()
                        .flatten()
                        .for_each(|&idx| light[idx] = !light[idx]);
                    self.lights == light
                })
                .any(|b| b == true);
            if fulfills_combination {
                return n as u32;
            }
        }
        panic!("no combination of buttons available!")
    }
}

fn main() {
    let file = File::open(FILE_NAME).unwrap();
    let mut reader = BufReader::new(file);
    let mut input = String::new();

    reader
        .read_to_string(&mut input)
        .expect("could not read from file");

    let input = input.trim();

    //--- Actual Task starts here ---//

    let machines: Vec<Machine> = input.split("\n").map(Machine::from).collect();
    let result: u32 = machines.iter().map(Machine::min_button_presses).sum();

    println!("Fewest total button presses required: {}", result)
}

fn permutations_of_len<T: Clone>(v: &Vec<T>, n: usize) -> Vec<Vec<T>> {
    if n > v.len() {
        return vec![];
    }

    if n == 1 {
        let mut result = Vec::new();
        for i in v {
            result.push(vec![i.clone()]);
        }
        return result;
    }

    let mut result = Vec::new();

    let mut residual = v.clone();
    for _ in 0..=residual.len() - n {
        let elem = residual.remove(0);

        let mut perms = permutations_of_len(&residual, n - 1);
        perms.iter_mut().for_each(|p| {
            let mut v = vec![elem.clone()];
            v.append(p);
            result.push(v);
        });
    }

    result
}
