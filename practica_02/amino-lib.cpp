#include "amino-lib.h"

void probabilities(char *path) {
    string input;
    ifstream i_chain(path);
    i_chain >> input;

    float p_A = 0,
          p_G = 0,
          p_C = 0,
          p_T = 0,
          p_Pur = 0,
          p_Pir = 0;
    for (auto& c: input) {
        switch (c)
        {
        case 'A':
            p_A++;
            break;
        case 'G':
            p_G++;
            break;
        case 'C':
            p_C++;
            break;
        case 'T':
            p_T++;
            break;
        default:
            break;
        }
    }

    size_t lngth = input.length();
    p_Pur = (p_A + p_G) / lngth;
    p_Pir = (p_C + p_T) / lngth;
    // cout << p_A << "\n" << p_G << "\n" << p_C << "\n" << p_T << "\n" << p_Pur << "\n" << p_Pir << "\n" << lngth;
    p_A /= lngth;
    p_G /= lngth;
    p_C /= lngth;
    p_T /= lngth;

    ofstream o_chain("EPuma_Practica02.txt");
    
    o_chain << p_A << "\n" << p_G << "\n" << p_C << "\n" << p_T << "\n" << p_Pur << "\n" << p_Pir;

    i_chain.close();
    o_chain.close();
    return;
}