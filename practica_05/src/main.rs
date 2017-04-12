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

    let val_a = geometric_distr(prob_pur, 3);
    let val_b = geometric_distr(prob_pur, 5);
    let val_c = geometric_distr(prob_pur, 7);
    let val_d = geometric_distr(prob_pur, 9);
    let val_e = geometric_distr(prob_pir, 4);
    let val_f = geometric_distr(prob_pir, 6);
    let val_g = geometric_distr(prob_pir, 8);
    let val_h = geometric_distr(prob_pir, 10);


    let mut file = File::create("EPuma_Practica05_A.txt").expect("Couldn't open write file");
    write!(file,
           "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
           val_a,
           val_b,
           val_c,
           val_d,
           val_e,
           val_f,
           val_g,
           val_h)
            .expect("Couldn't write in file");
}

fn part_b() {
    let path = "practica_05_B.txt";
    let mut counter: [u32; 3] = [0; 3];
    process_file(path, &mut counter);

    let prob_pur = counter[0] as f32 / counter[2] as f32;
    let prob_pir = counter[1] as f32 / counter[2] as f32;

    let val_a = geometric_distr(prob_pur, 2);
    let val_b = geometric_distr(prob_pur, 4);
    let val_c = geometric_distr(prob_pir, 3);
    let val_d = geometric_distr(prob_pir, 5);

    let mut file = File::create("EPuma_Practica05_B.txt").expect("Couldn't open write file");
    write!(file, "{}\n{}\n{}\n{}", val_a, val_b, val_c, val_d).expect("Couldn't write in file");
}

fn main() {
    println!("Hello, world!");
    part_a();
    part_b();
}
