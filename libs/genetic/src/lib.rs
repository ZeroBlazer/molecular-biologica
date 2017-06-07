extern crate rand;

use rand::Rng;

static BINARY: [&str; 2] = ["0", "1"];

#[derive(Debug)]
pub struct Cromosome {
    size: usize,
    genotype: String,
}

impl Cromosome {
    pub fn new(size: usize) -> Cromosome {
        Cromosome {
            size: size,
            genotype: String::new()
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
                2u32.pow(exp-1)
            } else {
                0
            };
            exp -= 1;
        }

        fitness
    }
}

#[derive(Debug)]
pub struct Population {
    pob_size: usize,
    crom_size: usize,
    population: Vec<Cromosome>,
    ruleta_vec: Vec<f32>,
}

impl Population {
    pub fn new(pob_size: usize, crom_size: usize) -> Population {
        Population {
            pob_size: pob_size,
            crom_size: crom_size,
            population: Vec::new(),
            ruleta_vec: Vec::new()
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
            println!("{}) {} - {}", i+1, self.population[i].genotype, self.population[i].fitness());
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
            self.ruleta_vec.push((self.population[i].fitness() as f32 * 100.0) / sum_ruleta as f32);
        }

        for i in 0..self.pob_size {
            println!("{}) {} - {} - {}", i+1, self.population[i].genotype, self.population[i].fitness(), self.ruleta_vec[i]);
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

    pub fn crossover(&self, crossover_prob: f32, crossover_point: usize) {
        let parent1 = self.random_parent_index();
        let mut parent2 = self.random_parent_index();

        while parent2 == parent1 {
            let mut rng = rand::thread_rng();
            parent2 = rng.gen::<usize>() % self.pob_size;
        }

        println!("Padre: {}", parent1 + 1);
        println!("Madre: {}", parent2 + 1);
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
               mutation_prob: f32) -> Solver {
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
            mutation_prob: mutation_prob
        }
    }

    pub fn evolve(&mut self) {
        for i in 0..self.iterations+1 {
            println!("\nIteración: {}", i);
            self.population.eval();
            self.population.ruleta();
            self.population.crossover(self.crossover_prob, self.crossover_point);
            // self.population.mutate();
            // self.population.crossover();
            // self.population.mutate();
            // self.population.selection();
        }

        println!("\nFin del Proceso");
    }
}