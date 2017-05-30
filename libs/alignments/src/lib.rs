#[derive(Debug)]
enum Direction {
    Diagonal,
    Up,
    Left,
    Undef,
}

impl Clone for Direction {
    fn clone(&self) -> Direction {
        match *self {
            Diagonal => Direction::Diagonal,
            Up => Direction::Up,
            Left => Direction::Left,
            Undef => Direction::Undef,
        }
    }
}

#[derive(Debug, Clone)]
struct Score {
    val: i32,
    dir: Direction,
}

impl Score {
    fn new() -> Score {
        Score { val: 0, dir: Undef }
    }
}

// mod tps_tree;

// use tps_tree::{TpsTree, TpsJoint};
use Direction::*;
use std::cmp::min;

fn get_hsp(w1: &String, w2: &String, match_scr: i32, mism_scr: i32, gap_scr: i32) -> i32 {
    let mut hsp = 0;

    let mut chars2 = w2.chars();
    for c1 in w1.chars() {
        let c2 = chars2.next().unwrap();
        hsp += if c1 == c2 {
            match_scr
        } else if c1 == '_' || c2 == '_' {
            gap_scr
        } else {
            mism_scr
        }
    }

    hsp
}

fn print_matrix(seq_vec1: &Vec<String>, seq_vec2: &Vec<String>, mtrx: &Vec<Vec<Score>>) {
    let len_2 = seq_vec2.len();
    for sec in seq_vec1.iter() {
        for _ in 0..len_2 {
            print!("\t");
        }
        for chr in sec.chars() {
            print!("{}\t", chr);
        }
        println!("");
    }

    for i in 0..seq_vec2[0].len() {
        for sec in seq_vec2.iter() {
            print!("{}\t", sec.chars().nth(i).unwrap());
        }
        for elem in mtrx[i].iter() {
            print!("{}\t", elem.val);
        }
        println!("");
    }
}

fn align_seqs(mut seq_vec1: Vec<String>, mut seq_vec2: Vec<String>) {
    /********* Initialize sequences **********/
    for sec in seq_vec1.iter_mut() {
        *sec = String::from("_") + sec.as_ref();
    }

    for sec in seq_vec2.iter_mut() {
        *sec = String::from("_") + sec.as_ref();
    }
    /********* score function **********/
    fn score(a: char, b: char) -> i32 {
        if a == b {
            0
        } else if a == '_' || b == '_' {
            2
        } else {
            3
        }
    }
    /********* Initialize matrix **********/
    let len_1 = seq_vec1[0].len();
    let len_2 = seq_vec2[0].len();

    let mut matrix: Vec<Vec<Score>> = vec![vec![Score::new(); len_1]; len_2];

    let mut val = 0;
    for j in 1..len_1 {
        for sec in seq_vec1.iter() {
            val += score(seq_vec2.chars().nth(0).unwrap(), sec.chars().nth(j).unwrap());
        }
        matrix[0][j].val = val;
        matrix[0][j].dir = Left;
    }

    val = 0;
    for i in 1..len_2 {
        for sec in seq_vec1.iter() {
            val += score(seq_vec2.chars().nth(i).unwrap(), sec.chars().nth(0).unwrap());
        }
        matrix[i][0].val = val;
        matrix[i][0].dir = Up;
    }
    /**************************************/

    let mut diag_val;
    let mut left_val;
    let mut uppr_val;

    for i in 1..len_2 {
        for j in 1..len_1 {
            diag_val = {
                let mut sum = 0;
                for sec in seq_vec1.iter() {
                    sum += score(seq_vec2.chars().nth(i).unwrap(), sec.chars().nth(j).unwrap());
                }
                sum += matrix[i - 1][j - 1].val;
                sum
            };

            left_val = {
                let mut sum = 0;
                for _ in seq_vec1.iter() {
                    sum += score(seq_vec2.chars().nth(i).unwrap(), '_');
                }
                sum += matrix[i][j - 1].val;
                sum
            };

            uppr_val = {
                let mut sum = 0;
                for sec in seq_vec1.iter() {
                    sum += score('_', sec.chars().nth(j).unwrap());
                }
                sum += matrix[i - 1][j].val;
                sum
            };

            matrix[i][j].val = min(diag_val, min(left_val, uppr_val));
            matrix[i][j].dir = if matrix[i][j].val == diag_val {
                Diagonal
            } else if matrix[i][j].val == left_val {
                Left
            } else if matrix[i][j].val == uppr_val {
                Up
            } else {
                Undef
            };
        }
    }

    print_matrix(&seq_vec1, &seq_vec2, &matrix);

    /********* Restore sequences **********/
    seq_vec2.remove(0);

    for sec in seq_vec1.iter_mut() {
        sec.remove(0);
    }
    /**************************************/

    let mut i = len_2 - 1;
    let mut j = len_1 - 1;
    let mut dir: Direction;
    while i != 0 && j != 0 {
        dir = matrix[i][j].dir.clone();

        match dir {
            Diagonal => {
                i -= 1;
                j -= 1;
            }
            Left => {
                let term = seq_vec2.split_off(j);
                seq_vec2 = format!("{}_{}", seq_vec2, term);
                j -= 1;
            }
            Up => {
                for mut sec in seq_vec1.iter_mut() {
                    let term = sec.split_off(i);
                    *sec = format!("{}_{}", sec, term);
                }
                i -= 1;
            }
            Undef => {
                break;
            }
        }
    }

    println!("\n> {}", seq_vec2);
    for sec in seq_vec1.iter() {
        println!("> {}", sec)
    }
}

fn pair_sum(sec_1: &str, sec_2: &str, match_scr: i32, mism_scr: i32, gap_scr: i32) -> f32 {
    let len_1 = sec_1.len();
    let len_2 = sec_2.len();
    let diff: i32 = len_1 as i32 - len_2 as i32;

    let sec_1s;
    let sec_2s;

    if diff > 0 {
        sec_1s = String::from(sec_1);
        sec_2s = format!("{}{}", sec_2, "_".repeat(diff as usize));
    } else {
        sec_1s = format!("{}{}", sec_1, "_".repeat(diff.abs() as usize));
        sec_2s = String::from(sec_2);
    }
    // println!("A: {}\nB: {}\n> {}\n", sec_1s, sec_2s, get_hsp(&sec_1s, &sec_2s, match_scr, mism_scr, gap_scr) as f32 / sec_1s.len() as f32);
    get_hsp(&sec_1s, &sec_2s, match_scr, mism_scr, gap_scr) as f32 / sec_1s.len() as f32
}

pub fn tps_alignment(seqs: Vec<&str>) {
    let len = seqs.len();
    let mut my_vec = Vec::new();

    for i in 0..len {
        let mut j = i + 1;
        while j < len {
            my_vec.push((pair_sum(seqs[i], seqs[j], 2, 1, 1), i, j));
            j += 1;
        }
    }

    my_vec.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    println!("{:?}", my_vec);

    // let tree = TpsTree::new();
    // let joint1 = TpsJoint::from_strings(seqs[my_vec[0].1], seqs[my_vec[0].2]);
    // println!("{:?}", joint1);
    // let joint2 = TpsJoint::from_strings(seqs[my_vec[2].1], seqs[my_vec[2].2]);
    // println!("{:?}", joint2);
}