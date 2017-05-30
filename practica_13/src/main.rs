extern crate alignments;

use alignments::tps_alignment;

fn main() {
    let seqs: Vec<&str> = vec!["TGTTAAC", "TGTAAC", "TGTAC", "ATGTC", "ATGTGGC"];
    tps_alignment(seqs);
}
