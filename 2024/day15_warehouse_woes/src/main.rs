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

    let (map, moves) = input.split_once("\n\n").unwrap();
    // println!("map:\n{}\nmoves:\n{}", map, moves);
    let mut map: Vec<Vec<char>> = map
        .split("\n")
        .map(|r| r.chars().collect::<Vec<char>>())
        .collect();
    let moves: Vec<(i32, i32)> = moves
        .replace("\n", "")
        .chars()
        .into_iter()
        .map(|c| match c {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            e => panic!("invalid move token: {}", e),
        })
        .collect();
    let mut robot = find_robot(&map).unwrap();
    // println!("robot: {:?}", robot);
    for m in moves {
        robot = move_object(&mut map, robot, m).unwrap_or(robot);
    }
    let result = evaluate_map(&map);
    println!("sum of all boxes' GPS coordinates: {}", result);
}

// Recursively calls the move_object until either a free space '.' or a wall '#'
// is hit. Returns None if moving is impossible and Some(pos) with pos being the
// desired position you just moved to otherwise.
fn move_object(
    map: &mut Vec<Vec<char>>,
    origin: (i32, i32),
    direction: (i32, i32),
) -> Option<(i32, i32)> {
    let goal = (origin.0 + direction.0, origin.1 + direction.1);
    match map[goal.1 as usize][goal.0 as usize] {
        '#' => {
            return None;
        }
        '.' => {
            map[goal.1 as usize][goal.0 as usize] = map[origin.1 as usize][origin.0 as usize];
            map[origin.1 as usize][origin.0 as usize] = '.';
            return Some(goal);
        }
        'O' => match move_object(map, goal, direction) {
            None => {
                return None;
            }
            Some((x, y)) => {
                map[goal.1 as usize][goal.0 as usize] = map[origin.1 as usize][origin.0 as usize];
                map[origin.1 as usize][origin.0 as usize] = '.';
                return Some(goal);
            }
        },
        e => {
            panic!("invalid token on map: {}", e);
        }
    }
}

fn find_robot(map: &Vec<Vec<char>>) -> Option<(i32, i32)> {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '@' {
                return Some((x as i32, y as i32));
            }
        }
    }
    None
}

fn evaluate_map(map: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'O' {
                result += 100 * y + x;
            }
        }
    }
    result
}
