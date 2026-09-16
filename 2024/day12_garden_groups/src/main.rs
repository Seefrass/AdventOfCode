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

    let mut map: Vec<Vec<Option<char>>> = input
        .split("\n")
        .map(|s| s.chars().map(|c| Some(c)).collect())
        .collect();
    // println!("{:?}", map);
    // IDEA: mark cells with optional, cells that have been assigned to a region
    // become NONE

    let mut regions: Vec<Region> = Vec::new();
    for y in 0..map.len() {
        for x in 0..map.first().unwrap().len() {
            regions.push(Region::from_map(&mut map, x, y));
        }
    }

    let result: usize = regions.iter().map(|r| r.price()).sum();

    println!("Total fence price: {}", result);

    let result: usize = regions.iter().map(|r| r.bulk_price()).sum();

    println!("Total fence price considering bulk discount: {}", result);
}

struct Region {
    //make these unsigned so out of bounds checks can be omitted!
    cells: Vec<(i64, i64)>,
}

impl Region {
    fn extract_cell(
        map: &mut Vec<Vec<Option<char>>>,
        x: usize,
        y: usize,
        id: char,
        cells: &mut Vec<(i64, i64)>,
    ) {
        match map[y][x] {
            Some(c) if c == id => {
                cells.push((i64::try_from(x).unwrap(), i64::try_from(y).unwrap()));
                map[y][x] = None;
                //unnecessary double call on none...
                if x != map.len() - 1 {
                    Region::extract_cell(map, x + 1, y, id, cells);
                }
                if x != 0 {
                    Region::extract_cell(map, x - 1, y, id, cells);
                }
                if y != map.first().unwrap().len() - 1 {
                    Region::extract_cell(map, x, y + 1, id, cells);
                }
                if y != 0 {
                    Region::extract_cell(map, x, y - 1, id, cells);
                }
            }
            _ => {
                return;
            }
        }
    }
    /// Extracts the Region containing plant at position (x, y) from a map
    fn from_map(map: &mut Vec<Vec<Option<char>>>, x: usize, y: usize) -> Self {
        if let Some(id) = map[y][x] {
            let mut cells = Vec::new();
            Region::extract_cell(map, x, y, id, &mut cells);
            Region { cells }
        } else {
            Region { cells: Vec::new() }
        }
    }
    fn area(&self) -> usize {
        self.cells.len()
    }
    fn perimeter(&self) -> usize {
        let mut result = 0;
        for (x, y) in self.cells.clone() {
            if !self.cells.contains(&(x - 1, y)) {
                result += 1;
            }
            if !self.cells.contains(&(x + 1, y)) {
                result += 1;
            }
            if !self.cells.contains(&(x, y - 1)) {
                result += 1;
            }
            if !self.cells.contains(&(x, y + 1)) {
                result += 1;
            }
        }
        result
    }
    fn sides(&self) -> usize {
        //IDEA: #sides = #corners, therfore we only have to find all positive and
        // negative corners!
        // This got kind of unreadable, refactor this later maybe :-)
        let mut result = 0;
        for (x, y) in self.cells.clone() {
            for dx in [-1, 1] {
                for dy in [-1, 1] {
                    if !self.cells.contains(&(x + dx, y + dy)) {
                        if self.cells.contains(&(x + dx, y)) && self.cells.contains(&(x, y + dy))
                            || !self.cells.contains(&(x + dx, y))
                                && !self.cells.contains(&(x, y + dy))
                        {
                            result += 1;
                        }
                    } else {
                        //special case: inner edge of region (example5)
                        if !self.cells.contains(&(x + dx, y)) && !self.cells.contains(&(x, y + dy))
                        {
                            result += 1;
                        }
                    }
                }
            }
        }
        result
    }
    fn price(&self) -> usize {
        self.area() * self.perimeter()
    }
    fn bulk_price(&self) -> usize {
        self.area() * self.sides()
    }
}
