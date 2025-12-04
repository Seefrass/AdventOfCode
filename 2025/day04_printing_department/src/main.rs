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

    let result1 = process_rolls(&mut grid, false);
    let result2 = process_rolls(&mut grid, true);

    println!("A total of {} rolls can be accessed", result1);
    println!("A total of {} rolls can be removed", result2);
}

fn process_rolls(grid: &mut Vec<Vec<char>>, remove: bool) -> u32 {
    let mut result = 0;

    loop {
        let mut roll_count = 0;
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if is_accessible(x, y, &grid) {
                    if remove {
                        grid[y][x] = '.';
                    }
                    roll_count += 1
                }
            }
        }
        result += roll_count;
        if roll_count == 0 || !remove {
            break result;
        }
    }
}

fn is_accessible(x: usize, y: usize, grid: &Vec<Vec<char>>) -> bool {
    if grid[y][x] != '@' {
        return false;
    }

    let mut num_neighbors = 0;
    for i in [-1, 0, 1] {
        let idx_y = y as i32 + i;
        if idx_y < 0 || idx_y >= grid.len() as i32 {
            continue;
        }

        for j in [-1, 0, 1] {
            if i == 0 && j == 0 {
                continue;
            }

            let idx_x = x as i32 + j;
            if idx_x < 0 || idx_x >= grid[0].len() as i32 {
                continue;
            }

            let val = grid[idx_y as usize][idx_x as usize];
            if val == '@' {
                num_neighbors += 1;
            }
        }
    }

    num_neighbors < 4
}
