use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn factorial(n: u32) -> u64 {
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

fn multinomial_distr(n: u32, pairs: Vec<(u32, f32)>) -> f64 {
    // let calc_den = pairs
    //     .iter()
    //     .map(|(x, y)| factorial(x))
    //     .fold(0, |sum, i| sum + i);
    // let calc_num = pairs.map(|(x, y)| (x as f32).powf(y));
    let mut calc_den: u64 = 1;
    let mut calc_num = 1.0;
    for &(x, y) in pairs.iter() {
        calc_den *= factorial(x);
        calc_num *= y.powf(x as f32);
    }
    // println!("calc_den: {}", calc_den as f64);
    // println!("calc_num: {}", calc_num);
    // println!("R: {}", (factorial(n) as f64 / calc_den as f64));

    (factorial(n) as f64 / calc_den as f64) * calc_num as f64
}

fn probal(mut n: u32, m: u32, mut total: u32, mut it: u32) -> f32 {
    let mut ret = 1.0;
    while it > 0 {
        ret *= n as f32 / total as f32;
        n -= 1;
        total -= 1;
        it -= 1;
    }
    ret *= m as f32 / total as f32;
    ret
}

fn expected_val(n: u32, m: u32) -> f32 {
    let total = n + m;
    let mut val = 0.0;

    for x in 1..(n + 2) {
        val += x as f32 * probal(n, m, total, x - 1);
    }
    val
}

fn process_file(p: &str) -> (u32, u32, u32, u32, u32) {
    let path = Path::new(p);
    let mut file = File::open(&path).expect("Unable to open");

    let mut s = String::new();
    file.read_to_string(&mut s).expect("not able to read");

    let mut ctr = (0, 0, 0, 0, s.len() as u32);

    for c in s.chars() {
        match c {
            'A' => ctr.0 += 1,
            'G' => ctr.1 += 1,
            'C' => ctr.2 += 1,
            'T' => ctr.3 += 1,
            _ => println!("Not a valid letter"),
        }
    }
    ctr
}

fn part_a() {
    // let pairs = vec![(5, 0.15), (10, 0.40), (5, 0.45)];
    // println!("Res: {}", multinomial_distr(20, pairs));
    // let pairs = vec![(3, 0.4), (3, 0.2), (1, 0.3), (2, 0.1)];
    // println!("Res: {}", multinomial_distr(9, pairs));
    let pairs1 = vec![(2, 0.35), (3, 0.35), (2, 0.15), (1, 0.15)];
    let pairs2 = vec![(3, 0.35), (2, 0.35), (2, 0.15), (1, 0.15)];

    let mut file = File::create("EPuma_Practica_06A.txt").expect("Couldn't open write file");
    write!(file, "{}\n", multinomial_distr(8, pairs1)).expect("Couldn't write in file");
    write!(file, "{}\n", multinomial_distr(8, pairs2)).expect("Couldn't write in file");
}

fn part_b() {
    let path = "practica_06B.txt";
    let (n_a, n_g, n_c, n_t, length) = process_file(path);

    let prob_a = n_a as f32 / length as f32;
    let prob_g = n_g as f32 / length as f32;
    let prob_c = n_c as f32 / length as f32;
    let prob_t = n_t as f32 / length as f32;

    let input1 = vec![(3, prob_c), (4, prob_g), (2, prob_a), (4, prob_t)];
    let input2 = vec![(2, prob_c), (3, prob_g), (2, prob_a), (3, prob_t)];
    let input3 = vec![(1, prob_c), (2, prob_g), (3, prob_a), (2, prob_t)];
    let input4 = vec![(2, prob_c), (1, prob_g), (3, prob_a), (1, prob_t)];

    let mut file = File::create("EPuma_Practica_06B.txt").expect("Couldn't open write file");
    write!(file, "{}\n", multinomial_distr(13, input1)).expect("Couldn't write in file");
    write!(file, "{}\n", multinomial_distr(10, input2)).expect("Couldn't write in file");
    write!(file, "{}\n", multinomial_distr(8, input3)).expect("Couldn't write in file");
    write!(file, "{}\n", multinomial_distr(7, input4)).expect("Couldn't write in file");
}

fn part_c() {
    let path = "practica_06C.txt";
    let (prob_a, prob_g, prob_c, prob_t, _) = process_file(path);

    let n_pur = prob_a + prob_g;
    let n_pir = prob_c + prob_t;

    let mut file = File::create("EPuma_Practica_06C.txt").expect("Couldn't open write file");

    write!(file, "{}\n", expected_val(n_pur, n_pir)).expect("Couldn't write in file");
    write!(file, "{}\n", expected_val(n_pir, n_pur)).expect("Couldn't write in file");
}

fn main() {
    println!("Up and running!");
    part_a();
    part_b();
    part_c();
}
