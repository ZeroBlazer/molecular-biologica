use std::collections::BTreeMap;

#[derive(Debug)]
struct Matrix {
    elems: Vec<Vec<f32>>,
    keys: BTreeMap<String, usize>,
}

impl Matrix {
    fn new(keys: &[&str]) -> Matrix {
        let e_matrix = vec![vec![0.0; keys.len()]; keys.len()];
        let mut k_indexes = BTreeMap::new();

        for key in keys {
            k_indexes.insert(key.to_string(), 0);
        }

        for (i, (_, index)) in k_indexes.iter_mut().enumerate() {
            *index = i;
        }

        Matrix {
            elems: e_matrix,
            keys: k_indexes,
        }
    }

    fn insert(&mut self, c1: &str, c2: &str, val: f32) {
        let i1 = self.keys[c1];
        let i2 = self.keys[c2];

        if i1 < i2 {
            self.elems[i1][i2] = val;
        } else {
            self.elems[i2][i1] = val;
        }
    }

    fn len(&self) -> usize {
        self.keys.len()
    }

    fn get_d(&self, c1: &str, c2: &str) -> &f32 {
        let i1 = self.keys[c1];
        let i2 = self.keys[c2];

        if i1 < i2 {
            &self.elems[i1][i2]
        } else {
            &self.elems[i2][i1]
        }
    }

    fn lowest_d(&self) -> (f32, String, String) {
        let mut lowest = (99999.9, "".to_string(), "".to_string());

        for key_1 in self.keys.keys() {
            for key_2 in self.keys.keys() {
                if key_1 != key_2 {
                    let &d = self.get_d(key_1, key_2);
                    if lowest.0 > d {
                        lowest = (d, key_1.to_string(), key_2.to_string());
                    }
                }
            }
        }

        lowest
    }

    fn join(&mut self, k1: String, k2: String) -> Matrix {
        let mut st_keys = self.keys.clone();
        st_keys.remove(&k1);
        st_keys.remove(&k2);

        let clone_keys = st_keys.clone();

        let mut new_key = if k1 < k2 { k1.clone() } else { k2.clone() };
        new_key.push_str(if k1 < k2 { k2.as_str() } else { k1.as_str() });

        let new_ins_key = new_key.clone();

        st_keys.insert(new_key, 0);
        let ret_keys: Vec<&str> = st_keys.iter().map(|a| a.0.as_ref()).collect();
        println!("{:?}", ret_keys);
        let mut ret_m = Matrix::new(&ret_keys);

        let clone_keys: Vec<&str> = clone_keys.iter().map(|a| a.0.as_ref()).collect();
        for key_1 in &clone_keys {
            for key_2 in &clone_keys {
                ret_m.insert(key_1, key_2, *self.get_d(key_1, key_2));
                // println!("{} <> {}", key_1, key_2);
            }

            let val_1 = *self.get_d(k1.as_ref(), key_1);
            let val_2 = *self.get_d(k2.as_ref(), key_1);
            ret_m.insert(new_ins_key.as_ref(), key_1, (val_1 + val_2) / 2.0);
        }

        ret_m
    }

    fn print_matrix(&self) {
        for row in &self.elems {
            println!("{:?}", row);
        }
    }
}

fn upgma(keys: &[&str], input: &[(&str, &str, f32)]) {
    let mut matrix = Matrix::new(keys);

    for &(c1, c2, val) in input {
        matrix.insert(c1, c2, val);
    }

    matrix.print_matrix();

    while matrix.len() > 2 {
        let (_, key_1, key_2) = matrix.lowest_d();
        println!("\nJoining: {} and {}", key_1, key_2);
        matrix = matrix.join(key_1, key_2);
        matrix.print_matrix();
    }
}

fn main() {
    let keys = vec!["A", "B", "C", "D", "E"];
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

    upgma(&keys, &input);
}
