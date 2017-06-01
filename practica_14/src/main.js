//Variables útiles
var number_of_nodes = 8,
    inf = 9999,
    minimizer = 40,
    maximizer = 999;

//Nodos: (Indice, Nombre)
// var nodes = [(1, 'C'),
//              (2, 'E'),
//              (3, 'F'),
//              (4, 'G'),
//              (5, 'H'),
//              (6, 'K'),
//              (7, 'L'),
//              (8, 'N')
//             ];

var nodes = ['C',
    'E',
    'F',
    'G',
    'H',
    'K',
    'L',
    'N'
];


function transform(letra) {
    if (letra == 'C')
        return 0;
    else if (letra == 'E')
        return 1;
    else if (letra == 'F')
        return 2;
    else if (letra == 'G')
        return 3;
    else if (letra == 'H')
        return 4;
    else if (letra == 'K')
        return 5;
    else if (letra == 'L')
        return 6;
    else if (letra == 'N')
        return 7;

}

//                                      Pesos del grafo
//                C    E    F    G    H    K    L    N
var weights = [[inf, 10, 20, inf, 30, 70, 10, 47],  //C
[10, inf, inf, 40, 60, 10, 5, inf],  //E
[20, inf, inf, 55, inf, inf, 10, 30],  //F
[inf, 40, 55, inf, 80, 90, inf, inf],  //G
[30, 60, inf, 80, inf, 73, 40, inf],  //H
[70, 10, inf, 90, 73, inf, inf, 60],  //K
[10, 5, 10, inf, 40, inf, inf, inf],  //L
[47, inf, 30, inf, inf, 60, inf, inf]   //N
];

//CLASE CROMOSOMA-------------------------------------------------------------------->
//Constructor, inicializa a un costo muy grande
var Cromosoma = function (cr_size, genotype) {
    this.size = cr_size;
    if (genotype)
        this.genotype = genotype;
    this.cost = inf; //Se crea el objeto con un costo muy grande
};

//Escoge genes aleatorios para el cromosoma
Cromosoma.prototype.random = function () {
    this.genotype = [];

    //Used para generar recorridos sin loops
    //   var used = [];
    //   for(var i = 0; i < this.size; i++)
    //     used[i] = false;

    //Generar recorrido
    //   for(var i = 0; i < this.size; i++) {
    //     pos = Math.floor(Math.random() * number_of_nodes);
    //     while(used[pos]) {
    //       pos = Math.floor(Math.random() * number_of_nodes);
    //     }

    this.genotype[0] = nodes[Math.floor(Math.random() * number_of_nodes)];
    for (var i = 1; i < this.size; i++) {
        pos = Math.floor(Math.random() * number_of_nodes);
        while (weights[transform(this.genotype[i - 1])][transform(nodes[pos])] >= inf)
            pos = Math.floor(Math.random() * number_of_nodes);
        this.genotype[i] = nodes[pos];
    }
}

//De acuerdo a la probabilidad muta un gen en el cromosoma
Cromosoma.prototype.mutate = function () {
    //La mutación
    //   var mut_pos = Math.floor(Math.random() * this.genotype.length);
    var mut_pos = 0;
    this.genotype[mut_pos] = nodes[Math.floor(Math.random() * number_of_nodes)];
    //Imprimir el genotipo
    c_gen = '';
    for (var j = 0; j < this.size; j++)
        c_gen += this.genotype[j];
    console.log(c_gen);
}

//Cruzamiento de dos Cromosomas
Cromosoma.prototype.crossover = function (parent2, crosspoint) {
    if (crosspoint > this.genotype.length) { //Si el crosspoint supera al tamaño del genotipo
        console.log("Not a valid crosspoint");
        return;
    }

    //Creamos los genotipos de los hijos
    var genotype_child1 = [],
        genotype_child2 = [];
    for (var i = 0; i < this.genotype.length; i++) {
        genotype_child1[i] = i < crosspoint ? this.genotype[i] : parent2.genotype[i];
        genotype_child2[i] = i < crosspoint ? parent2.genotype[i] : this.genotype[i];
    }

    //Imprime los genotipos de los hijos
    var c1_gen = '',
        c2_gen = '';
    for (var j = 0; j < this.genotype.length; j++) {
        c1_gen += genotype_child1[j];
        c2_gen += genotype_child2[j];
    }
    console.log(c1_gen);
    console.log(c2_gen);

    return [genotype_child1, genotype_child2];
}

