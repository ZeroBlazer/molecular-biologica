extern crate num;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use num::{BigInt, BigUint, Zero, One, FromPrimitive};

fn combinator(n: i32, k: i32) -> f64 {
    let mut num: f64 = 1.0;
    let mut n_aux = n;
    let mut d_aux = n - k;
    num = 1.0 / factorial(k) as f64;
    if k < d_aux {
        while n_aux > d_aux {
            num = num * n_aux as f64;
            n_aux -= 1;
        }
        num
    } else {
        while n_aux > k {
            num = num * n_aux as f64;
            n_aux -= 1;
        }
        num
    }
}

fn factorial(n: i32) -> u64 {
    if n <= 1 {
        return 1;
    }

    let mut fact: u64 = 1;
    let mut x: u64 = 2;

    while x <= n as u64 {
        fact = fact * x;
        x += 1;
    }

    fact
}

fn binomial_distrib(n: i32, k: i32, p: f32) -> f64 {
    let q = 1.0 - p;
    combinator(n, k) * (p.powf(k as f32) * q.powf((n - k) as f32)) as f64
}

fn do_calculation_a() {
    println!("Doing calculations");
    let val_a = binomial_distrib(40, 8, 0.35);
    let val_b = binomial_distrib(20, 12, 0.35);
    let val_c = binomial_distrib(30, 11, 0.35);
    let val_d = binomial_distrib(38, 9, 0.65);
    let val_e = binomial_distrib(51, 13, 0.65);

    println!("Writing to file");
    let mut file = File::create("EPuma_Practica04_A.txt").expect("Couldn't open write file");
    write!(file,
           "{}\n{}\n{}\n{}\n{}",
           val_a,
           val_b,
           val_c,
           val_d,
           val_e)
            .expect("Couldn't write in file");
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

fn do_calculation_b() {
    let path = "practica_04_B.txt";
    let mut counter: [u32; 3] = [0; 3];
    process_file(path, &mut counter);

    let prob_pur = counter[0] as f32 / counter[2] as f32;
    let prob_pir = counter[1] as f32 / counter[2] as f32;

    println!("Pur: {}, Pir: {}, probs: {} & {}",
             counter[0],
             counter[1],
             prob_pur,
             prob_pir);

    let val_a = binomial_distrib(10, 5, prob_pur);
    let val_b = binomial_distrib(15, 7, prob_pur);
    let val_c = binomial_distrib(13, 6, prob_pir);
    let val_d = binomial_distrib(12, 3, prob_pir);

    println!("Writing to file");
    let mut file = File::create("EPuma_Practica04_B.txt").expect("Couldn't open write file");
    write!(file, "{}\n{}\n{}\n{}", val_a, val_b, val_c, val_d).expect("Couldn't write in file");
}

fn main() {
    do_calculation_a();
    do_calculation_b();
}
