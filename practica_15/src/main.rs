extern crate rand;

use rand::Rng;

static BINARY: [&str; 2] = ["0", "1"];


pub trait Cromosome {
    fn new(size: usize, ) -> Cromosome;
    fn random(&mut self);
    fn fitness(&self) -> u32;
    fn crossover(&mut self, parent2: &mut Cromosome, crosspoint: usize);
    fn mutate(&mut self);
}



#[derive(Debug)]
pub struct Cromosome {
    size: usize,
    genotype: String,
}

impl Cromosome {
    pub fn new(size: usize) -> Cromosome {
        Cromosome {
            size: size,
            genotype: String::new(),
        }
    }

    pub fn random(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..self.size {
            self.genotype += BINARY[rng.gen::<usize>() % BINARY.len()];
        }
    }

    pub fn fitness(&self) -> u32 {
        let mut fitness = 0;
        let mut exp = self.genotype.len() as u32;

        for i in 0..self.genotype.len() {
            fitness += if self.genotype.chars().nth(i).unwrap() == '1' {
                2u32.pow(exp - 1)
            } else {
                0
            };
            exp -= 1;
        }

        fitness
    }

    pub fn crossover(&mut self, parent2: &mut Cromosome, crosspoint: usize) {
        if crosspoint > self.genotype.len() {
            println!("Not a valid crosspoint");
            return;
        }

        let remnant1 = self.genotype.split_off(crosspoint - 1);
        let remnant2 = parent2.genotype.split_off(crosspoint - 1);

        self.genotype += remnant2.as_ref();
        parent2.genotype += remnant1.as_ref();
    }

    pub fn mutate(&mut self) {
        println!("{}", self.genotype);
        let mut rng = rand::thread_rng();
        let pos = rng.gen::<usize>() % self.size;
        self.genotype.remove(pos);
        self.genotype
            .insert_str(pos, BINARY[rng.gen::<usize>() % BINARY.len()]);
        println!("{}", self.genotype);
    }
}

#[derive(Debug)]
pub struct Population<Cr> {
    pob_size: usize,
    crom_size: usize,
    population: Vec<Cr>,
    ruleta_vec: Vec<f32>,
}

impl Population {
    pub fn new(pob_size: usize, crom_size: usize) -> Population {
        Population {
            pob_size: pob_size,
            crom_size: crom_size,
            population: Vec::new(),
            ruleta_vec: Vec::new(),
        }
    }

    pub fn generate(&mut self) {
        println!("\nGenerando Población inicial");

        for _ in 0..self.pob_size {
            let mut crom = Cromosome::new(self.crom_size);
            crom.random();
            self.population.push(crom);
        }

        for i in 0..self.pob_size {
            println!("{}) {}", i + 1, self.population[i].genotype);
        }
    }

    pub fn eval(&self) {
        println!("Evaluando Individuos");
        for i in 0..self.pob_size {
            println!("{}) {} - {}",
                     i + 1,
                     self.population[i].genotype,
                     self.population[i].fitness());
        }
    }

    pub fn ruleta(&mut self) {
        println!("Selección de Individuos - Método de la Ruleta");
        let mut sum_ruleta = 0;
        self.ruleta_vec = Vec::new();

        for i in 0..self.pob_size {
            sum_ruleta += self.population[i].fitness();
        }

        for i in 0..self.pob_size {
            self.ruleta_vec
                .push((self.population[i].fitness() as f32 * 100.0) / sum_ruleta as f32);
        }

        for i in 0..self.pob_size {
            println!("{}) {} - {} - {}",
                     i + 1,
                     self.population[i].genotype,
                     self.population[i].fitness(),
                     self.ruleta_vec[i]);
        }
    }

    pub fn random_parent_index(&self) -> usize {
        let mut rng = rand::thread_rng();
        let num = rng.gen::<f32>() * 100.0;
        let mut s = 0.0;

        for i in 0..self.ruleta_vec.len() {
            s += self.ruleta_vec[i];
            if num < s {
                return i;
            }
        }

        self.ruleta_vec.len()
    }

