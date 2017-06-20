#[derive(Debug, Display)]
enum Nodes {
    A,
    B,
    C,
    D,
    E,
}

#[derive(Debug)]
struct Ant {
    memory: [Nodes; 5],
}

#[derive(Debug)]
struct AntColony {
    rho: f32,
    alpha: f32,
    beta: f32,
    q: i32,
    num_ant: usize,
    num_it: usize,
    visibility: [[f32; 5]; 5],
    pheromone: [[f32; 5]; 5],
}

impl AntColony {
    fn new(rho: f32, alpha: f32, beta: f32, q: i32, init_phero: f32, num_ant: usize, num_it: usize) -> AntColony {
        let distance = [[0.0, 12.0, 3.0, 23.0, 1.0],
                                [12.0, 0.0, 9.0, 18.0, 3.0],
                                [3.0, 9.0, 0.0, 89.0, 56.0],
                                [23.0, 18.0, 89.0, 0.0, 87.0],
                                [1.0, 3.0, 56.0, 87.0, 0.0]];
        
        let mut visibility = [[0.0; 5]; 5];
        let mut pheromone = [[0.0; 5]; 5];
        for y in 0..5 {
            for x in 0..5 {
                if x != y {
                    visibility[y][x] = 1.0 / distance[y][x];
                    pheromone[y][x] = init_phero;
                }
            }
        }

        AntColony {
            rho: rho,
            alpha: alpha,
            beta: beta,
            q: q,
            num_ant: num_ant,
            num_it: num_it,
            visibility: visibility,
            pheromone: pheromone
        }
    }

    // fn print_matrix(matrix: &[[f32; 5]; 5], descr: &str) {
    //     println!("{}", descr);
    //     for y in 0..5 {
    //         for x in 0..5 {
    //             print!("{}\t", matrix[y][x]);
    //         }
    //         println!("");
    //     }
    // }

    fn ant_iteration(&self) {
        // print_matrix(&SCORES, "Matriz Distancia");
            // print_matrix(&self.visibility, "Matriz Visibilidad");
            // print_matrix(&self.pheromone, "Matriz Feromona");
        for i in 0..self.num_ant {
            let mut ant = Ant {
                memory: 
            }
        }
    }
}

fn main() {
    let colony = AntColony::new(0.99, 1.0, 1.0, 1, 0.1, 3, 100);
    // rho: f32, alpha: f32, beta: f32, q: i32, init_phero: f32, num_ant: usize, num_it: usize
    // println!("{:#?}", colony);
}
