extern crate rand;

use rand::{thread_rng, Rng};

fn get_hsp(w1: &str, w2: &str, match_scr: i32, mism_scr: i32, gap_scr: i32) -> i32 {
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
    let post_s1 = s1.split_off(crosspoint);

    let chars = s1.replace("_", "");

    let mut ch_indx = 0;
    let mut indx = 0;
    while ch_indx < chars.len() {
        if s2.chars().nth(indx).unwrap() == chars.chars().nth(ch_indx).unwrap() {
            ch_indx += 1;
        }
        indx += 1;
    }

    let post_s2 = s2.split_off(indx);
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
    pub fn new(size: usize, input: &[String]) -> Cromosome {
        let mut st_gen = Vec::new();
        for align in input.iter() {
            st_gen.push(align.clone());
        }

        let mut rng = thread_rng();
        for align in &mut st_gen {
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
        for seq in &mut self.genotype {
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

    pub fn generate(&mut self, input: &[String]) {
        println!("\nGenerando Población inicial");

        for _ in 0..self.pob_size {
            let crom = Cromosome::new(self.crom_size, input);
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

    pub fn evolve(&mut self, input: Vec<String>) -> &Vec<String> {
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

        &self.population.population[0].genotype
    }
}

extern crate alignments;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::{BTreeMap, BTreeSet};
use alignments::{align_seqs, align_seqs_seq, align_alignments};

#[derive(Debug)]
struct Matrix {
    elems: BTreeMap<usize, BTreeMap<usize, f64>>,
    keys: BTreeSet<usize>,
}

impl Matrix {
    fn new() -> Matrix {
        Matrix {
            elems: BTreeMap::new(),
            keys: BTreeSet::new(),
        }
    }

    fn insert(&mut self, c1: &usize, c2: &usize, val: f64) {
        let mut ins_map = self.elems.entry(*c1).or_insert_with(BTreeMap::new);
        ins_map.insert(*c2, val);
        self.keys.insert(*c1);
        self.keys.insert(*c2);
    }

    fn len(&self) -> usize {
        self.keys.len()
    }

    fn get_d(&self, c1: &usize, c2: &usize) -> &f64 {
        match self.elems.get(c1) {
            Some(map) => {
                match map.get(c2) {
                    Some(val) => val,
                    None => &self.elems[c2][c1],
                }
            }
            None => {
                match self.elems.get(c2) {
                    Some(map) => {
                        match map.get(c1) {
                            Some(val) => val,
                            None => panic!("c1 wasn't found"),
                        }
                    }
                    None => panic!("c2 wasn't found"),
                }
            }
        }
    }

    fn s_calculation(&self, c: &usize) -> f64 {
        let mut keys = self.keys.clone();
        if keys.remove(c) {
            let mut sum = 0.0;
            for &key in &keys {
                match self.elems.get(c) {
                    Some(map) => {
                        match map.get(&key) {
                            Some(val) => sum += *val,
                            None => sum += self.elems[&key][c],
                        }
                    }
                    None => sum += self.elems[&key][c],
                }
            }
            sum / (keys.len() - 1) as f64
        } else {
            panic!("Key not found in matrix");
        }
    }

    fn m_calculation(&self, c1: &usize, c2: &usize) -> f64 {
        *self.get_d(c1, c2) as f64 - self.s_calculation(c1) - self.s_calculation(c2)
    }

    fn lowest_m(&self) -> (f64, usize, usize) {
        let mut iter = self.keys.iter();
        let mut key_1 = match iter.next() {
            Some(val) => val,
            None => panic!("No elements in matrix"),
        };

        let mut lowest = (999.9, *key_1, *key_1);

        for key_2 in iter {
            let m = self.m_calculation(key_1, key_2);
            println!("M_{:X}.{:X} = {}", key_1, key_2, m);

            if lowest.0 > m {
                lowest = (m, *key_1, *key_2);
            }
            key_1 = key_2;
        }

        lowest
    }

    fn join(&mut self, k1: &usize, k2: &usize, k: &usize) -> Matrix {
        if !(self.keys.contains(k1) || self.keys.contains(k2)) {
            panic!("Keys aren't in matrix");
        }
        /********************************************************************/
        let d_12_2 = *self.get_d(k1, k2) as f64 / 2.0;
        let s_1 = self.s_calculation(k1);
        let s_2 = self.s_calculation(k2);
        println!("S_{:X}.{:X} = {}", k1, k, d_12_2 + (s_1 - s_2) / 2.0);
        println!("S_{:X}.{:X} = {}", k2, k, d_12_2 + (s_2 - s_1) / 2.0);
        /********************************************************************/
        let mut elems = self.elems.clone();
        elems.remove(k1);
        elems.remove(k2);
        for val in elems.values_mut() {
            val.remove(k1);
            val.remove(k2);
        }

        let mut keys = self.keys.clone();
        keys.remove(k1);
        keys.remove(k2);
        let rem_keys = keys.clone();

        let mut ret_m = Matrix {
            elems: elems,
            keys: keys,
        };

        for key in rem_keys {
            let val = self.get_d(k1, &key) + self.get_d(k2, &key) - self.get_d(k1, k2);
            ret_m.insert(k, &key, val / 2.0);
        }

        ret_m
    }

    fn print_matrix(&self) {
        for (key, val) in &self.elems {
            println!("{:X}: {:?}", key, val);
        }
    }

    fn print_s_calculations(&self) {
        for &key in &self.keys {
            println!("S_{:X} = {}", key, self.s_calculation(&key));
        }
    }

    fn keys(&self) -> Vec<usize> {
        self.keys.iter().cloned().collect()
    }
}

#[derive(Debug)]
struct GuideTree {
    aligns: BTreeMap<usize, Vec<String>>,
}

impl GuideTree {
    fn new() -> GuideTree {
        GuideTree { aligns: BTreeMap::new() }
    }

    fn insert(&mut self, key: &usize, align: String) {
        self.aligns.insert(*key, vec![align]);
    }

    fn join(&mut self, k1: &usize, k2: &usize, kj: &usize) {
        /****************************************************************************/
        let mut join_align: Vec<String>;
        {
            let aligns_1 = &self.aligns[k1];
            let aligns_2 = &self.aligns[k2];

            join_align = if aligns_1.len() == 1 {
                if aligns_2.len() == 1 {
                    align_seqs(aligns_1[0].clone(), aligns_2[0].clone())
                } else {
                    align_seqs_seq(aligns_2.clone(), aligns_1[0].clone())
                }
            } else if aligns_2.len() == 1 {
                if aligns_1.len() == 1 {
                    align_seqs(aligns_1[0].clone(), aligns_2[0].clone())
                } else {
                    align_seqs_seq(aligns_1.clone(), aligns_2[0].clone())
                }
            } else {
                align_alignments(aligns_1.clone(), aligns_2.clone())
            };
        }
        let mut size = 0;
        for seq in &join_align {
            if seq.len() > size {
                size = seq.len();
            }
        }
        /****************************************************************************/
        let mut solver = Solver::new(4, size, 50, 0.9, 6, 0.2);
        join_align = solver.evolve(join_align).clone();
        /****************************************************************************/
        self.aligns.insert(*kj, join_align);
    }

    fn alignments(&self, k: &usize) -> &Vec<String> {
        &self.aligns[k]
    }
}

fn distance_score(align_1: &str, align_2: &str) -> f64 {
    let mut x_count = 0.0;
    let mut y_count = 0.0;
    let len = align_1.len();

    for i in 0..len {
        let c1 = align_1.chars().nth(i).unwrap();
        let c2 = align_2.chars().nth(i).unwrap();

        if c1 != '_' && c2 != '_' {
            // non-gap positions
            x_count += 1.0;
            if c1 == c2 {
                // identical positions
                y_count += 1.0;
            }
        }
    }

    // println!("x: {}, y: {}", x_count, y_count);

    1.0 - (y_count / x_count)
}

fn get_sequences(path: &str) -> Vec<String> {
    let mut ret_vec = Vec::new();
    let f = File::open(path).expect("Couldn't open file");
    let f = BufReader::new(f);
    for line in f.lines() {
        ret_vec.push(line.unwrap());
    }

    ret_vec
}

fn mult_seq_alignment(input: &[String]) {
    let mut matrix = Matrix::new();
    /****************************************************/
    for (i, seq_1) in input.iter().enumerate() {
        let mut j = i;
        for seq_2 in input.iter().skip(i + 1) {
            j += 1;
            if i != j {
                let aligns = align_seqs(seq_1.clone(), seq_2.clone());
                matrix.insert(&i, &j, distance_score(&aligns[0], &aligns[1]));
                println!("{}-{} > {:#?} - {}",
                         i,
                         j,
                         aligns,
                         distance_score(&aligns[0], &aligns[1]));
            }
        }
    }
    /******************** Guide Tree ********************/
    let mut g_tree = GuideTree::new();

    for (i, seq) in input.iter().enumerate() {
        g_tree.insert(&i, seq.clone());
    }
    /***************** Neighbor joining *****************/
    let mut letter = 193; // C1

    while matrix.len() - 2 > 0 {
        println!("");
        matrix.print_matrix();
        println!("");
        matrix.print_s_calculations();
        println!("");
        let (_, key_1, key_2) = matrix.lowest_m();
        println!("\nJoining: [{:X} and {:X}] -> {:X}", key_1, key_2, letter);
        matrix = matrix.join(&key_1, &key_2, &letter);
        g_tree.join(&key_1, &key_2, &letter);
        letter += 1;
    }
    matrix.print_matrix();
    let final_keys = matrix.keys();
    g_tree.join(&final_keys[0], &final_keys[1], &letter);
    println!("{:#?}", g_tree);
    /****************************************************/
    // let mut solver = Solver::new(8, 823, 100, 0.9, 6, 0.2);
    // let mut solver = Solver::new(8, 9, 100, 0.9, 6, 0.2);
    // solver.evolve(g_tree.alignments(&letter).clone());
    /****************************************************/
    let mut file = File::create("EPuma_Final_Alignments.txt").expect("Couldn't open write file");
    for align in g_tree.alignments(&letter) {
        write!(file, "{}\n", align).expect("Couldn't write in file");
    }
    // for align in solver.evolve(g_tree.alignments(&letter).clone()) {
    //     write!(file, "{}\n", align).expect("Couldn't write in file");
    // }
    /****************************************************/
    println!("Hello, world!");
}

fn main() {
    let input = get_sequences("input/MSA_16507.txt");
    // let input = get_sequences("input/test.txt");
    mult_seq_alignment(&input);
}
