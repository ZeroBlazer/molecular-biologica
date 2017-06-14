extern crate rand;

use rand::{thread_rng, Rng};

fn get_hsp(w1: &String, w2: &String, match_scr: i32, mism_scr: i32, gap_scr: i32) -> i32 {
    let mut hsp = 0;

    let mut chars2 = w2.chars();
    for c1 in w1.chars() {
        let c2 = chars2.next().unwrap();
        hsp += if c1 == '_' || c2 == '_' {
            gap_scr
        } else if c1 == c2 {
            match_scr
        } else {
            mism_scr
        }
    }

    hsp
}

fn ver_crossover(s1: &mut String, s2: &mut String, crosspoint: usize) {
    let mut post_s1 = s1.split_off(crosspoint);

    let chars = s1.replace("_", "");

    let mut ch_indx = 0;
    let mut indx = 0;
    while ch_indx < chars.len() {
        if s2.chars().nth(indx).unwrap() == chars.chars().nth(ch_indx).unwrap() {
            ch_indx += 1;
        }
        indx += 1;
    }

    let mut post_s2 = s2.split_off(indx);
    *s1 += post_s2.as_ref();
    *s2 += post_s1.as_ref();
}

fn adjust_to_size(len: usize, s: &mut String) {
    let mut s_len = s.len();
    let mut rng = thread_rng();
    while s_len != len {
        let indx = rng.gen_range(0, s_len);
        if s_len > len {
            if s.chars().nth(indx).unwrap() == '_' {
                s.remove(indx);
            }
        } else {
            s.insert_str(indx, "_");
        }
        s_len = s.len();
    }
}

// pub trait Cromosome {
//     fn new(size: usize, input: Vec<String>) -> Cromosome;
//     fn random(&mut self);
//     fn fitness(&self) -> u32;
//     fn crossover(&mut self, parent2: &mut Cromosome, crosspoint: usize);
//     fn mutate(&mut self);
// }

#[derive(Debug)]
pub struct Cromosome {
    size: usize,
    genotype: Vec<String>,
}

impl Cromosome {
    pub fn new(size: usize, input: &Vec<&str>) -> Cromosome {
        let mut st_gen = Vec::new();
        for align in input.iter() {
            st_gen.push(String::from(*align));
        }

        let mut rng = thread_rng();
        for align in st_gen.iter_mut() {
            let mut current_len = align.len();
            while current_len < size {
                align.insert_str(rng.gen_range(0, current_len), "_");
                current_len = align.len();
            }
        }

        Cromosome {
            size: size,
            genotype: st_gen,
        }
    }

    pub fn fitness(&self) -> i32 {
        let mut fitness = 0;
        let size = self.genotype.len();
        for i in 0..size {
            for j in i+1..size {
                fitness += get_hsp(&self.genotype[i], &self.genotype[j], 3, 1, 0);
            }
        }

        fitness
    }

    pub fn crossover(&mut self, parent2: &mut Cromosome, crosspoint: usize) {
        let len = self.genotype.len();
        if len != parent2.genotype.len() {
            panic!("Can't cross this pair of Cromosomes");
        }

        for i in 0..len {
            ver_crossover(&mut self.genotype[i], &mut parent2.genotype[i], crosspoint);
            adjust_to_size(self.size, &mut self.genotype[i]);
            adjust_to_size(self.size, &mut parent2.genotype[i]);
        }
    }

    pub fn mutate(&mut self) {
        for seq in self.genotype.iter_mut() {
            print!("{} -> ", seq);
            let mut c_seq = seq.replace("_", "");
            adjust_to_size(self.size, &mut c_seq);
            *seq = c_seq;
            println!("{}", seq);
        }
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
            ruleta_vec: Vec::new(),
        }
    }

    pub fn generate(&mut self, input: &Vec<&str>) {
        println!("\nGenerando Población inicial");

        for _ in 0..self.pob_size {
            let mut crom = Cromosome::new(self.crom_size, input);
            self.population.push(crom);
        }

        for i in 0..self.pob_size {
            println!("{}) {:?}", i + 1, self.population[i].genotype);
        }
    }

    pub fn eval(&self) {
        println!("Evaluando Individuos");
        for i in 0..self.pob_size {
            println!("{}) {:?} - {}",
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
            println!("{}) {:?} - {} - {}",
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
            // size: self.population[parent1].size,
            size: self.crom_size,
            genotype: self.population[parent1].genotype.clone(),
        };

        let mut child2 = Cromosome {
            // size: self.population[parent2].size,
            size: self.crom_size,
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
        let mut new_population: Vec<(i32, Vec<String>)> = Vec::new();
        for i in 0..self.population.len() {
            let fitness = self.population[i].fitness();
            let gen = self.population[i].genotype.clone();
            println!("{}) {:?} - {}", i + 1, gen, fitness);
            new_population.push((fitness, gen));
        }

        new_population.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        self.population.clear();
        for i in 0..self.pob_size {
            self.population
                .push(Cromosome {
                        //   size: new_population[i].1.len(),
                          size: self.crom_size,
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

        Solver {
            population: population,
            iterations: iterations,
            crossover_prob: crossover_prob,
            crossover_point: crossover_point,
            mutation_prob: mutation_prob,
        }
    }

    pub fn evolve(&mut self, input: Vec<&str>) {
        self.population.generate(&input);

        for i in 0..self.iterations {
            println!("\nIteración: {}", i);
            self.population.eval();
            self.population.ruleta();
            let pob_size = self.population.pob_size / 2;
            for _ in 0..pob_size {
            self.population.crossover(self.crossover_prob, self.crossover_point);
            self.population.mutate(self.mutation_prob);
            }
            self.population.selection();
        }

        println!("\nIteración: {}", self.iterations);
        self.population.eval();

        println!("\nFin del Proceso");
    }
}

fn main() {
    let mut solver = Solver::new(6, 9, 10, 0.9, 3, 0.2);
    solver.evolve(vec!["AGTCATTAATTGCGGTTAG", "CCAATTGTAGTT", "AGCATTCGTAGTT", "GTTCAAGGGAGTT"]);
    // solver.evolve(vec!["ADSDAAC", "ADTTC", "ADTCD"]);
    
    // let mut cro1 = Cromosome::new(9, vec!["ADSDAAC", "ADTTC"]);
    // let mut cro2 = Cromosome::new(9, vec!["ADASAC", "AADTC"]);
    // println!("{:#?}\n{}", cro1, cro1.fitness());
    // println!("{:#?}\n{}", cro2, cro1.fitness());
    // cro1.mutate();
    // cro2.mutate();
    // println!("{:#?}\n{}", cro1, cro1.fitness());
    // println!("{:#?}\n{}", cro2, cro1.fitness());
    // println!("{:#?}", Cromosome::new(22, vec!["AGTCATTAATTGCGGTTAG", "CCAATTGTAGTT", "AGCATTCGTAGTT", "GTTCAAGGGAGTT"]));
}
