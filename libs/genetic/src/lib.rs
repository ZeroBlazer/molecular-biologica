extern crate rand;

static BINARY: [char; 2] = ['0', '1'];

#[derive(Debug)]
pub struct Cromosome {
    size: usize,
    genotype: Vec<char>,
}

impl Cromosome {
    pub fn new(size: usize) -> Cromosome {
        Cromosome {
            size: size,
            genotype: Vec::new()
        }
    }

    pub fn random(&mut self) {
        for i in 0..self.size {
            self.genotype.push(BINARY[1]);
        }
    }
}

#[derive(Debug)]
pub struct Population {
    pob_size: usize,
    crom_size: usize,
    population: Vec<Cromosome>,
}

impl Population {
    pub fn new(pob_size: usize, crom_size: usize) -> Population {
        Population {
            pob_size: pob_size,
            crom_size: crom_size,
            population: Vec::new()
        }
    }

    pub fn generate(&mut self) {
        println!("\nGenerando Población inicial");
        
        for i in 0..self.pob_size {
            let crom = Cromosome::new(self.crom_size);
            crom.random();
            self.population.push(crom);
        }

        for i in 0..self.pob_size {
            println!("{}) {}", i, self.population[i].genotype.into_iter().collect());
        }
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

        let population = Population::new(pob_size, crom_size);
        population.generate();
        
        Solver {
            population: population,
            iterations: iterations,
            crossover_prob: crossover_prob,
            crossover_point: crossover_point,
            mutation_prob: mutation_prob
        }
    }

    pub fn evolve(&self) {
        println!("\nFin del Proceso");
    }
}