    pub fn crossover(&mut self, crossover_prob: f32, crossover_point: usize) {
        let parent1 = self.random_parent_index();
        let mut parent2 = self.random_parent_index();

        while parent2 == parent1 {
            let mut rng = rand::thread_rng();
            parent2 = rng.gen::<usize>() % self.pob_size;
        }

        println!("Padre: {}", parent1 + 1);
        println!("Madre: {}", parent2 + 1);

        let mut child1 = Cromosome {
            size: self.population[parent1].size,
            genotype: self.population[parent1].genotype.clone(),
        };

        let mut child2 = Cromosome {
            size: self.population[parent2].size,
            genotype: self.population[parent2].genotype.clone(),
        };

        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < crossover_prob {
            println!("Cruzamiento");
            child1.crossover(&mut child2, crossover_point + 1);
        } else {
            println!("Sin cruzamiento");
        }

        self.population.push(child1);
        self.population.push(child2);
    }

    pub fn mutate(&mut self, chance: f32) {
        for i in 0..self.population.len() {
            let mut rng = rand::thread_rng();
            if rng.gen::<f32>() > chance {
                return;
            }
            println!("Mutó {}", i + 1);
            self.population[i].mutate();
        }
    }

    pub fn selection(&mut self) {
        println!("Selección de Siguiente Población");
        let mut new_population: Vec<(u32, String)> = Vec::new();
        for i in 0..self.population.len() {
            let fitness = self.population[i].fitness();
            let gen = self.population[i].genotype.clone();
            println!("{}) {} - {}", i + 1, gen, fitness);
            new_population.push((fitness, gen));
        }

        new_population.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        self.population.clear();
        for i in 0..self.pob_size {
            self.population
                .push(Cromosome {
                          size: new_population[i].1.len(),
                          genotype: new_population[i].1.clone(),
                      });
        }

        // println!("{:?}", new_population);
    }
}

#[derive(Debug)]
pub struct Solver {
    population: Population,
    iterations: usize,
    crossover_prob: f32,
    crossover_point: usize,
    mutation_prob: f32,
}

impl Solver {
    pub fn new(pob_size: usize,
               crom_size: usize,
               iterations: usize,
               crossover_prob: f32,
               crossover_point: usize,
               mutation_prob: f32)
               -> Solver {
        println!("run:");
        println!("Tamaño de la Población: {}", pob_size);
        println!("Tamaño de los Cromosomas: {}", crom_size);
        println!("Cantidad de Iteraciones: {}", iterations);
        println!("Probabilidad de Cruzamiento: {}", crossover_prob);
        println!("Cruzamiento de un Punto - Punto {}", crossover_point);
        println!("Probabilidad de Mutación: {}", mutation_prob);
        println!("Mutación Simple");

        let mut population = Population::new(pob_size, crom_size);
        population.generate();

        Solver {
            population: population,
            iterations: iterations,
            crossover_prob: crossover_prob,
            crossover_point: crossover_point,
            mutation_prob: mutation_prob,
        }
    }

    pub fn evolve(&mut self) {
        for i in 0..self.iterations {
            println!("\nIteración: {}", i);
            self.population.eval();
            self.population.ruleta();
            self.population
                .crossover(self.crossover_prob, self.crossover_point);
            self.population.mutate(self.mutation_prob);
            self.population
                .crossover(self.crossover_prob, self.crossover_point);
            self.population.mutate(self.mutation_prob);
            self.population.selection();
        }

        println!("\nIteración: {}", self.iterations);
        self.population.eval();

        println!("\nFin del Proceso");
    }
}

fn main() {
    let mut solver = Solver::new(6, 8, 100, 0.9, 3, 0.05);
    solver.evolve();
    println!("Hello, world!");
}
