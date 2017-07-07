use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
struct Matrix {
    elems: BTreeMap<char, BTreeMap<char, u32>>,
    keys: BTreeSet<char>,
}

impl Matrix {
    fn new() -> Matrix {
        Matrix { elems: BTreeMap::new() }
    }

    fn insert(&mut self, c1: &char, c2: &char, val: u32) {
        let mut ins_map = self.elems.entry(*c1).or_insert_with(BTreeMap::new);
        ins_map.insert(*c2, val);
        self.keys.insert(c1);
        self.keys.insert(c2);
    }

    fn s_calculation(&self, c: &char) -> f64 {
        
    }
}

fn neighbor_joining(input: &[(char, char, u32)]) {
    let mut matrix = Matrix::new();

    for &(c1, c2, val) in input {
        matrix.insert(&c1, &c2, val);
    }
    // println!("{:#?}", matrix);
}

fn main() {
    let input = vec![('A', 'B', 5),
                     ('A', 'C', 4),
                     ('A', 'D', 7),
                     ('A', 'E', 6),
                     ('A', 'F', 8),
                     ('B', 'C', 7),
                     ('B', 'D', 10),
                     ('B', 'E', 9),
                     ('B', 'F', 11),
                     ('C', 'D', 7),
                     ('C', 'E', 6),
                     ('C', 'F', 8),
                     ('D', 'E', 5),
                     ('D', 'F', 9),
                     ('E', 'F', 8)];

    neighbor_joining(&input);
}
