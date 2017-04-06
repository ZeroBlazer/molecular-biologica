use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn process_file(p: &str) {
    let path = Path::new(p);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
}

fn probal_func() {
    let path1 = "practica_03A_1.txt";
    // let path2 = Path::new("practica_03A_2.txt");

    process_file(path1);
    // process_file(path2);


    // let mut counter_1: i32 = 3;
    // println!("{}", counter_1);
}

fn main() {
    probal_func();
    println!("Hello, world!");
}