//Calcula el costo del cromosoma
Cromosoma.prototype.getCost = function (node_origin, node_end) {
    var sum = 0;

    //Suma el costo del recorrido
    for (var i = 1; i < this.size; i++) {
        sum += weights[transform(this.genotype[i - 1])][transform(this.genotype[i])];
    }

    //Minimizamos el costo si el recorrido empieza en el origen y termina en end
    if (this.genotype[0] == node_origin)
        sum -= minimizer;
    else
        sum += maximizer;

    if (this.genotype[this.size - 1] == node_end)
        sum -= minimizer;
    else
        sum += maximizer;

    //   if(this.genotype[0] == node_origin && this.genotype[this.size - 1] == node_end)
    //     sum = 1;

    this.cost = sum;
    //   console.log("Costo: " + this.cost);
    return this.cost;
}

//CLASE POBLACIÓN-------------------------------------------------------------------->
var Poblacion = function (pb_size, cr_size) {
    //Guardamos datos para referencia
    this.cr_size = cr_size;
    this.pb_size = pb_size;

    //Crea una población Random
    console.log("\nGenerando Población inicial");
    this.population = [];
    while (pb_size--) {
        var cromosome = new Cromosoma(cr_size);
        cromosome.random();
        //     cromosome.getCost();
        //     console.log(cromosome.cost);
        this.population.push(cromosome);
    }

    //Imprime los cromosomas de la población inicial
    for (var i = 0; i < this.population.length; i++) {
        c_gen = '';
        for (var j = 0; j < this.population[i].size; j++)
            c_gen += this.population[i].genotype[j];
        console.log(i + 1 + ") " + c_gen);
    }
};

//Evalúa e imprime el fitness de cada individuo en la población
Poblacion.prototype.eval = function (node_origin, node_end, log_info) {
    if (log_info)
        console.log("Evaluando Individuos");
    for (var i = 0; i < this.population.length; i++) {
        c_gen = '';
        for (var j = 0; j < this.population[i].size; j++) {
            c_gen += this.population[i].genotype[j];
        }
        //     console.log("Evaluación de individuos con: " + node_origin + ", " + node_end);
        if (log_info)
            console.log(i + 1 + ") " + c_gen + " - " + this.population[i].getCost(node_origin, node_end));
        else
            this.population[i].getCost(node_origin, node_end);
    }
}

//Funcion ruleta
Poblacion.prototype.ruleta = function () {
    console.log("Selección de Individuos - Método de la Ruleta");
    var sum_ruleta = 0;
    this.ruleta_vect = [];

    for (var i = 0; i < this.population.length; i++) {
        sum_ruleta += 1 / this.population[i].cost;  // los pesos son inversamente proporcionales
    }

    for (var i = 0; i < this.population.length; i++) {
        this.ruleta_vect.push((1 / this.population[i].cost * 100) / sum_ruleta); //llenamos sus vec
    }

    //Imprime la población y sus valores en la ruleta
    for (var i = 0; i < this.population.length; i++) {
        c_gen = '';
        for (var j = 0; j < this.population[i].size; j++) {
            c_gen += this.population[i].genotype[j];
        }
        console.log(i + 1 + ") " + c_gen + " - " + this.population[i].cost + " -- " + this.ruleta_vect[i]);
    }
}

//Hace el crossover de los individuos
Poblacion.prototype.indexRandomParent = function () {
    var num = Math.random() * 100,
        s = 0,
        lastIndex = this.ruleta_vect.length - 1;

    for (var i = 0; i < lastIndex; ++i) {
        s += this.ruleta_vect[i];
        if (num < s)
            return i;
    }
    return lastIndex;
}

