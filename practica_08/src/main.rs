#[derive(Debug)]
enum Aminoacid {
    A,
    G,
    C,
    T,
    Undef
}

static scores: [[i32; 4]; 4] = [[10, -1, -3, -4],
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

    scores[b_pos as usize][a_pos as usize]
}

fn print_matrix(sec_1: &str, sec_2: &str, mtrx: Vec<Vec<i32>>) {
    for row in mtrx.into_iter() {
        println!("{:?}", row);
    }
}

fn align_secuence(sec_1: &str, sec_2: &str) {
    let len_1 = sec_1.len();
    let len_2 = sec_2.len();

    // Initialize matrix
    let matrix: Vec<Vec<i32>> = vec![vec![0; len_1]; len_2];

    println!("Dims: {} x {}", len_1, len_2);
    print_matrix(sec_1, sec_2, matrix);
}

fn main() {
    align_secuence("ACCGTCTT", "CGTCTT");
    // println!("Hello, world!");
}
