#include "amino-lib.h"

#define A_size 4

char trans_table[A_size][A_size][A_size] = {
//*U* U    C    A    G  *C* U    C    A    G  *A* U    C    A    G  *G* U    C    A    G
   {{'F', 'F', 'L', 'L'}, {'S', 'S', 'S', 'S'}, {'Y', 'Y', '.', '.'}, {'C', 'C', '.', 'W'}}, //U
   {{'L', 'L', 'L', 'L'}, {'P', 'P', 'P', 'P'}, {'H', 'H', 'Q', 'Q'}, {'Q', 'Q', 'Q', 'Q'}}, //C
   {{'I', 'I', 'I', 'M'}, {'T', 'T', 'T', 'T'}, {'N', 'N', 'K', 'K'}, {'S', 'S', 'R', 'R'}}, //A
   {{'V', 'V', 'V', 'V'}, {'A', 'A', 'A', 'A'}, {'D', 'D', 'E', 'E'}, {'G', 'G', 'G', 'G'}}  //G
};

unordered_map<char, vector<aminoacid> > inv_table;

void read_file(char *path, string &input) {
    ifstream chain(path);
    chain >> input;
    chain.close();
    return;
}

void save_file(const char *path, string &output) {
    ofstream chain(path);
    chain << output;
    chain.close();
    return;
}

void transcript(string &input) {
    replace(input.begin(), input.end(), 'T', 'U');
    return;
}

aminoacid get_enum(char a) {
    switch(a) {
    case 'U':
        return U;
        break;
    case 'C':
        return C;
        break;
    case 'A':
        return A;
        break;
    case 'G':
        return G;
        break;
    default:
        cout << "Not valid" << endl;
    }
}

char get_amin(aminoacid a) {
    switch(a) {
    case U:
        return 'U';
        break;
    case C:
        return 'C';
        break;
    case A:
        return 'A';
        break;
    case G:
        return 'G';
        break;
    default:
        cout << "Not valid" << endl;
        return '0';
    }
}

void translation(string &input) {
    string trnsltd;
    aminoacid first = UD,
              second = UD,
              third = UD;
    auto it0 = input.begin(),
         itf = input.end();
    for(; it0 < itf; it0+=3) {
        first  = get_enum(*it0);    second = get_enum(*(it0+1));    third  = get_enum(*(it0+2));
        trnsltd.push_back(trans_table[first][second][third]);
    }
    input = trnsltd;
    return;
}

void process_file(char *path) {
    string file_name = "EPuma_";
    file_name += path;
    string input_chain;
    read_file(path, input_chain);
    cout << "Original chain:\t\t" << input_chain << endl;
    transcript(input_chain);
    cout << "Transcripted chain:\t" << input_chain << endl;
    translation(input_chain);
    cout << "Translated chain:\t" << input_chain << endl;
    save_file(file_name.c_str(), input_chain);
    cout << endl;
}

void init_hashmap() {
    vector<aminoacid> vec(3);
    for (size_t i = 0; i < A_size; i++) {
        for (size_t j = 0; j < A_size; j++) {
            for (size_t k = 0; k < A_size; k++) {
                vec[0] = static_cast<aminoacid>(i);
                vec[1] = static_cast<aminoacid>(j);
                vec[2] = static_cast<aminoacid>(k);
                inv_table.insert({trans_table[i][j][k], vec});
            }
        }
    }
}

void read_fasta_file(char *path, string &input) {
    ifstream chain(path);
    string buffer;
    getline(chain, buffer); //Ignore first line
    while(!chain.eof()) {
        getline(chain, buffer);
        input += buffer;
    }
    chain.close();
    return;
}

void inv_translation(string &input) {
    string i_trnsltd;
    for (auto& c: input) {
        auto ptr = inv_table.find(c);
        if(ptr != inv_table.end()) {
            i_trnsltd.push_back(get_amin((ptr->second)[0]));
            i_trnsltd.push_back(get_amin((ptr->second)[1]));
            i_trnsltd.push_back(get_amin((ptr->second)[2]));
        }
        else
            cout << c << ": Not in table" << endl;
    }
    input = i_trnsltd;
}

void inv_transcript(string &input) {
    replace(input.begin(), input.end(), 'U', 'T');
    return;
}

void process_fasta_file(char *path, string &input_chain) {
    read_fasta_file(path, input_chain);
    cout << "Original chain:\t\t" << input_chain << endl;
    inv_translation(input_chain);
    cout << "Inv-translated chain:\t" << input_chain << endl;
    inv_transcript(input_chain);
    cout << "Inv-transcripted chain:\t" << input_chain << endl;
    cout << endl;
}

void process_fasta_files() {
    string input_chain;
    ofstream chain("EPuma_Practica01_B.txt");

    process_fasta_file("1cq0.fasta.txt", input_chain);
    chain << input_chain << endl;
    
    input_chain.clear();
    process_fasta_file("1ron.fasta.txt", input_chain);
    chain << input_chain << endl;
    
    input_chain.clear();
    process_fasta_file("1j6z.fasta.txt", input_chain);
    chain << input_chain << endl;

    input_chain.clear();
    process_fasta_file("2v26.fasta.txt", input_chain);
    chain << input_chain << endl;
    
    chain.close();
    return;
}