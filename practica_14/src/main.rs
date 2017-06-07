extern crate genetic;

use genetic::Solver;

fn main() {
    let solver = Solver::new(4, 5, 30, 0.9, 3, 0.05);
    solver.evolve();
}
