#[derive(Debug)]
enum Aminoacid {
    A,
    G,
    C,
    T,
    Undef,
}

static SCORES: [[i32; 4]; 4] = [[10, -1, -3, -4],
                                [-1, 7, -5, -3],
                                [-3, -5, 9, 0],
                                [-4, -3, 0, 8]];

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
struct Score<T> {
    val: T,
    dir: Direction,
}

impl Score<i32> {
    fn new() -> Score<i32> {
        Score { val: 0, dir: Undef }
    }
}

impl Score<f32> {
    fn new() -> Score<f32> {
        Score {
            val: 0.0,
            dir: Undef,
        }
    }
}

mod tps_tree;

use tps_tree::{TpsTree, TpsJoint};
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

fn print_matrix<T>(seq_vec1: &Vec<String>, seq_vec2: &Vec<String>, mtrx: &Vec<Vec<Score<T>>>)
    where T: std::fmt::Display
{
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
    println!("");
}

fn align_seqs(mut sec_1: String, mut sec_2: String) -> Vec<String> {
    sec_1 = String::from("_") + sec_1.as_ref();
    sec_2 = String::from("_") + sec_2.as_ref();
    /********* Initialize matrix **********/
    let len_1 = sec_1.len();
    let len_2 = sec_2.len();

    let mut matrix: Vec<Vec<Score<i32>>> = vec![vec![Score::<i32>::new(); len_1]; len_2];

    let mut val = -5;
    for i in 1..len_1 {
        matrix[0][i].val = val;
        matrix[0][i].dir = Left;
        val -= 5;
    }

    val = -5;
    for i in 1..len_2 {
        matrix[i][0].val = val;
        matrix[i][0].dir = Up;
        val -= 5;
    }
    /**************************************/
    fn get_score(a: char, b: char) -> i32 {
        let a_pos = match a {
            'A' => Aminoacid::A,
            'G' => Aminoacid::G,
            'C' => Aminoacid::C,
            'T' => Aminoacid::T,
            _ => Aminoacid::Undef,
        };

        let b_pos = match b {
            'A' => Aminoacid::A,
            'G' => Aminoacid::G,
            'C' => Aminoacid::C,
            'T' => Aminoacid::T,
            _ => Aminoacid::Undef,
        };

        SCORES[b_pos as usize][a_pos as usize]
    }
    /**************************************/
    let mut diag_val;
    let mut left_val;
    let mut uppr_val;

    for i in 1..len_2 {
        for j in 1..len_1 {
            diag_val = get_score(sec_1.chars().nth(j).unwrap(), sec_2.chars().nth(i).unwrap()) +
                       matrix[i - 1][j - 1].val;
            left_val = -5 + matrix[i][j - 1].val;
            uppr_val = -5 + matrix[i - 1][j].val;

            if diag_val >= left_val {
                if diag_val >= uppr_val {
                    matrix[i][j].val = diag_val;
                    matrix[i][j].dir = Diagonal;
                } else {
                    matrix[i][j].val = uppr_val;
                    matrix[i][j].dir = Up;
                }
            } else {
                if left_val >= uppr_val {
                    matrix[i][j].val = left_val;
                    matrix[i][j].dir = Left;
                } else {
                    matrix[i][j].val = uppr_val;
                    matrix[i][j].dir = Up;
                }
            }
        }
    }

    print_matrix(&vec![sec_1.clone()], &vec![sec_2.clone()], &matrix);
    /********* Restore sequences **********/
    sec_1.remove(0);
    sec_2.remove(0);
    /************** Alignment **************/
    let mut i = len_2 - 1;
    let mut j = len_1 - 1;
    let mut dir: Direction;
    while i != 0 || j != 0 {
        dir = matrix[i][j].dir.clone();
        match dir {
            Diagonal => {
                i -= 1;
                j -= 1;
            }
            Left => {
                let term = sec_2.split_off(i);
                sec_2 = format!("{}_{}", sec_2, term);
                j -= 1;
            }
            Up => {
                let term = sec_1.split_off(j);
                sec_1 = format!("{}_{}", sec_1, term);
                i -= 1;
            }
            Undef => {
                break;
            }
        }
    }
    /**************************************/
    // println!("> {}", sec_1);
    // println!("> {}", sec_2);
    vec![sec_1, sec_2]
}

