use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn geometric_distr(p: f32, n: u32) -> f32 {
    let diff = 1.0 - p;
    diff.powf((n - 1) as f32) * p
}

fn process_file(p: &str, ctr: &mut [u32; 3]) {
    let path = Path::new(p);

    let mut file = File::open(&path).expect("Unable to open");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("not able to read");

    ctr[2] = s.len() as u32;

    for c in s.chars() {
        match c {
            'A' => ctr[0] += 1,
            'G' => ctr[0] += 1,
            'C' => ctr[1] += 1,
            'T' => ctr[1] += 1,
            _ => println!("Not a valid letter"),
        }
    }
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
        let val = geometric_distr(x, y);
        write!(file, "{}\n", val).expect("Couldn't write in file");
    }
}

fn part_b() {
    let path = "practica_05_B.txt";
    let mut counter: [u32; 3] = [0; 3];
    process_file(path, &mut counter);

    let prob_pur = counter[0] as f32 / counter[2] as f32;
    let prob_pir = counter[1] as f32 / counter[2] as f32;

    let input = vec![(prob_pur, 2), (prob_pur, 4), (prob_pir, 3), (prob_pir, 5)];

    let mut file = File::create("EPuma_Practica05_B.txt").expect("Couldn't open write file");

    for &(x, y) in input.iter() {
        let val = geometric_distr(x, y);
        write!(file, "{}\n", val).expect("Couldn't write in file");
    }
}

fn main() {
    println!("Hello, world!");
    part_a();
    part_b();
}
