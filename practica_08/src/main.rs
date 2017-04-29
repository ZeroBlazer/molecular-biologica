use std::cmp::max;

#[derive(Debug)]
enum Aminoacid {
    A,
    G,
    C,
    T,
    Undef,
}

macro_rules! char_at {
    ($str: ident, $index: ident) => {
        &$str[$index..$index+1]
    }
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
    for c2 in sec_2.chars() {
        let mut j = 1;
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

    let mut i = sec_1.len();
    let mut j = sec_2.len();
    let mut alignment = String::new();

    while i > 0 || j > 0 {
        diag_val = matrix[j - 1][i - 1];
        left_val = matrix[j - 1][i];
        uppr_val = matrix[j][i - 1];

        println!("{}\t{}\t{}", diag_val, left_val, uppr_val);

        if diag_val >= left_val && diag_val >= uppr_val {
                j -= 1;
                i-= 1;
                alignment += char_at!(sec_2, j);
        } else if left_val > uppr_val {
                j -= 1;
                alignment += "_";
            } else {
                i -= 1;
                alignment += "_";
            }
    }
    
    println!("\nAlignment: {}", alignment.chars().rev().collect::<String>());
    println!("\nMatrix:");
    print_matrix(sec_1, sec_2, matrix);
}

fn main() {
    align_secuence("ACCGTCTT", "CGTCTT");
    // align_secuence("ACTTAG", "ACTTC");
    // println!("Hello, world!");
}
