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

    // Note: I'm not really happy with this solution, as it just becomes unreadable
    // when looking at all the case distinctions i have to make in push :(
    // maybe some day, I'll re-do this task in a nicer way.

    let (map_str, moves) = input.split_once("\n\n").unwrap();
    // println!("map:\n{}\nmoves:\n{}", map, moves);
    let mut map: Vec<Vec<char>> = map_str
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
    for m in moves.clone() {
        robot = move_object(&mut map, robot, m).unwrap_or(robot);
    }
    let result = evaluate_map(&map);
    println!("sum of all boxes' GPS coordinates: {}", result);

    let mut wide_map: Vec<Vec<char>> = map_str
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.")
        .split("\n")
        .map(|r| r.chars().collect::<Vec<char>>())
        .collect();
    let mut robot = find_robot(&wide_map).unwrap();
    for m in moves {
        // print_map(&wide_map);
        if m == (-1, 0) || m == (1, 0) {
            // pushing horizontally works exactly like part one so we just reuse
            // this code.
            // TODO: refactor this code... wide_move_object isn't necessary anymore,
            // but for some reason removing it causes stack overflow!?!?
            robot = wide_move_object(&mut wide_map, robot, m).unwrap_or(robot);
        } else {
            // the problem gets harder now so we split checking and actual pushing.
            //
            if can_push(&wide_map, robot, m) {
                push(&mut wide_map, robot, m);
                robot = (robot.0 + m.0, robot.1 + m.1);
            }
        }
    }
    let result = evaluate_map(&wide_map);
    // print_map(&wide_map);
    println!("sum of all wide boxes' GPS coordinates: {}", result);
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
            Some(_) => {
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

fn can_push(map: &Vec<Vec<char>>, origin: (i32, i32), direction: (i32, i32)) -> bool {
    let goal = (origin.0 + direction.0, origin.1 + direction.1);
    match map[goal.1 as usize][goal.0 as usize] {
        '#' => {
            return false;
        }
        '.' => {
            return true;
        }
        '[' => {
            return can_push(map, goal, direction) && can_push(map, (goal.0 + 1, goal.1), direction)
        }
        ']' => {
            return can_push(map, goal, direction) && can_push(map, (goal.0 - 1, goal.1), direction)
        }
        e => {
            panic!("invalid token on map: {}", e);
        }
    }
}

fn push(map: &mut Vec<Vec<char>>, origin: (i32, i32), direction: (i32, i32)) {
    match map[origin.1 as usize][origin.0 as usize] {
        '.' => return,
        '@' => {
            push(
                map,
                (origin.0 + direction.0, origin.1 + direction.1),
                direction,
            );
            map[(origin.1 + direction.1) as usize][(origin.0 + direction.0) as usize] = '@';
            map[origin.1 as usize][origin.0 as usize] = '.';
        }
        '[' => {
            if direction == (-1, 0) || direction == (1, 0) {
                push(
                    map,
                    (origin.0 + direction.0, origin.1 + direction.1),
                    direction,
                );
                map[(origin.1 + direction.1) as usize][(origin.0 + direction.0) as usize] = '[';
                map[origin.1 as usize][origin.0 as usize] = '.';
            } else {
                if map[(origin.1 + direction.1) as usize][(origin.0 + direction.0) as usize] == ']'
                    && map[(origin.1 + direction.1) as usize][(origin.0 + direction.0 + 1) as usize]
                        == '['
                {
                    push(
                        map,
                        (origin.0 + direction.0, origin.1 + direction.1),
                        direction,
                    );
                    push(
                        map,
                        (origin.0 + direction.0 + 1, origin.1 + direction.1),
                        direction,
                    );
                } else if map[(origin.1 + direction.1) as usize][(origin.0 + direction.0) as usize]
                    == '.'
                    && map[(origin.1 + direction.1) as usize][(origin.0 + direction.0 + 1) as usize]
                        == '['
                {
                    push(
                        map,
                        (origin.0 + direction.0 + 1, origin.1 + direction.1),
                        direction,
                    );
                } else {
                    push(
                        map,
                        (origin.0 + direction.0, origin.1 + direction.1),
                        direction,
                    );
                }
                map[(origin.1 + direction.1) as usize][(origin.0 + direction.0) as usize] = '[';
                map[origin.1 as usize][origin.0 as usize] = '.';
                map[(origin.1 + direction.1) as usize][(origin.0 + direction.0 + 1) as usize] = ']';
                map[origin.1 as usize][(origin.0 + 1) as usize] = '.';
            }
        }
        ']' => {
            if direction == (-1, 0) || direction == (1, 0) {
                push(
                    map,
                    (origin.0 + direction.0, origin.1 + direction.1),
                    direction,
                );
                map[(origin.1 + direction.1) as usize][(origin.0 + direction.0) as usize] = ']';
                map[origin.1 as usize][origin.0 as usize] = '.';
            } else {
                if map[(origin.1 + direction.1) as usize][(origin.0 + direction.0) as usize] == '['
                    && map[(origin.1 + direction.1) as usize][(origin.0 + direction.0 - 1) as usize]
                        == ']'
                {
                    push(
                        map,
                        (origin.0 + direction.0, origin.1 + direction.1),
                        direction,
                    );
                    push(
                        map,
                        (origin.0 + direction.0 - 1, origin.1 + direction.1),
                        direction,
                    );
                } else if map[(origin.1 + direction.1) as usize][(origin.0 + direction.0) as usize]
                    == '.'
                    && map[(origin.1 + direction.1) as usize][(origin.0 + direction.0 - 1) as usize]
                        == ']'
                {
                    push(
                        map,
                        (origin.0 + direction.0 - 1, origin.1 + direction.1),
                        direction,
                    );
                } else {
                    push(
                        map,
                        (origin.0 + direction.0, origin.1 + direction.1),
                        direction,
                    );
                }
                map[(origin.1 + direction.1) as usize][(origin.0 + direction.0) as usize] = ']';
                map[origin.1 as usize][origin.0 as usize] = '.';
                map[(origin.1 + direction.1) as usize][(origin.0 + direction.0 - 1) as usize] = '[';
                map[origin.1 as usize][(origin.0 - 1) as usize] = '.';
            }
        }
        _ => panic!(),
    }
}

fn wide_move_object(
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
        '[' => {
            if direction == (-1, 0) || direction == (1, 0) {
                match wide_move_object(map, goal, direction) {
                    None => {
                        return None;
                    }
                    Some(_) => {
                        map[goal.1 as usize][goal.0 as usize] =
                            map[origin.1 as usize][origin.0 as usize];
                        map[origin.1 as usize][origin.0 as usize] = '.';
                        return Some(goal);
                    }
                }
            } else {
                match (
                    wide_move_object(map, goal, direction),
                    wide_move_object(map, (goal.0 + 1, goal.1), direction),
                ) {
                    (Some(_), Some(_)) => {
                        if map[origin.1 as usize][origin.0 as usize] == '@' {
                            map[goal.1 as usize][goal.0 as usize] =
                                map[origin.1 as usize][origin.0 as usize];
                            map[origin.1 as usize][origin.0 as usize] = '.';
                            return Some(goal);
                        }
                        map[goal.1 as usize][goal.0 as usize] =
                            map[origin.1 as usize][origin.0 as usize];
                        map[origin.1 as usize][origin.0 as usize] = '.';
                        map[goal.1 as usize][(goal.0 + 1) as usize] =
                            map[origin.1 as usize][(origin.0 + 1) as usize];
                        map[origin.1 as usize][(origin.0 + 1) as usize] = '.';
                        return Some(goal);
                    }
                    _ => {
                        return None;
                    }
                }
            }
        }
        ']' => {
            if direction == (-1, 0) || direction == (1, 0) {
                match wide_move_object(map, goal, direction) {
                    None => {
                        return None;
                    }
                    Some(_) => {
                        map[goal.1 as usize][goal.0 as usize] =
                            map[origin.1 as usize][origin.0 as usize];
                        map[origin.1 as usize][origin.0 as usize] = '.';
                        return Some(goal);
                    }
                }
            } else {
                match (
                    wide_move_object(map, goal, direction),
                    wide_move_object(map, (goal.0 - 1, goal.1), direction),
                ) {
                    (Some(_), Some(_)) => {
                        if map[origin.1 as usize][origin.0 as usize] == '@' {
                            map[goal.1 as usize][goal.0 as usize] =
                                map[origin.1 as usize][origin.0 as usize];
                            map[origin.1 as usize][origin.0 as usize] = '.';
                            return Some(goal);
                        }
                        map[goal.1 as usize][goal.0 as usize] =
                            map[origin.1 as usize][origin.0 as usize];
                        map[origin.1 as usize][origin.0 as usize] = '.';
                        map[goal.1 as usize][(goal.0 - 1) as usize] =
                            map[origin.1 as usize][(origin.0 - 1) as usize];
                        map[origin.1 as usize][(origin.0 - 1) as usize] = '.';
                        return Some(goal);
                    }
                    _ => {
                        return None;
                    }
                }
            }
        }
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
            if map[y][x] == 'O' || map[y][x] == '[' {
                result += 100 * y + x;
            }
        }
    }
    result
}

fn print_map(map: &Vec<Vec<char>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", map[y][x]);
        }
        println!();
    }
}