fn align_seqs_seq(mut seq_vec: Vec<String>, mut seq: String) -> Vec<String> {
    /********* Initialize sequences **********/
    seq = String::from("_") + seq.as_ref();

    for sec in seq_vec.iter_mut() {
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
    let len_1 = seq_vec[0].len();
    let len_2 = seq.len();

    let mut matrix: Vec<Vec<Score<i32>>> = vec![vec![Score::<i32>::new(); len_1]; len_2];

    let mut val = 0;
    for j in 1..len_1 {
        for sec in seq_vec.iter() {
            val += score(seq.chars().nth(0).unwrap(), sec.chars().nth(j).unwrap());
        }
        matrix[0][j].val = val;
        matrix[0][j].dir = Left;
    }

    val = 0;
    for i in 1..len_2 {
        for sec in seq_vec.iter() {
            val += score(seq.chars().nth(i).unwrap(), sec.chars().nth(0).unwrap());
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
                for sec in seq_vec.iter() {
                    sum += score(seq.chars().nth(i).unwrap(), sec.chars().nth(j).unwrap());
                }
                sum += matrix[i - 1][j - 1].val;
                sum
            };

            left_val = {
                let mut sum = 0;
                for _ in seq_vec.iter() {
                    sum += score(seq.chars().nth(i).unwrap(), '_');
                }
                sum += matrix[i][j - 1].val;
                sum
            };

            uppr_val = {
                let mut sum = 0;
                for sec in seq_vec.iter() {
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

    print_matrix(&seq_vec, &vec![seq.clone()], &matrix);

    /********* Restore sequences **********/
    seq.remove(0);

    for sec in seq_vec.iter_mut() {
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
                let term = seq.split_off(j);
                seq = format!("{}_{}", seq, term);
                j -= 1;
            }
            Up => {
                for mut sec in seq_vec.iter_mut() {
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
    /**************************************/
    // println!("\n> {}", seq);
    // for sec in seq_vec.iter() {
    //     println!("> {}", sec)
    // }
    seq_vec.push(seq);
    seq_vec
}

fn align_alignments(mut seq_vec1: Vec<String>, mut seq_vec2: Vec<String>) -> Vec<String> {
    /********* Initialize sequences **********/
    for sec in seq_vec1.iter_mut() {
        *sec = String::from("_") + sec.as_ref();
    }
    for sec in seq_vec2.iter_mut() {
        *sec = String::from("_") + sec.as_ref();
    }
    /********* score function **********/
    fn score(a: char, b: char) -> f32 {
        if a == '_' || b == '_' {
            0.0
        } else if a == b {
            2.0
        } else {
            1.0
        }
    }
    /********* Initialize matrix **********/
    let len_1 = seq_vec1[0].len();
    let len_2 = seq_vec2[0].len();
    let n_seqs = (seq_vec1.len() + seq_vec2.len()) as f32;

    let mut matrix: Vec<Vec<Score<f32>>> = vec![vec![Score::<f32>::new(); len_1]; len_2];

    for j in 1..len_1 {
        matrix[0][j].dir = Left;
    }
    for i in 1..len_2 {
        matrix[i][0].dir = Up;
    }
    /**************************************/
    let mut diag_val;
    let mut left_val;
    let mut uppr_val;

    for i in 1..len_2 {
        for j in 1..len_1 {
            // println!("({}, {})", j, i);
            diag_val = {
                let mut sum = 0.0;
                for sec1 in seq_vec1.iter() {
                    for sec2 in seq_vec2.iter() {
                        sum += score(sec1.chars().nth(j).unwrap(), sec2.chars().nth(i).unwrap());
                        // println!("{}:{}", sec1.chars().nth(j).unwrap(), sec2.chars().nth(i).unwrap());
                    }
                }
                sum /= n_seqs;
                sum += matrix[i - 1][j - 1].val;
                // println!("D: {}\n", sum);
                sum
            };

            left_val = {
                let mut sum = 0.0;
                for sec1 in seq_vec1.iter() {
                    for sec2 in seq_vec2.iter() {
                        sum += score(sec1.chars().nth(j - 1).unwrap(),
                                     sec2.chars().nth(i).unwrap());
                        // println!("{}:{}", sec1.chars().nth(j-1).unwrap(), sec2.chars().nth(i).unwrap());
                    }
                }
                sum /= n_seqs;
                // sum += matrix[i][j - 1].val;
                // println!("L: {}\n", sum);
                sum
            };

            uppr_val = {
                let mut sum = 0.0;
                for sec1 in seq_vec1.iter() {
                    for sec2 in seq_vec2.iter() {
                        sum += score(sec1.chars().nth(j).unwrap(),
                                     sec2.chars().nth(i - 1).unwrap());
                        // println!("{}:{}", sec1.chars().nth(j).unwrap(), sec2.chars().nth(i-1).unwrap());
                    }
                }
                sum /= n_seqs;
                // sum += matrix[i - 1][j].val;
                // println!("U: {}\n", sum);
                sum
            };

            if diag_val >= left_val {
                if diag_val >= uppr_val {
                    matrix[i][j].val = diag_val;
                    matrix[i][j].dir = Diagonal;
                } else {
                    matrix[i][j].val = uppr_val;
                    matrix[i][j].dir = Up;
                }
            } else {
                if left_val >= uppr_val {
                    matrix[i][j].val = left_val;
                    matrix[i][j].dir = Left;
                } else {
                    matrix[i][j].val = uppr_val;
                    matrix[i][j].dir = Up;
                }
            }
        }
    }

    print_matrix(&seq_vec1, &seq_vec2, &matrix);

    /********* Restore sequences **********/
    for sec in seq_vec1.iter_mut() {
        sec.remove(0);
    }
    for sec in seq_vec2.iter_mut() {
        sec.remove(0);
    }
    /**************************************/
    let mut i = len_2 - 1;
    let mut j = len_1 - 1;
    let mut dir: Direction;
    while i != 0 || j != 0 {
        dir = matrix[i][j].dir.clone();
        match dir {
            Diagonal => {
                i -= 1;
                j -= 1;
            }
            Left => {
                for mut sec in seq_vec2.iter_mut() {
                    let term = sec.split_off(i);
                    *sec = format!("{}_{}", sec, term);
                }
                j -= 1;
            }
            Up => {
                for mut sec in seq_vec1.iter_mut() {
                    let term = sec.split_off(j);
                    *sec = format!("{}_{}", sec, term);
                }
                i -= 1;
            }
            Undef => {
                break;
            }
        }
    }
    /**************************************/
    // for sec in seq_vec1.iter() {
    //     println!("> {}", sec)
    // }
    // for sec in seq_vec2.iter() {
    //     println!("> {}", sec)
    // }
    seq_vec1.append(&mut seq_vec2);
    seq_vec1
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

    // align_alignments(vec![String::from("AAAC"), String::from("_GAC")],
    //                  vec![String::from("AGC"), String::from("ACC")]);

    // align_seqs_seq(vec![String::from("ACTCAT"), String::from("AGTCAT")],
    //                String::from("ACGTCCT"));

    // align_seqs(String::from("ACCGTCTT"), String::from("CGTCTT"));
    // align_seqs(String::from("TGTAAC"), String::from("TGTAC"));
    // align_seqs(String::from("ATGTC"), String::from("ATGTGGC"));

    // align_alignments(vec![String::from("TGTAAC")],
    //                  vec![String::from("TGTAC")]);

    let mut tree = TpsTree::new();
    let joint1 = TpsJoint::from_strings(seqs[my_vec[0].1], seqs[my_vec[0].2]);
    // println!("{:?}", joint1);
    tree.insert_joint(joint1);
    let joint2 = TpsJoint::from_strings(seqs[my_vec[2].1], seqs[my_vec[2].2]);
    // println!("{:?}", joint2);
    tree.insert_joint(joint2);
    println!("{:?}", tree);
}