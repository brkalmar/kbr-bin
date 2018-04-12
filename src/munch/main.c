/*
 * Munch on mebibytes of memory.
 * Based on: http://www.linuxatemyram.com/play.html
 *
 * 2015  Bence Kalmar
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char **argv) {
    char *buffer;
    int max;
    int mem_mib;

    max = -1;
    if(argc > 1) {
        max = atoi(argv[1]);
    }

    mem_mib = 0;
    while ((buffer = malloc(1024 * 1024)) != NULL && mem_mib != max) {
        memset(buffer, 0, 1024 * 1024);
        mem_mib++;
        printf("%d MiB\n", mem_mib);
    }
    
    return 0;
}
