extern crate alignments;

use alignments::tps_alignment;

fn main() {
    let seqs: Vec<&str> = vec!["TGTTAACA", "TGTAACA", "TGTACA", "ATGTCA", "ATGTGG"];
    tps_alignment(seqs);
}
