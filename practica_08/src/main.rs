use std::cmp::max;

#[derive(Debug)]
enum Aminoacid {
    A,
    G,
    C,
    T,
    Undef,
}

#[derive(Debug)]
enum Direction {
    Diagonal,
    Up,
    Left,
}

use Direction::*;

static SCORES: [[i32; 4]; 4] = [[10, -1, -3, -4],
                                [-1, 7, -5, -3],
                                [-3, -5, 9, 0],
                                [-4, -3, 0, 8]];

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

// fn print_matrix(mtrx: &Vec<Vec<i32>>) {
//     for row in mtrx.into_iter() {
//         // println!("{:?}", row);
//         for elem in row.into_iter() {
//             print!("{}\t", elem);
//         }
//         println!("");
//     }
// }

fn print_matrix(sec_1: &str, sec_2: &str, mtrx: &Vec<Vec<i32>>) {
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

fn align_secuence(sec_1: &str, sec_2: &str) {
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
            left_val = -5 + matrix[i][j - 1];
            uppr_val = -5 + matrix[i - 1][j];
            matrix[i][j] = max(diag_val, max(left_val, uppr_val));
            j += 1;
        }
        i += 1;
    }

    print_matrix(sec_1, sec_2, &matrix);

    /************** Alignment **************/ {
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
    
    println!("\nAlignment: {}", align.chars().rev().collect::<String>()); }
    /****************************************/

    /************** Alignment **************/ {
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

        if greater == left_val {
            direct = Left;
            j -= 1;
        } else if greater == diag_val {
            direct = Diagonal;
            i -= 1;
            j -= 1;
        } else if greater == uppr_val {
            direct = Up;
            i -= 1;
        }
    }
    
    println!("Alignment: {}", align.chars().rev().collect::<String>()); }
    /****************************************/
}

fn main() {
    // align_secuence("ACCGTCTT", "ACGTCTT");
    align_secuence("GATGTGCCG", "CCTGTAGG");
}
