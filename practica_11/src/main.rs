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

fn print_matrix(mtrx: &Vec<Vec<i32>>) {
    for row in mtrx.iter() {
        for elem in row.iter() {
            print!("{}\t", elem);
        }
        println!("");
        // println!("{:?}", row);
    }
}

fn align_secuence(sec_1: &str, sec_2: &str) -> i32 {
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
    let mut j = 1;
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
    let mut align = String::new();

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
                align.push(sec_2.as_bytes()[i - 1] as char);
            }
            Up => {
                align.push('_');
            }
            Left => {
                align.push('_');
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

    println!("\nAlignment: {}", align.chars().rev().collect::<String>());
    /****************************************/

    matrix[len_2 - 1][len_1 - 1]
}

fn star_alignment(seqs: Vec<&str>) {
    let len = seqs.len();
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; len]; len];
    let mut D = vec![0; len];

    let mut max_pos = 0;
    for j in 0..len {
        let mut i = j + 1;
        while i < len {
            let val = align_secuence(seqs[i], seqs[j]);
            matrix[j][i] = val;
            matrix[i][j] = val;
            i += 1;
        }
        // for i in 0..len {
        //     matrix[j][i] = if i != j {
        //         align_secuence(seqs[i], seqs[j])
        //     } else {
        //         0
        //     };
        // }

        for i in 0..len {
            D[j] += matrix[j][i];
        }

        if D[j] > D[max_pos] {
            max_pos = j;
        }
    }

    print_matrix(&matrix);
    println!("{:?} -> {}", D, max_pos);


}

fn main() {
    let seqs: Vec<&str> = vec!["CCTGCTGCAG",
                               "GATGTGCCG",
                               "GATGTGCAG",
                               "CCGCTAGCAG",
                               "CCTGTAGG"];
    star_alignment(seqs);
}
