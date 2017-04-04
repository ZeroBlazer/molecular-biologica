#include <iostream>
#include <fstream>
#include <string>

using namespace std;

void process_file(char *path, size_t cntr[5]) {
    ifstream inf(path);
    string buff;
    inf >> buff;
    
    cntr[0] = 0,
    cntr[1] = 0,
    cntr[2] = 0,
    cntr[3] = 0;
    cntr[4] = buff.length();

    // cout << buff << ": ";
    int i = 0;
    for(c: buff) {
        switch(c) {
        case 'A':
            cntr[0]++;
            break;
        case 'T':
            cntr[1]++;
            break;
        case 'C':
            cntr[2]++;
            break;
        case 'G':
            cntr[3]++;
            break;
        default:
            cout << "Not a valid character!: " << c << " @ "<< i << " " << path << endl;
            break;
        }
        i++;
    }
    // cout << "\t" << cntr[0] << "\t" <<  cntr[1] << "\t" <<  cntr[2] << "\t" <<  cntr[3] << "\t" << cntr[4];
    cout << endl;
}

void probal_func() {
    size_t  cntr_1[5],
            cntr_2[5],
            cntr_3[5],
            cntr_4[5];

    float   prob_A = 0,
            prob_T = 0,
            prob_C = 0,
            prob_G = 0; 

    process_file("practica_03A_1.txt", cntr_1);
    process_file("practica_03A_2.txt", cntr_2);
    process_file("practica_03A_3.txt", cntr_3);
    process_file("practica_03A_4.txt", cntr_4);

    prob_A =    cntr_1[0] * 0.25 / cntr_1[4] +
                cntr_2[0] * 0.25 / cntr_2[4] +
                cntr_3[0] * 0.25 / cntr_3[4] +
                cntr_4[0] * 0.25 / cntr_4[4];

    prob_T =    cntr_1[1] * 0.25 / cntr_1[4] +
                cntr_2[1] * 0.25 / cntr_2[4] +
                cntr_3[1] * 0.25 / cntr_3[4] +
                cntr_4[1] * 0.25 / cntr_4[4];
    
    prob_C =    cntr_1[2] * 0.25 / cntr_1[4] +
                cntr_2[2] * 0.25 / cntr_2[4] +
                cntr_3[2] * 0.25 / cntr_3[4] +
                cntr_4[2] * 0.25 / cntr_4[4];
    
    prob_G =    cntr_1[3] * 0.25 / cntr_1[4] +
                cntr_2[3] * 0.25 / cntr_2[4] +
                cntr_3[3] * 0.25 / cntr_3[4] +
                cntr_4[3] * 0.25 / cntr_4[4];
    
    ofstream outf("EPuma_Practica03_A.txt");
    outf << prob_A << endl << prob_T << endl << prob_C << endl << prob_G << endl;
    outf.close();

////END OF FIRST PART

    process_file("practica_03B_1.txt", cntr_1);
    process_file("practica_03B_2.txt", cntr_2);
    process_file("practica_03B_3.txt", cntr_3);
    process_file("practica_03B_4.txt", cntr_4);

    prob_A =    cntr_1[0] * 0.25 / cntr_1[4] +
                cntr_2[0] * 0.25 / cntr_2[4] +
                cntr_3[0] * 0.25 / cntr_3[4] +
                cntr_4[0] * 0.25 / cntr_4[4];
    
    float   prob_A1 = 0,
            prob_A2 = 0,
            prob_A3 = 0,
            prob_A4 = 0; 

    prob_A1 = cntr_1[0] * 0.25 / cntr_1[4] / prob_A;
    prob_A2 = cntr_2[0] * 0.25 / cntr_2[4] / prob_A;
    prob_A3 = cntr_3[0] * 0.25 / cntr_3[4] / prob_A;
    prob_A4 = cntr_4[0] * 0.25 / cntr_4[4] / prob_A;
    
    
    ofstream outf2("EPuma_Practica03_B.txt");
    outf2 << prob_A1 << endl << prob_A2 << endl << prob_A3 << endl << prob_A4 << endl;
    outf2.close();
}

int main() {
    probal_func();
    return 0;
}