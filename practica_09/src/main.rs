use std::cmp::max;

#[derive(Debug)]
enum Direction {
    Diagonal,
    Up,
    Left,
}

use Direction::*;

fn print_matrix(sec_1: &str, sec_2: &str, mtrx: &Vec<Vec<i32>>) {
    print!("\t\t");
    for chr in sec_1.chars() {
        print!(" {}\t", chr);
    }
    print!("\n\t");

    for elem in mtrx[0].iter() {
        print!("{}\t", elem);
    }
    println!("");

    let mut i: usize = 1;
    for chr in sec_2.chars() {
        print!("{}\t", chr);
        for elem in mtrx[i].iter() {
            print!("{}\t", elem);
        }
        println!("");
        i += 1;
    }
}

fn align_secuence(sec_1: &str, sec_2: &str, match_scr: i32, mism_scr: i32, gap_pen: i32) {
    let len_1 = sec_1.len() + 1;
    let len_2 = sec_2.len() + 1;

    /********* Initialize matrix **********/
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; len_1]; len_2];
    /**************************************/

    let mut diag_val;
    let mut left_val;
    let mut uppr_val;

    let mut i = 1;
    let mut j;
    for c2 in sec_2.chars() {
        j = 1;
        for c1 in sec_1.chars() {
            let score = if c1 == c2 { match_scr } else { mism_scr };
            diag_val = matrix[i - 1][j - 1] + score;
            left_val = matrix[i][j - 1] + gap_pen;
            uppr_val = matrix[i - 1][j] + gap_pen;
            matrix[i][j] = max(max(0, diag_val), max(left_val, uppr_val));
            j += 1;
        }
        i += 1;
    }

    print_matrix(sec_1, sec_2, &matrix);

    //Looking for the greatest value in matrix
    i = sec_1.len() + 1;
    j = sec_2.len() + 1;

    let mut x_gst = 0;
    let mut y_gst = 0;
    for y in (0..j).rev() {
        for x in (0..i).rev() {
            if matrix[y_gst][x_gst] < matrix[y][x] {
                x_gst = x;
                y_gst = y;
            }
        }
    }
    ///////////////////////////////////////////

    /************** Alignment **************/{
    let mut align = String::new();
    let mut direct = Diagonal;

    i = x_gst;
    j = y_gst;

    while matrix[j][i] != 0 {
        let c1 = sec_1.as_bytes()[i - 1] as char;
        let c2 = sec_2.as_bytes()[j - 1] as char;

        let score = if c1 == c2 { match_scr } else { mism_scr };
        diag_val = matrix[j - 1][i - 1] + score;
        left_val = matrix[j][i - 1] + gap_pen;
        uppr_val = matrix[j - 1][i] + gap_pen;

        if matrix[j][i] == left_val {
            direct = Left;
            i -= 1;
        } else if matrix[j][i] == diag_val {
            direct = Diagonal;
            i -= 1;
            j -= 1;
        } else if matrix[j][i] == uppr_val {
            direct = Up;
            j -= 1;
        }

        match direct {
            Diagonal => {
                align.push(c2);
            }
            Up => {
                align.push(c2);
            }
            Left => {
                align.push('_');
            }
        }
    }

    println!("\nAlignment1: {}", align.chars().rev().collect::<String>());}
    /****************************************/

    /************** Alignment **************/{
    let mut align = String::new();
    let mut direct = Diagonal;

    i = x_gst;
    j = y_gst;

    while matrix[j][i] != 0 {
        let c1 = sec_1.as_bytes()[i - 1] as char;
        let c2 = sec_2.as_bytes()[j - 1] as char;

        let score = if c1 == c2 { match_scr } else { mism_scr };
        diag_val = matrix[j - 1][i - 1] + score;
        left_val = matrix[j][i - 1] + gap_pen;
        uppr_val = matrix[j - 1][i] + gap_pen;

        if matrix[j][i] == left_val {
            direct = Left;
            i -= 1;
        } else if matrix[j][i] == diag_val {
            direct = Diagonal;
            i -= 1;
            j -= 1;
        } else if matrix[j][i] == uppr_val {
            direct = Up;
            j -= 1;
        }

        match direct {
            Diagonal => {
                align.push(c1);
            }
            Up => {
                align.push('_');
            }
            Left => {
                align.push(c2);
            }
        }
    }

    println!("Alignment2: {}", align.chars().rev().collect::<String>());}
    /****************************************/
}

fn main() {
    // align_secuence("GGAT", "GAT", 1, -1, -1);
    // align_secuence("GAATTCAGTTA", "GGATCGA", 5, -3, -4);
    // align_secuence("GAATTCGAGTTA", "GGATCGA", 5, -3, -4);
    // align_secuence("CAGTATCGT", "GTACGTATC", 1, -1, -2);
    align_secuence("GACTTC", "ATT", 1, -1, -2);
}
