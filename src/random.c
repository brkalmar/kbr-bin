/**
 * Usage:
 *   random A B
 *
 * Print a random integer between A and B, inclusive.
 *
 * A - lower limit integer, min. -1,000,000
 * B - upper limit integer, max. +1,000,000
 *
 * The random bytes are read from '/dev/urandom' & a modulo is applied to them,
 * thereby creating the number.  This means that the larger the range [A,B], the
 * less uniform the distribution of numbers is.
 */

#include <stdint.h>
#include <stdio.h>
#include <err.h>
#include <errno.h>
#include <stdlib.h>

const char* program_name = "random";

/**
 * Return random number in range [min,max] from '/dev/urandom'.
 */
int32_t random(int32_t min, int32_t max)
{
    uint64_t n;
    FILE* f;
    int32_t range;

    f = fopen("/dev/urandom", "r");
    if (f == NULL) {
        err(2, "could not open '/dev/urandom'");
    }
    fread(&n, 64 / 8, 1, f);
    fclose(f);
    range = max - min + 1;
    
    return (n % range) + min;
}

typedef struct {
    int32_t min;
    int32_t max;
} Args;

int64_t str_to_int(const char *s)
{
    int64_t res;
    char* endptr;
    
    errno = 0;
    res = strtol(s, &endptr, 10);
    if (errno != 0) {
        err(1, "could not convert to integer: '%s'", s);
    }
    if (*endptr != '\0') {
        errx(1, "could not convert to integer: '%s'", s);
    }

    return res;
}

Args parse_args(int argc, char* argv[])
{
    Args res;

    argc--;
    argv++;
    if (argc != 2) {
        errx(1, "incorrect number of arguments");
    }
    res.min = str_to_int(argv[0]);
    if (res.min < -1000000) {
        errx(1, "A cannot be less than %d: %d", -1000000, res.min);
    }
    res.max = str_to_int(argv[1]);
    if (res.max > +1000000) {
        errx(1, "B cannot be greater than %d: %d", 1000000, res.max);
    }
    if (res.min > res.max) {
        errx(1, "A cannot be larger than B");
    }

    return res;
}

int main(int argc, char* argv[])
{
    Args args;
    
    args = parse_args(argc, argv);
    printf("%d\n", random(args.min, args.max));
    return 0;
}
