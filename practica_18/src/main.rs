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

fn get_hsp(w1: &String, w2: &String, match_scr: f64, mism_scr: f64, gap_scr: f64) -> f64 {
    let mut hsp = 0.0;

    let mut chars2 = w2.chars();
    for c1 in w1.chars() {
        let c2 = chars2.next().unwrap();
        hsp += if c1 == '_' || c2 == '_' {
            gap_scr
        } else if c1 == c2 {
            match_scr 
        } else {
            mism_scr
        } as f64;
    }

    hsp
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

#[derive(Debug)]
struct Ant {
    memory: Vec<usize>,
}

impl Ant {
    fn new() -> Ant {
        Ant {
            // memory: Vec::new()
            // memory: vec![0, 3, 3, 4, 1, 5, 5, 5, 3, 6, 6, 6, 4, 8, 0, 0],
            memory: vec![2, 1, 1, 3, 4, 3, 3, 4, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 0, 0],
        }
    }

    fn print(&self) {
        println!("{:?}", self.memory);
    }
}

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

    fn scr_alignment(&self, ant: &Ant) -> f64 {
        let n_seqs = self.seqs.len();
        let n_arr = ant.memory.len() / n_seqs;
        let mut indx_iter = ant.memory.iter();
        let mut alignments = vec![String::new(); n_seqs];
        let mut indx_aux = vec![0; n_seqs];
        let mut offset = vec![0; n_seqs];
        let mut seqs = self.seqs.clone();

        for x in 0..n_arr {
            let mut greatest_offset = 0;
            for i in 0..n_seqs {
                let indx = *indx_iter.next().unwrap();
                
                offset[i] = if indx > 0 || x == 0 {
                                indx - indx_aux[i]
                            } else {
                                1
                            };
                
                if greatest_offset < offset[i] {
                    greatest_offset = offset[i];
                }

                indx_aux[i] = indx;
            }

            for i in 0..n_seqs {
                // print!("{} - {} -> ", greatest_offset, offset[i]);
                let n_dashes = greatest_offset - offset[i];
                // print!("\"-\" = {}: ", n_dashes);

                let slice = if seqs[i].len() > 0 {
                    let remnant = seqs[i].split_off(offset[i]);
                    let slice = seqs[i].clone();
                    seqs[i] = remnant;
                    slice
                } else {
                    seqs[i].clone()
                };
                alignments[i].push_str(slice.as_str());

                for _ in 0..n_dashes {
                    alignments[i].push_str("_");
                }
                // alignments[i].push_str(".");
            }

        }
        
        let mut lngst_seq = 0;
        for i in 0..n_seqs {
            if lngst_seq < seqs[i].len() {
                lngst_seq = seqs[i].len();
            }
            alignments[i].push_str(seqs[i].as_str());
        }

        for i in 0..n_seqs {
            for _ in 0..(lngst_seq - seqs[i].len()) {
                alignments[i].push_str("_");
            }
        }

        for seq in &alignments {
            println!("{}", seq);
        }

        let mut score = 0.0;
        // for i in 0..n_seqs {
        //     for j in i + 1..n_seqs {
        //         score += get_hsp(alignments[i], alignments[j], match_scr, mism_scr, gap_scr);
        //     }
        // }

        println!("Suma Pares: {}", score);
        score
    }

    fn iterate(&mut self) {
        let ant = Ant::new();
        ant.print();
        let score = self.scr_alignment(&ant);
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