use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
struct Matrix {
    elems: BTreeMap<String, BTreeMap<String, f32>>,
    keys: BTreeSet<String>,
}

impl Matrix {
    fn new() -> Matrix {
        Matrix {
            elems: BTreeMap::new(),
            keys: BTreeSet::new(),
        }
    }

    fn insert(&mut self, c1: &str, c2: &str, val: f32) {
        let mut ins_map = self.elems
            .entry(c1.to_string())
            .or_insert_with(BTreeMap::new);
        ins_map.insert(c2.to_string(), val);
        self.keys.insert(c1.to_string());
        self.keys.insert(c2.to_string());
    }

    fn len(&self) -> usize {
        self.keys.len()
    }

    fn get_d(&self, c1: &str, c2: &str) -> &f32 {
        match self.elems.get(c1) {
            Some(map) => {
                match map.get(c2) {
                    Some(val) => val,
                    None => &self.elems[c2][c1],
                }
            }
            None => {
                match self.elems.get(c2) {
                    Some(map) => {
                        match map.get(c1) {
                            Some(val) => val,
                            None => panic!("c1 wasn't found"),
                        }
                    }
                    None => panic!("c2 wasn't found"),
                }
            }
        }
    }

    fn lowest_d(&self) -> (f32, String, String) {
        let mut iter = self.keys.iter();
        let mut key_1 = match iter.next() {
            Some(val) => val,
            None => panic!("No elements in matrix"),
        };

        let mut lowest = (9999.9, key_1.clone(), key_1.clone());

        for key_2 in iter {
            let &d = self.get_d(key_1, key_2);

            if lowest.0 > d {
                lowest = (d, key_1.clone(), key_2.clone());
            }

            key_1 = key_2;
        }

        lowest
    }

    fn join(&mut self, k1: &str, k2: &str) -> Matrix {
        if !(self.keys.contains(k1) || self.keys.contains(k2)) {
            panic!("Keys aren't in matrix");
        }
        /********************************************************************/
        let var1 = &self.elems[k1];
        let var2 = &self.elems[k2];

        println!("{:?} > {:?}", var1, var2);
        /********************************************************************/
        let d_12 = self.get_d(k1, k2) / 2.0;
        let d_21 = self.get_d(k2, k1) / 2.0;
        println!("D_{}{} / 2 = {}", k1, k2, d_12);
        println!("D_{}{} / 2 = {}", k2, k1, d_21);
        /********************************************************************/
        let mut k = String::from(k1);
        k.push_str(k2);
        let mut elems = self.elems.clone();
        elems.remove(k1);
        elems.remove(k2);

        let mut keys = self.keys.clone();
        keys.remove(k1);
        keys.remove(k2);
        let rem_keys = keys.clone();

        let mut ret_m = Matrix {
            elems: elems,
            keys: keys,
        };

        for key in rem_keys {
            let val = self.get_d(k1, &key) + self.get_d(k2, &key) - self.get_d(k1, k2);
            ret_m.insert(&k, &key, val / 2.0);
        }

        println!("{}", k);

        ret_m
    }

    fn print_matrix(&self) {
        println!("{:#?}", self.elems);
    }
}

fn upgma(input: &[(&str, &str, f32)]) {
    let mut matrix = Matrix::new();

    for &(c1, c2, val) in input {
        matrix.insert(c1, c2, val);
    }

    matrix.print_matrix();
    let (_, key_1, key_2) = matrix.lowest_d();
    println!("Joining: {} and {}", key_1, key_2);
    matrix = matrix.join(&key_1, &key_2);
}

fn main() {
    let input = vec![/*("A", "A", 0.0)0,*/
                     ("A", "B", 10.0),
                     ("A", "C", 12.0),
                     ("A", "D", 10.0),
                     ("A", "E", 7.0),
                     //  ("B", "B", 0.0),
                     ("B", "C", 4.0),
                     ("B", "D", 4.0),
                     ("B", "E", 13.0),
                     //  ("C", "C", 0.0),
                     ("C", "D", 6.0),
                     ("C", "E", 15.0),
                     //  ("D", "D", 0.0),
                     ("D", "E", 13.0) /* ("E", "E", 0.0)*/];

    upgma(&input);
}
