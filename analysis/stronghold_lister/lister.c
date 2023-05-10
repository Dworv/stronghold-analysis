#include <stdio.h>
#include "cubiomes/finders.h"

int main()
{
    int mc = MC_1_16_1;

    for (uint64_t i=1; i<10000000; i++) 
    {
        printStrongholds(i);
    }

    return 0;
}

void printStrongholds(uint64_t seed) 
{
    StrongholdIter sh;
    Pos pos = initFirstStronghold(&sh, MC_1_16_1, seed);

    Generator g;
    setupGenerator(&g, MC_1_16_1, 0);
    applySeed(&g, DIM_OVERWORLD, seed);

    int i;
    for (i=0; i<3; i++)
    {
        if (nextStronghold(&sh, &g) <= 0)
            break;
        printf("%6d,%6d;", sh.pos.x, sh.pos.z);
    }
    printf("\n");
}