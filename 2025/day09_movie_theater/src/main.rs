use std::{
    fs::File,
    io::{BufReader, Read},
    vec,
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

    let corners: Vec<(u64, u64)> = input
        .split("\n")
        .map(|l| l.split_once(",").unwrap())
        .map(|(x_str, y_str)| (x_str.parse::<u64>().unwrap(), y_str.parse::<u64>().unwrap()))
        .collect();

    let mut idx_pairs: Vec<(usize, usize)> = Vec::new();
    for idx1 in 0..corners.len() {
        for idx2 in idx1 + 1..corners.len() {
            idx_pairs.push((idx1, idx2));
        }
    }

    let result1 = idx_pairs
        .iter()
        .map(|&(idx1, idx2)| {
            let (x1, y1) = corners[idx1 as usize];
            let (x2, y2) = corners[idx2 as usize];
            (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)
        })
        .max()
        .unwrap();

    println!("Largest Area is {}", result1);

    let result2 = idx_pairs
        .iter()
        .filter(|&idx_pair| is_in_green(*idx_pair, corners.clone()))
        .map(|&(idx1, idx2)| {
            let (x1, y1) = corners[idx1 as usize];
            let (x2, y2) = corners[idx2 as usize];
            (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)
        })
        .max()
        .unwrap();

    println!("Largest Area within the green area is {}", result2);
}

fn is_in_green(idx_pair: (usize, usize), corners: Vec<(u64, u64)>) -> bool {
    let (x1, y1) = corners[idx_pair.0];
    let (x2, y2) = corners[idx_pair.1];

    let min_corner = (x1.min(x2), y1.min(y2));
    let max_corner = (x1.max(x2), y1.max(y2));

    // if there is a corner inside the middle of our rectangle we cannot be completely in the green area
    for c in corners.clone() {
        if c.0 > min_corner.0 && c.1 > min_corner.1 && c.0 < max_corner.0 && c.1 < max_corner.1 {
            return false;
        }
    }

    // let corner1 be the corner with the lower y value!
    let corner1_idx = if y1 < y2 { idx_pair.0 } else { idx_pair.1 };
    let corner1 = corners[corner1_idx];

    let corner2_idx = if y1 < y2 { idx_pair.1 } else { idx_pair.0 };
    let corner2 = corners[corner2_idx];

    // there are only 3 cases that can occurr now
    if corner1.0 < corner2.0 {
        /* CASE (1): c2 is to the right of c1
         *
         * first we travel along the outline of the green field starting at c1 until c2
         *
         *      c1--------->x
         *      .           |
         *      .           v
         *      . . . . . . c2
         *
         * on the path from c1->...->c2 no corners/edges are allowed to hit the dotted line!
         * (note that we already know that the paths wont be in the middle of the rectangle
         * thus the arrow shows the shortes path to c2, though it could of course be longer)
         */
        let mut corners_tmp = corners
            .clone()
            .into_iter()
            .cycle()
            .skip_while(|&c| c != corner1)
            .take_while(|&c| c != corner2)
            .collect::<Vec<(u64, u64)>>();
        corners_tmp.push(corner2);
        let path = to_full_path(&corners_tmp);

        for tile in path {
            if tile == corner1 {
                continue;
            }

            if tile == corner2 {
                break;
            }

            if tile.0 == corner1.0 {
                if (corner1.1 + 1..=corner2.1).contains(&tile.1) {
                    return false;
                }
            }
            if tile.1 == corner2.1 {
                if (corner1.0..corner2.0).contains(&tile.0) {
                    return false;
                }
            }
        }

        /*
         * now we travel back form c2 to c1, continuing the cycle
         *
         *      c1 . . . . . .
         *      ^            .
         *      |            .
         *      x <---------c2
         *
         * on the path from c2->...->c1 no corners/edges are allowed to hit the dotted line again!
         */

        let mut corners_tmp = corners
            .clone()
            .into_iter()
            .cycle()
            .skip_while(|&c| c != corner2)
            .take_while(|&c| c != corner1)
            .collect::<Vec<(u64, u64)>>();
        corners_tmp.push(corner1);
        let path = to_full_path(&corners_tmp);

        for tile in path {
            if tile == corner2 {
                continue;
            }

            if tile == corner1 {
                break;
            }

            if tile.1 == corner1.1 {
                if (corner1.0 + 1..=corner2.0).contains(&tile.0) {
                    return false;
                }
            }
            if tile.0 == corner2.0 {
                if (corner1.1..corner2.1).contains(&tile.1) {
                    return false;
                }
            }
        }
    } else {
        /* CASE (1): c2 is to the left of c1
         *
         * we have the following situation:
         *
         *      . . . . . . c1
         *      .           |
         *      .           v
         *     c2<----------x
         *
         * on the path from c1->...->c2 no corners are allowed on the dotted line!
         */

        let mut corners_tmp = corners
            .clone()
            .into_iter()
            .cycle()
            .skip_while(|&c| c != corner1)
            .take_while(|&c| c != corner2)
            .collect::<Vec<(u64, u64)>>();
        corners_tmp.push(corner2);
        let path = to_full_path(&corners_tmp);

        for tile in path {
            if tile == corner1 {
                continue;
            }

            if tile == corner2 {
                break;
            }

            if tile.1 == corner1.1 {
                if (corner2.0..corner1.0).contains(&tile.0) {
                    return false;
                }
            }
            if tile.0 == corner2.0 {
                if (corner1.1..corner2.1).contains(&tile.1) {
                    return false;
                }
            }
        }

        /*
         * we have the following situation:
         *
         *      x---------->c1
         *      ^           .
         *      |           .
         *     c2 . . . . . .
         *
         * on the path from c1->...->c2 no corners are allowed on the dotted line!
         */
        let mut corners_tmp = corners
            .clone()
            .into_iter()
            .cycle()
            .skip_while(|&c| c != corner2)
            .take_while(|&c| c != corner1)
            .collect::<Vec<(u64, u64)>>();
        corners_tmp.push(corner1);
        let path = to_full_path(&corners_tmp);

        for tile in path {
            if tile == corner1 {
                break;
            }

            if tile.0 == corner1.0 {
                if (corner1.1 + 1..=corner2.1).contains(&tile.1) {
                    return false;
                }
            }
            if tile.1 == corner2.1 {
                if (corner2.0 + 1..=corner1.0).contains(&tile.0) {
                    return false;
                }
            }
        }
    }

    /* CASE (3): c2 and c1 are in a line
     * in any case they are in the green
     */
    true
}

