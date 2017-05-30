enum TpsBranch {
    Seq(String),
    Tree(Box<TpsTree>),
    Empty,
}

use self::TpsBranch::*;

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
}

pub struct TpsTree {
    root: Box<TpsJoint>,
}

impl TpsTree {
    pub fn new() -> TpsTree {
        TpsTree { root: Box::new(TpsJoint::new()) }
    }
}