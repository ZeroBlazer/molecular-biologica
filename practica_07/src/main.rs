use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

//Intento de push

fn process_file(p: &str) -> ((u32, u32, u32, u32, u32, u32), [[f64; 4]; 4]) {
    let path = Path::new(p);
    let mut file = File::open(&path).expect("Unable to open");

    let mut s = String::new();
    file.read_to_string(&mut s).expect("not able to read");

    let mut ctr = (0, 0, 0, 0, 0, s.len() as u32);
    let mut is_beginning = true;
    let mut trans_mtrx: [[f64; 4]; 4] = [[0.0; 4]; 4];
    let mut prev_pos = 0;
    let mut curr_pos = 0;

    for c in s.chars() {
        match c {
            'A' => {
                ctr.0 += 1;
                curr_pos = 0;
            }
            'C' => {
                ctr.1 += 1;
                curr_pos = 1;
            }
            'G' => {
                ctr.2 += 1;
                curr_pos = 2;
            }
            'T' => {
                ctr.3 += 1;
                curr_pos = 3;
            }
            _ => println!("Not a valid letter"),
        }

        if is_beginning {
            ctr.4 = curr_pos as u32;
            is_beginning = false;
        } else {
            trans_mtrx[prev_pos][curr_pos] += 1.0;
            prev_pos = curr_pos;
        }
    }

    match s.pop().unwrap() {
        'A' => ctr.0 -= 1,
        'C' => ctr.1 -= 1,
        'G' => ctr.2 -= 1,
        'T' => ctr.3 -= 1,
        _ => println!("Not a valid letter"),
    }

    (ctr, trans_mtrx)
}

fn calc_trans_mtrx(nums: [u32; 4], trans_mtrx: &mut [[f64; 4]; 4]) {
    for i in 0..4 {
        for j in 0..4 {
            trans_mtrx[i][j] /= nums[i] as f64;
        }
    }
}

fn prob(first: u32, length: u32, prob_1: f64, prob_2: f64, prob_3: f64) -> f64 {
    (first as f64 / length as f64) * prob_1 * prob_2 * prob_3
}

fn main() {
    // let path = ;
    let (tuple, mut trans_mtrx) = process_file("practica_07.txt");
    calc_trans_mtrx([tuple.0, tuple.1, tuple.2, tuple.3], &mut trans_mtrx);

    let mut file = File::create("EPuma_Practica_07B.txt").expect("Couldn't open write file");
    // println!("{}\n",
    //          prob(tuple.0,
    //               tuple.5,
    //               trans_mtrx[0][1],
    //               trans_mtrx[1][2],
    //               trans_mtrx[2][3]));
    write!(file,
           "P(ATCA): {}\n",
           prob(tuple.0,
                tuple.5,
                trans_mtrx[0][3],
                trans_mtrx[3][1],
                trans_mtrx[1][0]))
            .expect("Couldn't write in file");
    write!(file,
           "P(CAGG): {}\n",
           prob(tuple.1 + 1,
                tuple.5,
                trans_mtrx[1][0],
                trans_mtrx[0][2],
                trans_mtrx[2][2]))
            .expect("Couldn't write in file");
    write!(file,
           "P(GAAT): {}\n",
           prob(tuple.2,
                tuple.5,
                trans_mtrx[2][0],
                trans_mtrx[0][0],
                trans_mtrx[0][3]))
            .expect("Couldn't write in file");
    write!(file,
           "P(TGAC): {}\n",
           prob(tuple.3,
                tuple.5,
                trans_mtrx[3][2],
                trans_mtrx[2][0],
                trans_mtrx[0][1]))
            .expect("Couldn't write in file");
}
