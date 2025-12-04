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

    let result1 = count_accessible_rolls(&grid);
    let result2 = remove_accessible_rolls(&mut grid);

    println!("A total of {} rolls are accesible", result1);
    println!("A total of {} rolls can be removed", result2);
}

fn count_accessible_rolls(grid: &Vec<Vec<char>>) -> u32 {
    let size_x = grid[0].len();
    let size_y = grid.len();

    let mut result = 0;

    for x in 0..size_x {
        for y in 0..size_y {
            if is_accessible(x, y, &grid) {
                result += 1;
            }
        }
    }

    result
}

fn remove_accessible_rolls(grid: &mut Vec<Vec<char>>) -> u32 {
    let size_x = grid[0].len();
    let size_y = grid.len();

    let mut result = 0;

    loop {
        let mut roll_count = 0;
        for x in 0..size_x {
            for y in 0..size_y {
                if is_accessible(x, y, &grid) {
                    grid[y][x] = '.';
                    roll_count += 1
                }
            }
        }
        result += roll_count;
        if roll_count == 0 {
            break;
        }
    }

    result
}

fn is_accessible(x: usize, y: usize, grid: &Vec<Vec<char>>) -> bool {
    if grid[y][x] != '@' {
        return false;
    }

    let size_x = grid[0].len();
    let size_y = grid.len();

    let mut num_neighbors = 0;
    for i in vec![-1i32, 0i32, 1i32] {
        for j in vec![-1i32, 0i32, 1i32] {
            let idx_y = y as i32 + i;
            if idx_y < 0 || idx_y >= size_y as i32 {
                continue;
            }

            let idx_x = x as i32 + j;
            if idx_x < 0 || idx_x >= size_x as i32 {
                continue;
            }

            let val = grid[idx_y as usize][idx_x as usize];
            if val == '@' {
                num_neighbors += 1;
            }
        }
    }
    num_neighbors -= 1;

    num_neighbors < 4
}
