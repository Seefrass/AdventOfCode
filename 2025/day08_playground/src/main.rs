use std::{
    collections::HashSet,
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

    let data: Vec<Vec<i64>> = input
        .split("\n")
        .map(|s| {
            s.split(",")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    let mut distances: Vec<(u64, usize, usize)> = Vec::new();

    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            distances.push((euclidean_distance(&data[i], &data[j]), i, j));
        }
    }

    distances.sort_by(|(d1, _, _), (d2, _, _)| d1.cmp(&d2));

    let mut groups: Vec<HashSet<usize>> = Vec::new();
    for i in 0..data.len() {
        groups.push(HashSet::from([i]));
    }

    distances
        .clone()
        .iter()
        .take(1_000)
        .for_each(|(_, idx1, idx2)| {
            sort_to_groups(&mut groups, *idx1, *idx2);
        });

    groups.sort_by(|g1, g2| g1.len().cmp(&g2.len()));
    let result: usize = groups.iter().rev().take(3).map(|g| g.len()).product();

    println!("Product of the three largest circuits: {}", result);

    // Part two

    let mut groups: Vec<HashSet<usize>> = Vec::new();
    for i in 0..data.len() {
        groups.push(HashSet::from([i]));
    }

    let mut distances_iter = distances.iter();
    let mut current = (0, 0, 0);
    while groups.len() > 1 {
        current = *distances_iter.next().unwrap();
        sort_to_groups(&mut groups, current.1, current.2);
    }

    let result = data[current.1].first().unwrap() * data[current.2].first().unwrap();

    println!(
        "Product of the x coordinates of the last junction: {}",
        result
    );
}

// TODO: maybe make this return f32 if more accuracy is needed...
fn euclidean_distance(v1: &Vec<i64>, v2: &Vec<i64>) -> u64 {
    let result: u64 = v1
        .iter()
        .zip(v2.iter())
        .map(|(v1, v2)| ((v1 - v2) * (v1 - v2)) as u64)
        .sum();
    u64::isqrt(result as u64)
}

fn sort_to_groups(groups: &mut Vec<HashSet<usize>>, idx1: usize, idx2: usize) {
    let g1_idx = groups.iter().position(|g| g.contains(&idx1)).unwrap();
    let g1 = groups.remove(g1_idx);

    if g1.contains(&idx2) {
        groups.push(g1);
        return;
    }

    let g2_idx = groups.iter().position(|g| g.contains(&idx2)).unwrap();
    let g2 = groups.remove(g2_idx);

    let group: HashSet<_> = g1.union(&g2).map(|&v| v).collect();
    groups.push(group);
}
