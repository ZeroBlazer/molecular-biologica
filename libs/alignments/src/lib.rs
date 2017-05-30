mod tps_tree;

use tps_tree::TpsTree;

fn get_hsp(w1: &String, w2: &String, match_scr: i32, mism_scr: i32, gap_scr: i32) -> i32 {
    let mut hsp = 0;

    let mut chars2 = w2.chars();
    for c1 in w1.chars() {
        let c2 = chars2.next().unwrap();
        hsp += if c1 == c2 {
            match_scr
        } else if c1 == '_' || c2 == '_' {
            gap_scr
        } else {
            mism_scr
        }
    }

    hsp
}

fn pair_sum(sec_1: &str, sec_2: &str, match_scr: i32, mism_scr: i32, gap_scr: i32) -> f32 {
    let len_1 = sec_1.len();
    let len_2 = sec_2.len();
    let diff: i32 = len_1 as i32 - len_2 as i32;

    let sec_1s;
    let sec_2s;

    if diff > 0 {
        sec_1s = String::from(sec_1);
        sec_2s = format!("{}{}", sec_2, "_".repeat(diff as usize));
    } else {
        sec_1s = format!("{}{}", sec_1, "_".repeat(diff.abs() as usize));
        sec_2s = String::from(sec_2);
    }
    // println!("A: {}\nB: {}\n> {}\n", sec_1s, sec_2s, get_hsp(&sec_1s, &sec_2s, match_scr, mism_scr, gap_scr) as f32 / sec_1s.len() as f32);
    get_hsp(&sec_1s, &sec_2s, match_scr, mism_scr, gap_scr) as f32 / sec_1s.len() as f32
}

pub fn tps_alignment(seqs: Vec<&str>) {
    let len = seqs.len();
    let mut my_vec = Vec::new();

    for i in 0..len {
        let mut j = i + 1;
        while j < len {
            my_vec.push((pair_sum(seqs[i], seqs[j], 2, 1, 1), i, j));
            j += 1;
        }
    }

    my_vec.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    println!("{:?}", my_vec);

    let tree = TpsTree::new();
}