fn get_words(sec: &str, w: usize) -> Vec<String> {
    let lim = sec.len() - w + 1;
    let mut words = Vec::new();

    for i in 0..lim {
        let mut word = String::new();
        word.push(sec.as_bytes()[i] as char);
        word.push(sec.as_bytes()[i + 1] as char);
        word.push(sec.as_bytes()[i + 2] as char);

        words.push(word);
    }

    words
}

fn get_hsp(w1: &String, w2: &String, match_scr: i32, mism_scr: i32) -> i32 {
    let mut hsp = 0;

    let mut chars2 = w2.chars();
    for c in w1.chars() {
        hsp += if c == chars2.next().unwrap() {
            match_scr
        } else {
            mism_scr
        }
    }

    hsp
}

fn expand_words(align1: &mut String,
                align2: &mut String,
                sec_1: &str,
                sec_2: &str,
                indx1_i: &mut usize,
                indx1_f: &mut usize,
                indx2_i: &mut usize,
                indx2_f: &mut usize)
                -> (bool, bool) {
    if *indx1_i != 0 && *indx2_i != 0 {
        let mut ps_align1 = align1.chars().rev().collect::<String>();
        let mut ps_align2 = align2.chars().rev().collect::<String>();
        ps_align1.push(sec_1.as_bytes()[*indx1_i - 1] as char);
        ps_align2.push(sec_2.as_bytes()[*indx2_i - 1] as char);
        *align1 = ps_align1.chars().rev().collect::<String>();
        *align2 = ps_align2.chars().rev().collect::<String>();
        *indx1_i -= 1;
        *indx2_i -= 1;
    }

    if *indx1_f < sec_1.len() && *indx2_f < sec_2.len() {
        align1.push(sec_1.as_bytes()[*indx1_f] as char);
        align2.push(sec_2.as_bytes()[*indx2_f] as char);
        *indx1_f += 1;
        *indx2_f += 1;
    }
    
    (!(*indx1_i != 0 && *indx2_i != 0), !(*indx1_f < sec_1.len() && *indx2_f < sec_2.len()))
}

fn print_alignment(align1: String, align2: String) {
    let mut chars2 = align2.chars();
    let mut end_first_it = false;
    let mut algn1 = String::new();
    let mut algn2 = String::new();

    for c1 in align1.chars() {
        let c2 = chars2.next().unwrap();
        if c1 == c2 {
            end_first_it = true;
        }
        if end_first_it {
            algn1.push(c1);
            algn2.push(c2);
        }
    }

    loop {
        let c1 = algn1.pop().unwrap();
        let c2 = algn2.pop().unwrap();

        if c1 == c2 {
            algn1.push(c1);
            algn2.push(c2);
            break;
        }
    }
    
    println!("S1> {}\nS2> {}", algn1, algn2);
}

fn align_secuence(sec_1: &str,
                  sec_2: &str,
                  w: usize,
                  match_scr: i32,
                  mism_scr: i32,
                  hps: i32,
                  u: i32) {
    let words_s1 = get_words(sec_1, w);
    let words_s2 = get_words(sec_2, w);

    let mut indx1_i: usize = 0;
    let mut indx2_i: usize = 0;
    let mut align1 = String::new();
    let mut align2 = String::new();

    'outer: for word1 in words_s1.iter() {
        indx2_i = 0;
        for word2 in words_s2.iter() {
            if get_hsp(word1, word2, match_scr, mism_scr) >= hps {
                align1 = word1.clone();
                align2 = word2.clone();
                break 'outer;
            }
            indx2_i += 1;
        }
        indx1_i += 1;
    }

    let mut indx1_f = indx1_i + w;
    let mut indx2_f = indx2_i + w;

    let mut algn1 = align1.clone();
    let mut algn2 = align2.clone();

    while {
        let hsp = get_hsp(&align1, &align2, match_scr, mism_scr);
        // println!("{}, {}\n{} -> {}\n", align1, align2, hsp, hsp > u);
        hsp > u
    } {
        algn1 = align1.clone();
        algn2 = align2.clone();

        let (a, b) = expand_words(&mut align1, &mut align2, &sec_1, &sec_2, &mut indx1_i, &mut indx1_f, &mut indx2_i, &mut indx2_f);
            
        if a && b {
            algn1 = align1.clone();
            algn2 = align2.clone();
            break;
        }
    }

    print_alignment(algn1, algn2);
}

fn main() {
    align_secuence("TCAGATCAACT", "GTATCGAAA", 3, 1, -2, 3, -5);
    // align_secuence("TCAGATCACTT", "GTATCGCTC", 3, 1, -2, 3, -5);
}