/// Convert a path of only corners to a completed path, filling all the spaces in between the
/// corners with edges. This costs a lot of time, since we call this method a lot, but it is
/// the only way I found to solve the following edge case
///
///     .....................
///     .#XXXX#....#XX#......
///     .X....X....X..X......
///     .X----X----X--#XC....
///     .X....X....X....X....
///     .CX#--X----X----X....
///     ...X..X....X....X....
///     ...X..#XXXX#....X....
///     ...X............X....
///     ...#XXXXXXXXXXXX#....
///     .....................
///
/// Look at the corners marked with 'C' they do not have any corners in inside their rectangle,
/// nor on the edges of their rectangle, however it is still not completely in the green tiles.
///
fn to_full_path(path: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut result = Vec::new();

    let remainder: (u64, u64) = *path
        .into_iter()
        .reduce(|c1, c2| {
            if c1.0 == c2.0 {
                let mut to_append = vec![];
                if c1.1 < c2.1 {
                    for i in c1.1..c2.1 {
                        to_append.push((c1.0, i));
                    }
                } else {
                    for i in c2.1..=c1.1 {
                        to_append.push((c1.0, i));
                    }
                    to_append.remove(0);
                    to_append.reverse();
                }
                result.append(&mut to_append);
            } else if c1.1 == c2.1 {
                let mut to_append = vec![];
                if c1.0 < c2.0 {
                    for i in c1.0..c2.0 {
                        to_append.push((i, c1.1));
                    }
                } else {
                    for i in c2.0..=c1.0 {
                        to_append.push((i, c1.1));
                    }
                    to_append.remove(0);
                    to_append.reverse();
                }
                result.append(&mut to_append);
            } else {
                dbg!((c1, c2));
                panic!("invalid input format!");
            }
            c2
        })
        .unwrap();
    result.push(remainder);

    result
}
