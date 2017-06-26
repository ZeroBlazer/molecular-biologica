// extern crate rand;

// use rand::{thread_rng, Rng};

fn score(c1: char, c2: char, scr_match: f64, scr_miss: f64, scr_gap: f64) -> f64 {
    if c1 == '_' || c2 == '_' {
        scr_gap
    } else if c1 == c2 {
        scr_match
    } else {
        scr_miss
    }
}

fn seqs_distance(seq_1: &str,
                 seq_2: &str,
                 scr_match: f64,
                 scr_miss: f64,
                 scr_gap: f64)
                 -> Vec<Vec<f64>> {
    println!("{} vs {}", seq_1, seq_2);
    let len_1 = seq_1.len();
    let len_2 = seq_2.len();
    let mut dist_vec = vec![Vec::new(); seq_2.len()];

    for j in 0..len_2 {
        for i in 0..len_1 {
            dist_vec[j].push(score(seq_1.chars().nth(i).unwrap(),
                                   seq_2.chars().nth(j).unwrap(),
                                   scr_match,
                                   scr_miss,
                                   scr_gap));
        }
        println!("{:?}", dist_vec[j]);
    }
    println!("");

    dist_vec
}

fn seqs_visibility(seq_1: &str,
                   seq_2: &str,
                   scr_match: f64,
                   scr_miss: f64,
                   scr_gap: f64)
                   -> Vec<Vec<f64>> {
    println!("{} vs {}", seq_1, seq_2);
    let len_1 = seq_1.len();
    let len_2 = seq_2.len();
    let mut visib_vec = vec![Vec::new(); seq_2.len()];

    for j in 0..len_2 {
        for i in 0..len_1 {
            visib_vec[j].push(1.0 /
                              score(seq_1.chars().nth(i).unwrap(),
                                    seq_2.chars().nth(j).unwrap(),
                                    scr_match,
                                    scr_miss,
                                    scr_gap));
        }
        println!("{:?}", visib_vec[j]);
    }
    println!("");

    visib_vec
}

fn seqs_pheromone(seq_1: &str, seq_2: &str, init_phero: f64) -> Vec<Vec<f64>> {
    println!("{} vs {}", seq_1, seq_2);
    let len_1 = seq_1.len();
    let len_2 = seq_2.len();
    let mut phero_vec = vec![Vec::new(); seq_2.len()];

    for j in 0..len_2 {
        for _ in 0..len_1 {
            phero_vec[j].push(init_phero);
        }
        println!("{:?}", phero_vec[j]);
    }
    println!("");

    phero_vec
}

fn seqs_deviation(seq_1: &str, seq_2: &str) -> Vec<Vec<f64>> {
    println!("{} vs {}", seq_1, seq_2);
    let mut deviat_vec = vec![Vec::new(); seq_2.len()];

    for j in 0..seq_2.len() {
        for i in 0..seq_1.len() {
            deviat_vec[j].push(1.0 / ((i as f64 - j as f64).abs() + 1.0));
        }
        println!("{:?}", deviat_vec[j]);
    }
    println!("");

    deviat_vec
}

// #[derive(Debug)]
// struct Ant {
//     memory: Vec<usize>,
// }

// impl Ant {
//     fn new() -> Ant {
//         Ant { memory: Vec::new() }
//     }
// }

#[derive(Debug)]
struct AntColony {
    rho: f64,
    q: i32,
    q_0: f64,
    phi: f64,
    init_phero: f64,
    num_ant: usize,
    num_it: usize,
    seqs: Vec<String>,
    distance: Vec<Vec<Vec<f64>>>,
    visibility: Vec<Vec<Vec<f64>>>,
    pheromone: Vec<Vec<Vec<f64>>>,
    deviation: Vec<Vec<Vec<f64>>>,
}

impl AntColony {
    fn new(rho: f64,
           q: i32,
           q_0: f64,
           phi: f64,
           init_phero: f64,
           num_ant: usize,
           num_it: usize)
           -> AntColony {
        AntColony {
            rho: rho,
            q: q,
            q_0: q_0,
            phi: phi,
            init_phero: init_phero,
            num_ant: num_ant,
            num_it: num_it,
            seqs: Vec::new(),
            distance: Vec::new(),
            visibility: Vec::new(),
            pheromone: Vec::new(),
            deviation: Vec::new(),
        }
    }

    fn set_sequences(&mut self, input: &[&str], scr_match: f64, scr_miss: f64, scr_gap: f64) {
        self.seqs = input.iter().map(|x| x.to_string()).collect();
        let len = self.seqs.len();
        /****************DISTANCIAS*****************/
        for i in 0..len {
            let j = i + 1;
            if j == len {
                break;
            }
            println!("Distancias {} {}", i, j);
            self.distance
                .push(seqs_distance(input[i], input[j], scr_match, scr_miss, scr_gap));
        }
        /****************VISIBILIDAD*****************/
        for i in 0..len {
            let j = i + 1;
            if j == len {
                break;
            }
            println!("Visibilidad {} {}", i, j);
            self.visibility
                .push(seqs_visibility(input[i], input[j], scr_match, scr_miss, scr_gap));
        }
        /****************FEROMONA*****************/
        for i in 0..len {
            let j = i + 1;
            if j == len {
                break;
            }
            println!("Feromona {} {}", i, j);
            self.visibility
                .push(seqs_pheromone(input[i], input[j], self.init_phero));
        }
        /****************DEVIATION*****************/
        for i in 0..len {
            let j = i + 1;
            if j == len {
                break;
            }
            println!("Desviación {} {}", i, j);
            self.visibility.push(seqs_deviation(input[i], input[j]));
        }
    }

    fn iterate(&mut self) {
        println!("Hello world!");
    }
}

fn main() {
    let mut colony = AntColony::new(0.99, 1, 0.7, 0.05, 1.0, 3, 100);
    colony.set_sequences(&["AGTCATTAAT", "CCAATTAGG", "ACCATTC", "GTTCAAGG"],
                         1.0,
                         4.0,
                         5.0);
    colony.iterate();
    // let input = vec!["AGTCATTAAT", "CCAATTAGG", "ACCATTC", "GTTCAAGG"];
    // colony.ant_iteration(&input);
}