extern crate rand;

use rand::{thread_rng, Rng};

#[derive(Debug, Clone, PartialEq)]
enum Node {
    A,
    B,
    C,
    D,
    E,
    F,
}

use Node::*;

const N_NODES: usize = 6;
static AVAIL_NODES: [Node; N_NODES] = [A, B, C, D, E, F];

#[derive(Debug)]
struct Ant {
    memory: Vec<Node>,
    available: Vec<Node>,
}

impl Ant {
    fn new(init_node: &Node) -> Ant {
        let mut vec = Vec::new();
        vec.push(init_node.clone());
        let mut available: Vec<Node> = Vec::new();
        for node in &AVAIL_NODES {
            if *init_node != *node {
                available.push(node.clone());
            }
        }

        Ant {
            memory: vec,
            available: available,
        }
    }
}

#[derive(Debug)]
struct AntColony {
    rho: f64,
    alpha: f64,
    beta: f64,
    q: i32,
    q_0: f32,
    init_phero: f64,
    phi: f32,
    num_ant: usize,
    num_it: usize,
    distance: [[f32; N_NODES]; N_NODES],
    visibility: [[f64; N_NODES]; N_NODES],
    pheromone: [[f64; N_NODES]; N_NODES],
}

impl AntColony {
    fn new(rho: f64,
           alpha: f64,
           beta: f64,
           q: i32,
           q_0: f32,
           phi: f32,
           init_phero: f64,
           num_ant: usize,
           num_it: usize)
           -> AntColony {
        let distance = [[0.0, 12.0, 3.0, 23.0, 1.0, 5.0],
                        [12.0, 0.0, 9.0, 18.0, 3.0, 41.0],
                        [3.0, 9.0, 0.0, 89.0, 56.0, 21.0],
                        [23.0, 18.0, 89.0, 0.0, 87.0, 46.5],
                        [1.0, 3.0, 56.0, 87.0, 0.0, 55.0],
                        [5.0, 41.0, 21.0, 46.0, 55.0, 0.0]];

        let mut visibility = [[0.0; N_NODES]; N_NODES];
        let mut pheromone = [[0.0; N_NODES]; N_NODES];
        for y in 0..N_NODES {
            for x in 0..N_NODES {
                if x != y {
                    visibility[y][x] = 1.0 / distance[y][x] as f64;
                    pheromone[y][x] = init_phero as f64;
                }
            }
        }

        AntColony {
            rho: rho,
            alpha: alpha,
            beta: beta,
            q: q,
            q_0: q_0,
            phi: phi,
            init_phero: init_phero,
            num_ant: num_ant,
            num_it: num_it,
            distance: distance,
            visibility: visibility,
            pheromone: pheromone,
        }
    }

    fn compute_available_nodes(&self, ant: &mut Ant) {
        let current = ant.memory.last().unwrap();
        let mut to_remove = ant.available.len();
        for (indx, next) in ant.available.iter().enumerate() {
            if current == next {
                to_remove = indx;
                break;
            }
        }

        ant.available.remove(to_remove);
    }

    fn compute_next_node(&self, ant: &Ant) -> Node {
        let mut sum = 0.0;
        let current = ant.memory.last().unwrap();
        let mut vec_tnp = Vec::new();

        let mut rng = thread_rng();
        let q = rng.gen::<f32>();
        println!("Valor de q: {}", q);
        if q > self.q_0 {
            println!("Recorrido por Diversificación");
        } else {
            println!("Recorrido por Intensificación");
        }

        for next in &ant.available {
            let curr_indx = current.clone() as usize;
            let next_indx = next.clone() as usize;
            let t = self.pheromone[curr_indx][next_indx].powf(self.alpha);
            let n = self.visibility[curr_indx][next_indx].powf(self.beta);
            let t_n = t * n;
            sum += t_n;
            println!("{:?}-{:?}: t = {}\tn = {}\tt*n = {}",
                     current,
                     next,
                     t,
                     n,
                     t_n);
            vec_tnp.push(t_n);
        }

        let mut next_indx = 0;
        if q > self.q_0 {
            println!("Suma: {}", sum);

            for (indx, next) in ant.available.iter().enumerate() {
                let prob = vec_tnp[indx] / sum;
                println!("{:?}-{:?}: prob = {}", current, next, prob);
                vec_tnp[indx] = prob;
            }

            let mut rng = thread_rng();
            let num = rng.gen::<f64>();
            let mut s = 0.0;
            for (i, &prob) in vec_tnp.iter().enumerate() {
                s += prob;
                if num < s {
                    next_indx = i;
                    break;
                }
            }

            println!("Numero aleatorio para la Probabilidad: {}", num);
        } else {
            for (i, &t_n) in vec_tnp.iter().enumerate() {
                if t_n > vec_tnp[next_indx] {
                    next_indx = i;
                }
            }
        }
        ant.available[next_indx].clone()
    }

