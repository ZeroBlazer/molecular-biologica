#[derive(Debug)]
enum TpsBranch {
    Seq(String),
    Branch(Box<TpsJoint>),
}

use self::TpsBranch::*;
use super::align_seqs;

#[derive(Debug)]
pub struct TpsJoint {
    left: Option<TpsBranch>,
    right: Option<TpsBranch>,
    alignments: Vec<String>,
}

impl TpsJoint {
    pub fn new() -> TpsJoint {
        TpsJoint {
            left: None,
            right: None,
            alignments: Vec::new(),
        }
    }

    pub fn from_strings(sec_1: &str, sec_2: &str) -> TpsJoint {
        let aligns = align_seqs(String::from(sec_1), String::from(sec_2));

        TpsJoint {
            left: Some(Seq(String::from(sec_1))),
            right: Some(Seq(String::from(sec_2))),
            alignments: aligns,
        }
    }
}

#[derive(Debug)]
pub struct TpsTree {
    root: Option<Box<TpsJoint>>,
    // keys: HashMap<usize, >
}

impl TpsTree {
    pub fn new() -> TpsTree {
        TpsTree { root: None }
    }

    pub fn insert_joint(&mut self, joint: TpsJoint) {
        match self.root {
            Some(ref p_root) => {
                let mut left_child = TpsJoint {
                    left: p_root.left,
                    right: p_root.right,
                    alignments: p_root.alignments
                };
                // p_root = Box::new(new_root);
            }
            None => self.root = Some(Box::new(joint)),
        }
    }
}