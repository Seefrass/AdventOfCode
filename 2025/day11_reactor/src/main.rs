use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

const FILE_NAME: &str = "input.txt";

struct ArenaDAG<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

impl<T> ArenaDAG<T>
where
    T: PartialEq,
{
    fn new() -> Self {
        ArenaDAG { arena: Vec::new() }
    }

    fn find(&self, val: T) -> Option<usize> {
        for node in &self.arena {
            if node.val == val {
                return Some(node.idx);
            }
        }
        None
    }

    fn find_or_insert_node(&mut self, val: T) -> usize {
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }

        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        idx
    }

    fn count_paths_to_node(
        &self,
        node_idx: usize,
        end_idx: usize,
        cache: &mut HashMap<usize, usize>,
    ) -> usize {
        let node = &self.arena[node_idx];

        if node.idx == end_idx {
            return 1;
        }

        if cache.contains_key(&node_idx) {
            return *cache.get(&node_idx).unwrap();
        }

        let mut count = 0;
        for c in &node.children {
            count += self.count_paths_to_node(*c, end_idx, cache);
        }

        cache.insert(node_idx, count);

        count
    }
}

struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    val: T,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            children: Vec::new(),
        }
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

    let mut dag: ArenaDAG<&str> = ArenaDAG::new();

    let nodes = input
        .split("\n")
        .map(|l| {
            let (val, children) = l.split_once(":").unwrap();
            let val_idx = dag.find_or_insert_node(val);
            (val_idx, children.trim().split(" ").collect::<Vec<_>>())
        })
        .collect::<Vec<_>>();

    nodes.iter().for_each(|(val_idx, children)| {
        let children = children
            .iter()
            .map(|c| dag.find_or_insert_node(c))
            .collect::<Vec<_>>();

        let node = &mut dag.arena[*val_idx];
        node.children = children;
    });

    let start = dag.find("you").expect("a start node (you)");
    let out = dag.find("out").expect("an out node (out)");
    let mut cache = HashMap::new();

    let result = dag.count_paths_to_node(start, out, &mut cache);

    println!("Number of paths to out: {}", result);

    let svr = dag.find("svr").unwrap();
    let fft = dag.find("fft").unwrap();
    let dac = dag.find("dac").unwrap();
    let out = dag.find("out").expect("an out node (out)");

    let mut c1 = HashMap::new();
    let mut c2 = HashMap::new();
    let mut c3 = HashMap::new();

    let result: usize = [
        dag.count_paths_to_node(svr, fft, &mut c1),
        dag.count_paths_to_node(fft, dac, &mut c2),
        dag.count_paths_to_node(dac, out, &mut c3),
    ]
    .iter()
    .product();

    println!("Number of paths to out visiting fft and dac: {}", result);
}
