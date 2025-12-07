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

    let mut grid: Vec<Vec<char>> = input.split("\n").map(|s| s.chars().collect()).collect();

    let start_idx = grid.remove(0).iter().position(|&c| c == 'S').unwrap();
    let mut split_cnt = 0;

    grid.clone().iter().fold(vec![start_idx], |acc, line| {
        tachion_step(acc, line, &mut split_cnt)
    });

    let timeline_cnt: u64 = grid
        .iter()
        .fold(vec![(start_idx, 1)], |acc, line| tachion_step2(acc, line))
        .iter()
        .map(|(_, t)| t)
        .sum();

    println!("Total number of splits: {}", split_cnt);
    println!("Total number of timelines {}", timeline_cnt);
}

fn tachion_step(t_idx: Vec<usize>, line: &Vec<char>, split_cnt: &mut u32) -> Vec<usize> {
    let mut result = Vec::new();

    t_idx.iter().for_each(|&i| {
        if line[i] == '^' {
            *split_cnt += 1;
            result.push(i - 1);
            result.push(i + 1);
        } else {
            result.push(i);
        }
    });

    result.sort();
    result.dedup();
    result
}

fn tachion_step2(t_idx: Vec<(usize, u64)>, line: &Vec<char>) -> Vec<(usize, u64)> {
    let mut tmp = Vec::new();

    t_idx.iter().for_each(|&(i, timeline_num)| {
        if line[i] == '^' {
            tmp.push((i - 1, timeline_num));
            tmp.push((i + 1, timeline_num));
        } else {
            tmp.push((i, timeline_num));
        }
    });

    let mut result = Vec::new();

    let mut current_idx = 0;
    let mut idx_timelines = 0;

    for (idx, timeline_num) in tmp {
        if current_idx == idx {
            idx_timelines += timeline_num;
        } else {
            result.push((current_idx, idx_timelines));
            current_idx = idx;
            idx_timelines = timeline_num;
        }
    }
    result.push((current_idx, idx_timelines));

    result
}
