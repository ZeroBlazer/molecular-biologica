fn get_sequences(path: &str) -> Vec<String> {
    vec!["PPGVKSDCAS".to_string(),
         "PADGVKDCAS".to_string(),
         "PPDGKSDS".to_string(),
         "GADGKDCCS".to_string(),
         "GADGKDCAS".to_string()]
    // Vec::new()
}

fn mult_seq_alignment(input: &[String]) {
    println!("Hello, world!");
}

fn main() {
    let input = get_sequences("input/MSA_16507.txt");
    mult_seq_alignment(&input);
}
