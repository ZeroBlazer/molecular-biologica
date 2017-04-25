use std::cmp::min;

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

fn print_matrix(sec_1: &str, sec_2: &str, mtrx: Vec<Vec<i32>>) {
    for row in mtrx.into_iter() {
        println!("{:?}", row);
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
    let mut score;

    

    for i in 1..len_2 {
        let char_1: char = String::from(sec_2).chars().collect()[i];
        for j in 1..len_1 {
            score = 10;
            diag_val = score + matrix[i - 1][j - 1];
            left_val = score + matrix[i - 1][j];
            uppr_val = score + matrix[i][j - 1];
            matrix[i][j] = min(diag_val, min(left_val, uppr_val));
        }
    }

    println!("Dims: {} x {}", len_1, len_2);
    print_matrix(sec_1, sec_2, matrix);
}

fn main() {
    align_secuence("ACCGTCTT", "CGTCTT");
    // println!("Hello, world!");
}
