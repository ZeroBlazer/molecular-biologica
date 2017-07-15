extern crate alignments;

use alignments::align_seqs;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
struct Matrix {
    elems: BTreeMap<char, BTreeMap<char, u32>>,
    keys: BTreeSet<char>,
}

impl Matrix {
    fn new() -> Matrix {
        Matrix {
            elems: BTreeMap::new(),
            keys: BTreeSet::new(),
        }
    }

    fn insert(&mut self, c1: &char, c2: &char, val: u32) {
        let mut ins_map = self.elems.entry(*c1).or_insert_with(BTreeMap::new);
        ins_map.insert(*c2, val);
        self.keys.insert(*c1);
        self.keys.insert(*c2);
    }

    fn len(&self) -> usize {
        self.keys.len()
    }

    fn get_d(&self, c1: &char, c2: &char) -> &u32 {
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

    fn s_calculation(&self, c: &char) -> f64 {
        let mut keys = self.keys.clone();
        if keys.remove(c) {
            let mut sum = 0;
            for &key in &keys {
                match self.elems.get(c) {
                    Some(map) => {
                        match map.get(&key) {
                            Some(val) => sum += *val,
                            None => sum += self.elems[&key][c],
                        }
                    }
                    None => sum += self.elems[&key][c],
                }
            }
            sum as f64 / (keys.len() - 1) as f64
        } else {
            panic!("Key not found in matrix");
        }
    }

    fn m_calculation(&self, c1: &char, c2: &char) -> f64 {
        *self.get_d(c1, c2) as f64 - self.s_calculation(c1) - self.s_calculation(c2)
    }

    fn lowest_m(&self) -> (f64, char, char) {
        let mut iter = self.keys.iter();
        let mut key_1 = match iter.next() {
            Some(val) => val,
            None => panic!("No elements in matrix"),
        };

        let mut lowest = (999.9, *key_1, *key_1);

        for key_2 in iter {
            let m = self.m_calculation(key_1, key_2);
            println!("M_{}{} = {}", key_1, key_2, m);

            if lowest.0 > m {
                lowest = (m, *key_1, *key_2);
            }
            key_1 = key_2;
        }

        lowest
    }

    fn join(&mut self, k1: &char, k2: &char, k: &char) -> Matrix {
        if !(self.keys.contains(k1) || self.keys.contains(k2)) {
            panic!("Keys aren't in matrix");
        }
        /********************************************************************/
        let d_12_2 = *self.get_d(k1, k2) as f64 / 2.0;
        let s_1 = self.s_calculation(k1);
        let s_2 = self.s_calculation(k2);
        println!("S_{}{} = {}", k1, k, d_12_2 + (s_1 - s_2) / 2.0);
        println!("S_{}{} = {}", k2, k, d_12_2 + (s_2 - s_1) / 2.0);
        /********************************************************************/
        let mut elems = self.elems.clone();
        elems.remove(k1);
        elems.remove(k2);
        for (_, val) in &mut elems {
            val.remove(k1);
            val.remove(k2);
        }

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
            ret_m.insert(k, &key, val / 2);
        }

        ret_m
    }

    fn print_matrix(&self) {
        for (key, val) in &self.elems {
            println!("{}: {:?}", key, val);
        }
    }

    fn print_s_calculations(&self) {
        for &key in &self.keys {
            println!("S_{} = {}", key, self.s_calculation(&key));
        }
    }
}

fn distance_score(align_1: &str, align_2: &str) -> f64 {
    let mut x_count = 0.0;
    let mut y_count = 0.0;
    let len = align_1.len();

    for i in 0..len {
        let c1 = align_1.chars().nth(i).unwrap();
        let c2 = align_2.chars().nth(i).unwrap();

        if c1 != '_' && c2 != '_' {         // non-gap positions
            x_count += 1.0;
            if c1 == c2 {                   // identical positions
                y_count += 1.0;
            }
        }
    }

    println!("x: {}, y: {}", x_count, y_count);

    1.0 - (y_count / x_count)
}

fn get_sequences(path: &str) -> Vec<String> {
    // vec!["PPGVKSDCAS".to_string(),
    //      "PADGVKDCAS".to_string(),
    //      "PPDGKSDS".to_string(),
    //      "GADGKDCCS".to_string(),
    //      "GADGKDCAS".to_string()]
    vec!["ACTCATGC".to_string(),
         "AGCCATAC".to_string(),
         "ACGTCCTGT".to_string()]
    // Vec::new()
}

fn mult_seq_alignment(input: &[String]) { 
    let mut matrix = Matrix::new();
    /****************************************************/
    for (i, seq_1) in input.iter().enumerate() {
        let mut j = i;
        for seq_2 in input.iter().skip(i + 1) {
            j += 1;
            if i != j {
                let aligns = align_seqs(seq_1.clone(), seq_2.clone());
                matrix.insert(i, j, distance_score(&aligns[0], &aligns[1]));
                println!("{}-{} > {:#?} - {}", i, j, aligns, distance_score(&aligns[0], &aligns[1]));
            }
        }
    }
    /***************** Neighbor joining *****************/
    let mut letter: u8 = 49; // 85; // 'U'

    while matrix.len() - 2 > 0 {
        matrix.print_matrix();
        println!("");
        matrix.print_s_calculations();
        println!("");
        let (_, key_1, key_2) = matrix.lowest_m();
        println!("\nJoining: {} and {}", key_1, key_2);
        matrix = matrix.join(&key_1, &key_2, &char::from(letter));
        letter += 1;
    }
    matrix.print_matrix();

    println!("Hello, world!");
}

fn main() {
    let input = get_sequences("input/MSA_16507.txt");
    mult_seq_alignment(&input);
}
