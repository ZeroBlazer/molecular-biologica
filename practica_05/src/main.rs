use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn geometric_distr(p: f32, n: u32) -> f32 {
    let diff = 1.0 - p;
    diff.powf((n - 1) as f32) * p
}

fn process_file(p: &str) -> (f32, f32, f32) {
    let path = Path::new(p);
    let mut file = File::open(&path).expect("Unable to open");

    let mut s = String::new();
    file.read_to_string(&mut s).expect("Not able to read");

    let mut ctr_0 = 0.0;
    let mut ctr_1 = 0.0;

    for c in s.chars() {
        match c {
            'A' => ctr_0 += 1.0,
            'G' => ctr_0 += 1.0,
            'C' => ctr_1 += 1.0,
            'T' => ctr_1 += 1.0,
            _ => println!("Not a valid letter"),
        }
    }
    (ctr_0, ctr_1, s.len() as f32)
}

fn part_a() {
    let prob_pur = 0.68;
    let prob_pir = 0.32;

    let input = vec![(prob_pur, 3),
                     (prob_pur, 5),
                     (prob_pur, 7),
                     (prob_pur, 9),
                     (prob_pir, 4),
                     (prob_pir, 6),
                     (prob_pir, 8),
                     (prob_pir, 10)];

    let mut file = File::create("EPuma_Practica05_A.txt").expect("Couldn't open write file");
    for &(x, y) in input.iter() {
        write!(file, "{val}\n", val = geometric_distr(x, y)).expect("Couldn't write in file");
    }
}

fn part_b() {
    let path = "practica_05_B.txt";
    let (mut prob_pur, mut prob_pir, length) = process_file(path);

    prob_pur = prob_pur / length;
    prob_pir = prob_pir /length;

    let input = vec![(prob_pur, 2), (prob_pur, 4), (prob_pir, 3), (prob_pir, 5)];

    let mut file = File::create("EPuma_Practica05_B.txt").expect("Couldn't open write file");

    for &(x, y) in input.iter() {
        write!(file, "{}\n", geometric_distr(x, y)).expect("Couldn't write in file");
    }
}

fn main() {
    println!("Running Part A");
    part_a();
    println!("Running Part B");
    part_b();
    println!("Done ;)");
}