    fn get_path_cost(&self, path: &[Node]) -> f32 {
        let mut dist = 0.0;
        for i in 1..path.len() {
            dist += self.distance[path[i - 1].clone() as usize][path[i].clone() as usize];
        }
        dist
    }

    fn update_pheromone(&mut self, best_path: &(Vec<Node>, f32)) {
        for i in 0..N_NODES {
            for j in 0..N_NODES {
                if i != j {
                    self.pheromone[i][j] *= self.rho;
                    print!("{:?}-{:?}: Feromona = {}",
                           AVAIL_NODES[i],
                           AVAIL_NODES[j],
                           self.pheromone[i][j]);
                    let &(ref path, ref cost) = best_path;
                    let mut f_x = 0.0;
                    for x in 1..path.len() {
                        if (path[x] == AVAIL_NODES[i] && path[x - 1] == AVAIL_NODES[j]) ||
                           (path[x] == AVAIL_NODES[j] && path[x - 1] == AVAIL_NODES[i]) {
                            f_x = 1.0 / cost;
                        }
                    }
                    print!(" + {}", f_x);
                    self.pheromone[i][j] += f_x as f64;
                    println!(" = {}", self.pheromone[i][j]);
                }
            }
        }
    }

    fn ant_iteration(&mut self) {
        let mut best_global_ant: (Vec<Node>, f32) = (Vec::new(), 100000000.0);
        for iter in 0..self.num_it {
            println!("\nIteración {}", iter + 1);
            println!("Matriz Distancia\n{:?}", &self.distance);
            println!("Matriz Visibilidad\n{:?}", &self.visibility);
            println!("Matriz Feromona\n{:?}", &self.pheromone);

            let mut best_iter_ant: (Vec<Node>, f32) = (Vec::new(), 100000000.0);

            let mut vec_ants = Vec::new();
            for i in 0..self.num_ant {
                println!("Hormiga {}", i + 1);
                let start_node = E;
                let mut ant = Ant::new(&start_node);
                println!("Ciudad inicial: {:?}", start_node);
                for _ in 0..N_NODES - 1 {
                    let next = self.compute_next_node(&ant);
                    println!("Ciudad Siguiente: {:?}", next);
                    let first = self.pheromone[ant.memory.last().unwrap().clone() as usize][next.clone() as usize];
                    let second = self.init_phero;
                    let result = (1.0 - self.phi) as f64 * first + self.phi as f64 * second;
                    println!("Actualizamos el arco {:?}-{:?}(v): (1-e)*{} + e*{} = {}\n",
                             ant.memory.last().unwrap(),
                             next, first, second, result);
                    self.pheromone[ant.memory.last().unwrap().clone() as usize][next.clone() as usize] = result;
                    self.pheromone[next.clone() as usize][ant.memory.last().unwrap().clone() as usize] = result;
                    ant.memory.push(next);
                    self.compute_available_nodes(&mut ant);
                }
                println!("Hormiga {}: {:?}\n", i + 1, ant.memory);
                vec_ants.push(ant.memory);
            }

            let mut tupl_vec = Vec::new();
            for (indx, ant) in vec_ants.iter().enumerate() {
                let cost = self.get_path_cost(ant);
                println!("Hormiga {} {:?} - Costo: {}", indx + 1, ant, cost);
                if cost < best_iter_ant.1 {
                    best_iter_ant.0 = ant.clone();
                    best_iter_ant.1 = cost;
                }
                if cost < best_global_ant.1 {
                    best_global_ant.0 = ant.clone();
                    best_global_ant.1 = cost;
                }
                tupl_vec.push((ant, cost));
            }
            println!("----------\nMejor Hormiga Local: {:?} - Costo: {}\n----------",
                     best_iter_ant.0,
                     best_iter_ant.1);
            println!("----------\nMejor Hormiga Global: {:?} - Costo: {}\n----------",
                     best_global_ant.0,
                     best_global_ant.1);

            self.update_pheromone(&best_global_ant);
        }

        println!("----------\nMejor Hormiga Global: {:?} - Costo: {}\n----------",
                 best_global_ant.0,
                 best_global_ant.1);
    }
}

fn main() {
    let mut colony = AntColony::new(0.99, 1.0, 1.0, 1, 0.7, 0.05, 0.1, 3, 100);
    colony.ant_iteration();
}
