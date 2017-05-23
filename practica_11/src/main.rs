#[derive(Debug)]
enum Direction {
    Diagonal,
    Up,
    Left,
}

use Direction::*;
use std::cmp::max;

fn get_score(a: char, b: char) -> i32 {
    if a == b { 1 } else { -1 }
}

fn print_matrix(mtrx: &Vec<Vec<(i32, String, String)>>) {
    for row in mtrx.iter() {
        for elem in row.iter() {
            print!("{}\t", elem.0);
        }
        println!("");
        // println!("{:?}", row);
    }
}

fn print_matrix2(sec_1: &str, sec_2: &str, mtrx: &Vec<Vec<i32>>) {
    print!("\t\t");
    for chr in sec_1.chars() {
        print!(" {}\t", chr);
    }
    print!("\n\t");

    for elem in mtrx[0].iter() {
        print!("{}\t", elem);
    }
    println!("");

    let mut i: usize = 1;
    for chr in sec_2.chars() {
        print!("{}\t", chr);
        for elem in mtrx[i].iter() {
            print!("{}\t", elem);
        }
        println!("");
        i += 1;
    }
}

fn get_hsp(w1: &String, w2: &String, match_scr: i32, mism_scr: i32, gap_scr: i32) -> i32 {
    let mut hsp = 0;

    let mut chars2 = w2.chars();
    for c1 in w1.chars() {
        let c2 = chars2.next().unwrap();
        hsp += if c1 == c2 {
            match_scr
        } else if c1 == '_' {
            gap_scr
        } else if c2 == '_' {
            gap_scr
        } else {
            mism_scr
        }
    }

    hsp
}

fn align_secuence(sec_1: &str, sec_2: &str) -> (i32, String, String) {
    let len_1 = sec_1.len() + 1;
    let len_2 = sec_2.len() + 1;

    /********* Initialize matrix **********/
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; len_1]; len_2];

    let mut init_val = -5;
    for i in 1..len_1 {
        matrix[0][i] = init_val;
        init_val -= 5;
    }

    init_val = -5;
    for i in 1..len_2 {
        matrix[i][0] = init_val;
        init_val -= 5;
    }
    /******************************print_matrix(sec_1, sec_2, &matrix);********/

    let mut diag_val;
    let mut left_val;
    let mut uppr_val;

    let mut i = 1;
    let mut j;
    for c2 in sec_2.chars() {
        j = 1;
        for c1 in sec_1.chars() {
            let score = get_score(c2, c1);
            diag_val = score + matrix[i - 1][j - 1];
            left_val = -2 + matrix[i][j - 1];
            uppr_val = -2 + matrix[i - 1][j];
            matrix[i][j] = max(diag_val, max(left_val, uppr_val));
            j += 1;
        }
        i += 1;
    }

    println!("");
    print_matrix2(sec_1, sec_2, &matrix);

    /************** Alignment **************/
    let mut align1 = String::new();
    let mut align2 = String::new();

    i = sec_2.len();
    j = sec_1.len();

    let mut direct = Diagonal;
    let mut greater;
    loop {
        if i == 0 || j == 0 {
            break;
        }

        diag_val = matrix[i - 1][j - 1];
        left_val = matrix[i][j - 1];
        uppr_val = matrix[i - 1][j];
        greater = max(diag_val, max(left_val, uppr_val));

        match direct {
            Diagonal => {
                align1.push(sec_1.as_bytes()[j - 1] as char);
                align2.push(sec_2.as_bytes()[i - 1] as char);
            }
            Up => {
                align1.push('_');
                align2.push(sec_2.as_bytes()[i - 1] as char);
            }
            Left => {
                align1.push(sec_1.as_bytes()[j - 1] as char);
                align2.push('_');
            }
        }

        if greater == diag_val {
            direct = Diagonal;
            i -= 1;
            j -= 1;
        } else if greater == left_val {
            direct = Left;
            j -= 1;
        } else if greater == uppr_val {
            direct = Up;
            i -= 1;
        }
    }
    /****************************************/

    align1 = align1.chars().rev().collect::<String>();
    align2 = align2.chars().rev().collect::<String>();

    (get_hsp(&align1, &align2, 1, -1, -2), align2, align1)
}

fn star_alignment(seqs: Vec<&str>) {
    let len = seqs.len();
    let mut final_seqs: Vec<(String, String)> = vec![(String::new(), String::new()); len];
    {
        let mut matrix: Vec<Vec<(i32, String, String)>> =
            vec![vec![(0, String::new(), String::new()); len]; len];
        let mut d = vec![0; len];

        let mut max_pos = 0;
        for j in 0..len {
            let mut i = j + 1;
            while i < len {
                let val = align_secuence(seqs[i], seqs[j]);
                matrix[j][i] = val;
                matrix[i][j].0 = matrix[j][i].0;
                matrix[i][j].1 = matrix[j][i].2.clone();
                matrix[i][j].2 = matrix[j][i].1.clone();
                i += 1;
            }

            for i in 0..len {
                d[j] += matrix[j][i].0;
            }

            if d[j] > d[max_pos] {
                max_pos = j;
            }
        }
        matrix[max_pos][max_pos].1 = String::from(seqs[max_pos]);
        matrix[max_pos][max_pos].2 = String::from(seqs[max_pos]);

        print_matrix(&matrix);

        for i in 0..len {
            final_seqs[i].0 = matrix[max_pos][i].1.clone();
            final_seqs[i].1 = matrix[max_pos][i].2.clone();
        }
    }

    // println!("{:?}", final_seqs);

    let mut aux = 0;
    for i in 1..len {
        if let Some(offset) = final_seqs[i].0.find('_') {
            // println!("{}> [{}]", i, offset);
            for j in 0..len {
                if j != i {
                    let t: String = final_seqs[j].1.split_off(offset + aux);
                    // println!("{} -> {}", final_seqs[i].1, t);
                    final_seqs[j].1.push_str("_");
                    final_seqs[j].1 += t.as_ref();
                }
            }
            aux += 1;
        }
    }

    println!("");
    for i in 0..len {
        println!("S{}> {}", i + 1, final_seqs[i].1);
    }
    // println!("{:?}", final_seqs);
}

fn main() {
    // let seqs: Vec<&str> = vec!["CCTGCTGCAG",
    //                            "GATGTGCCG",
    //                            "GATGTGCAG",
    //                            "CCGCTAGCAG",
    //                            "CCTGTAGG"];
    // star_alignment(seqs);

    println!("{:?}", align_secuence("GATGTGCCG", "CCTGTAGG"));
}
