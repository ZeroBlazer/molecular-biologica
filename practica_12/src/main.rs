#[derive(Debug)]
enum Direction {
    Diagonal,
    Up,
    Left,
}

use Direction::*;
use std::cmp::max;

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

fn get_score(a: char, b: char) -> i32 {
    if a == b { 1 } else { -1 }
}

fn align_secuence(sec_1: &str, sec_2: &str) -> (String, String) {
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
    /**************************************/

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

    (align2, align1)
}

fn tps_alignment(seqs: Vec<&str>) {
    let len = seqs.len();

    for i in 0..len {
        let mut j = i + 1;
        while j < len {
            let aligns = align_secuence(seqs[i], seqs[j]);
            println!("{}", get_hsp(&aligns.0, &aligns.1, 0, 3, 2));
            j += 1;
        }
    }
}

fn main() {
    let seqs: Vec<&str> = vec!["ACTCAT",
                               "AGTCAT",
                               "ACGTCCT"];
    tps_alignment(seqs);
}
