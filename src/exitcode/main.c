/* Exit with first argument as the exit code.
 * If this argument is not a number in the range 0-255 then exit with 0.
 */

#include <stdlib.h>
#include <errno.h>

int main(int argc, char *argv[]) {
    char *endp;
    int exit_code;

    /* No arguments or first argument is empty. */
    if (argc < 2 || *argv[1] == '\0') {
        return 0;
    }
    exit_code = (int)strtol(argv[1], &endp, 10);
    /* Invalid conversion or argument out of long range or not in rage 0-255. */
    if (*endp != '\0' || errno == ERANGE || exit_code < 0 || exit_code > 255) {
        return 0;
    }
    return exit_code;
}
