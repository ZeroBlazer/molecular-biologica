// use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn process_file(p: &str, ctr: &mut [u32; 5]) {
    let path = Path::new(p);
    // let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    // let mut file = match File::open(&path) {
    //     Err(why) => panic!("couldn't open {}: {}", display, why.description()),
    //     Ok(file) => file,
    // };
    let mut file = File::open(&path).expect("Unable to open");

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    // match file.read_to_string(&mut s) {
    //     Err(why) => panic!("couldn't read {}: {}", display, why.description()),
    //     Ok(_) => print!("{} contains:\n{}", display, s),
    // }
    file.read_to_string(&mut s).expect("not able to read");
    // print!("{} contains:\n{}", display, s);

    ctr[4] = s.len() as u32;

    for c in s.chars() {
        match c {
            'A' => ctr[0] += 1,
            'T' => ctr[1] += 1,
            'C' => ctr[2] += 1,
            'G' => ctr[3] += 1,
            _ => println!("Not a valid letter"),
        }
    }
}

fn probal_func() {
    let path1 = "practica_03A_1.txt";
    let path2 = "practica_03A_2.txt";
    let path3 = "practica_03A_3.txt";
    let path4 = "practica_03A_4.txt";
    let mut counter1: [u32; 5] = [0; 5];
    let mut counter2: [u32; 5] = [0; 5];
    let mut counter3: [u32; 5] = [0; 5];
    let mut counter4: [u32; 5] = [0; 5];
    // let path2 = Path::new("practica_03A_2.txt");

    process_file(path1, &mut counter1);
    process_file(path2, &mut counter2);
    process_file(path3, &mut counter3);
    process_file(path4, &mut counter4);

    let prob_a: f32 = 0.25 * counter1[0] as f32 / counter1[4] as f32 +
                      0.25 * counter2[0] as f32 / counter2[4] as f32 +
                      0.25 * counter3[0] as f32 / counter3[4] as f32 +
                      0.25 * counter4[0] as f32 / counter4[4] as f32;

    let prob_t: f32 = 0.25 * counter1[1] as f32 / counter1[4] as f32 +
                      0.25 * counter2[1] as f32 / counter2[4] as f32 +
                      0.25 * counter3[1] as f32 / counter3[4] as f32 +
                      0.25 * counter4[1] as f32 / counter4[4] as f32;

    let prob_c: f32 = 0.25 * counter1[2] as f32 / counter1[4] as f32 +
                      0.25 * counter2[2] as f32 / counter2[4] as f32 +
                      0.25 * counter3[2] as f32 / counter3[4] as f32 +
                      0.25 * counter4[2] as f32 / counter4[4] as f32;

    let prob_g: f32 = 0.25 * counter1[3] as f32 / counter1[4] as f32 +
                      0.25 * counter2[3] as f32 / counter2[4] as f32 +
                      0.25 * counter3[3] as f32 / counter3[4] as f32 +
                      0.25 * counter4[3] as f32 / counter4[4] as f32;

    println!("Values are: {}, {}, {}, {}", prob_a, prob_t, prob_c, prob_g);

    let mut file = File::create("EPuma_Practica03_A.txt").expect("Couldn't open write file");
    write!(file, "{}\n{}\n{}\n{}", prob_a, prob_t, prob_c, prob_g).expect("Couldn't write in file");
}

fn main() {
    probal_func();
    println!("Hello, world!");
}