//Hace el crossover de los individuos
Poblacion.prototype.crossover = function (crossover_prob, crossover_point) {
    //Seleccionamos a los padres
    var parent1_indx = this.indexRandomParent(), //Seleccionados por la ruleta
        parent2_indx = this.indexRandomParent();
    //Verificar que los dos padres no sean iguales
    while (parent2_indx == parent1_indx)
        parent2_indx = Math.floor(Math.random() * this.population.length);

    console.log("Padre: " + (parent1_indx + 1));
    console.log("Madre: " + (parent2_indx + 1));

    //   //Imprime padres
    //   console.log(this.population[parent1_indx].genotype);
    //   console.log(this.population[parent2_indx].genotype);

    var child1,
        child2;
    if (Math.random() < crossover_prob) {
        console.log("Cruzamiento");
        //Cruzamiento de los padres
        var children = this.population[parent1_indx].crossover(this.population[parent2_indx], crossover_point),
            child1 = new Cromosoma(this.cr_size, children[0]),
            child2 = new Cromosoma(this.cr_size, children[1]);
    }
    else {
        console.log("Sin cruzamiento");
        child1 = new Cromosoma(this.cr_size, this.population[parent1_indx].genotype),
            child2 = new Cromosoma(this.cr_size, this.population[parent2_indx].genotype);
    }

    //Insertamos a los hijos a la población
    this.population.push(child1);
    this.population.push(child2);
}

//Muta a los individuos de acuerdo a la probabilidad de alcanzarlos
Poblacion.prototype.mutate = function (chance) {
    for (var i = 0; i < this.population.length; i++) {
        if (Math.random() > chance)
            return;
        console.log("Mutó " + (i + 1));
        this.population[i].mutate();
    }
}

//Evalúa e imprime el fitness de cada individuo en la población
Poblacion.prototype.selection = function (node_origin, node_end) {
    console.log("Selección de Siguiente Población");
    this.eval(node_origin, node_end, false);
    //Imprime toda la población
    for (var i = 0; i < this.population.length; i++) {
        c_gen = '';
        for (var j = 0; j < this.population[i].size; j++) {
            c_gen += this.population[i].genotype[j];
        }
        console.log(i + 1 + ") " + c_gen + " - " + this.population[i].cost);
    }

    //Ordenamos 
    this.population.sort(function (a, b) {
        return a.cost - b.cost;
    });

    //Seleccionamos a los más aptos
    this.population.length = this.pb_size;
}

//CLASE SOLVER que hará evolucionar nuestro algoritmo genético -------------------------------------->
var Solver = function (pb_size, cr_size, iterations, crossover_prob, crossover_point, mut_prob) {
    //Creamos una nueva población
    this.poblacion = new Poblacion(pb_size, cr_size);

    //Copiamos los parámetros útiles
    this.iterations = iterations;
    this.mut_prob = mut_prob;
    this.crossover_prob = crossover_prob;
    this.crossover_point = crossover_point;
};

//Función que realiza las iteraciones para evolucionar nuestro GA
Solver.prototype.evolve = function (node_origin, node_end) {
    var i = 0;
    for (i = 0; i < this.iterations; i++) {
        console.log("\n\nIteración: " + i);
        //     console.log("poblacion.eval de individuos con: " + node_origin + ", " + node_end);
        this.poblacion.eval(node_origin, node_end, true);
        this.poblacion.ruleta();
        this.poblacion.crossover(this.crossover_prob, this.crossover_point);
        this.poblacion.mutate(this.mut_prob);
        this.poblacion.crossover(this.crossover_prob, this.crossover_point);
        this.poblacion.mutate(this.mut_prob);
        this.poblacion.selection(node_origin, node_end);
    }
    console.log("\n\nIteración: " + i);
    this.poblacion.eval(node_origin, node_end, true);
}

function main() {
    console.log("\nrun:");

    //Parámetros de la ejecución
    var pob_size = 25,
        cromosoma_size = 5,
        iterations = 10,
        crossover_prob = 0.9,
        crossover_point = 4,
        mut_prob = 0.1;

    //Outputs
    console.log("Tamaño de la Población: " + pob_size);
    console.log("Tamaño de los Cromosomas: " + cromosoma_size);
    console.log("Cantidad de Iteraciones: " + iterations);
    console.log("Probabilidad de Cruzamiento: " + crossover_prob);
    console.log("Cruzamiento de un Punto - Punto " + crossover_point);
    console.log("Probabilidad de Mutación: " + mut_prob);
    console.log("Mutación Simple");

    var solver = new Solver(pob_size, cromosoma_size, iterations, crossover_prob, crossover_point, mut_prob);
    solver.evolve(nodes[2], nodes[3]);

    console.log("\n\nFin del Proceso");
}

main();

//node GA.js