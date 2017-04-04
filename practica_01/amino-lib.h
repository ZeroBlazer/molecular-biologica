#include <iostream>
#include <fstream>
#include <algorithm>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

enum aminoacid {
    U = 0,
    C = 1,
    A = 2,
    G = 3,
    UD
};

void read_file(char *path, string &input);

void save_file(const char *path, string &output);

void transcript(string &input);

aminoacid get_enum(char a);

char get_amin(aminoacid a);

void translation(string &input);

void process_file(char *path);

void init_hashmap();

void read_fasta_file(char *path, string &input);

void inv_translation(string &input);

void inv_transcript(string &input);

void process_fasta_files();