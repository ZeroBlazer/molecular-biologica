#[derive(Debug)]
enum TpsBranch {
    Seq(String),
    Tree(Box<TpsTree>),
    Empty,
}

use self::TpsBranch::*;
use super::align_secuence;

#[derive(Debug)]
pub struct TpsJoint {
    left: TpsBranch,
    right: TpsBranch,
    alignments: Vec<String>,
}

impl TpsJoint {
    pub fn new() -> TpsJoint {
        TpsJoint {
            left: Empty,
            right: Empty,
            alignments: Vec::new()
        }
    }

    pub fn from_strings(sec_1: &str, sec_2: &str) -> TpsJoint {
        let (align2, align1) = align_secuence(sec_1, sec_2);

        TpsJoint {
            left: Seq(String::from(sec_1)),
            right: Seq(String::from(sec_2)),
            alignments: vec![align1, align2]
        }
    }
}

#[derive(Debug)]
pub struct TpsTree {
    root: Box<TpsJoint>,
    // keys: HashMap<usize, >
}

impl TpsTree {
    pub fn new() -> TpsTree {
        TpsTree { root: Box::new(TpsJoint::new()) }
    }
}