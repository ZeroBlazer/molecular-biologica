extern crate genetic;

use genetic::Solver;

fn main() {
    let mut solver = Solver::new(6, 8, 100, 0.9, 3, 0.05);
    solver.evolve();
}
