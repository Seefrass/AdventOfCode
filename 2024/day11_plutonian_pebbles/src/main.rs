use std::{
    collections::HashMap,
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
    // Notes: Number of Pebbles grows worst case O(2^n). I try to optimize this
    // by having a ref count for each unique pebble. The Idea is that the number
    // of unique pebbles will eventually hit some kind of limit (I hope that the
    // * 2024 case wont be too big of a problem).

    let mut pebble_map: HashMap<String, u64> = HashMap::new();

    input
        .split(" ")
        // .map(|s| s.parse::<u64>().unwrap())
        .for_each(|n| {
            pebble_map
                .entry(n.to_string())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        });

    for _ in 0..75 {
        pebble_map = blink(&pebble_map);
        // println!("{:?}", pebble_map);
    }

    let result = pebble_map.into_values().sum::<u64>();

    println!("Total number of Pebbles after 25 blinks: {}", result);
}

fn blink(pebbles: &HashMap<String, u64>) -> HashMap<String, u64> {
    let mut new_pebble_map: HashMap<String, u64> = HashMap::new();
    pebbles.iter().for_each(|(n, &c)| {
        if n == "0" {
            new_pebble_map
                .entry("1".to_string())
                .and_modify(|x| *x += c)
                .or_insert(c);
        } else if n.len() % 2 == 0 {
            new_pebble_map
                .entry(String::from(&n[0..n.len() / 2]))
                .and_modify(|x| *x += c)
                .or_insert(c);
            new_pebble_map
                .entry(
                    String::from(&n[n.len() / 2..])
                        .parse::<u64>()
                        .unwrap()
                        .to_string(),
                )
                .and_modify(|x| *x += c)
                .or_insert(c);
        } else {
            let entry: String = (n.parse::<u64>().unwrap() * 2024).to_string();
            new_pebble_map
                .entry(entry)
                .and_modify(|x| *x += c)
                .or_insert(c);
        }
    });
    new_pebble_map
}
