#include <iostream>
#include "amino-lib.h"

int main() {
    process_file("practica01A_1.txt");
    process_file("practica01A_2.txt");

    init_hashmap(); //Necesario para traducir de forma inversa
    process_fasta_files();

    return 0;
}