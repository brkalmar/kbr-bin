/**
 * Usage:
 *  synchsafe-int BYTES
 * 
 * Print the synchsafe integer represented by BYTES to stdout.
 *
 * A synchsafe integer is a string of bytes as defined in the informal standard:
 * id3v2.4.0-structure, section 6.2.
 *
 * Bence Kalmar
 */

#include <stdio.h>
#include <string.h>
#include <stdint.h>

const char *bytes_to_binary(uint64_t bytes, size_t len) {
    static char res[65];

    uint64_t bit;
    char *p = res;
    for (bit = (uint64_t)0x01 << (8 * len - 1); bit != 0; bit >>= 1) {
        *(p++) = (bytes & bit) ? '1' : '0';
    }
    *p = '\0';

    return res;
}

int main(int argc, char *argv[]) {

    if (argc < 2) {
        return 1;
    }

    int len;
    len = strlen(argv[1]);
    // 9 * 7 = 63
    if (len > 9) {
        return 2;
    }

    uint8_t byte;
    uint64_t integer = 0;
    int i;
    for (i = 0; i != len; ++i) {
        byte = argv[1][i];
        // only low 7 bits used
        integer <<= 7;
        integer |= (byte & 0x7F);
    }

    printf("%lu\n", integer);
    return 0;
}
