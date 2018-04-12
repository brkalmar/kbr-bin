#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <dirent.h>
#include <unistd.h>

void* xmalloc(size_t size) {
    void* res = malloc(size);
    if (res != NULL) {
        return res;
    }
    fprintf(stderr, "malloc error\n");
    exit(10);
}

int main(int argc, char* argv[]) {
    --argc; ++argv;
    char** dirnames = argv;
    if (argc < 1) {
        dirnames = xmalloc((sizeof *dirnames) * 2);
        dirnames[0] = getcwd(NULL, 0);
        if (dirnames[0] == NULL) {
            fprintf(stderr, "error resolving CWD\n");
            return 1;
        }
        dirnames[1] = NULL;
    }

    const bool print_dirnames = argc > 1;
    const char* nr_fmt = print_dirnames ? "%6zu" : "%zu";

    for (; *dirnames != NULL; ++dirnames) {
        DIR* dir = opendir(*dirnames);
        if (dir == NULL) {
            fprintf(stderr, "error opening '%s': %s\n", *dirnames,
                    strerror(errno));
            return 1;
        }
        size_t nr = 0;
        struct dirent* ent;
        while ((ent = readdir(dir)) != NULL) {
            if (strcmp(ent->d_name, ".") == 0
                || strcmp(ent->d_name, "..") == 0) {
                continue;
            }
            ++nr;
        }
        printf(nr_fmt, nr);
        if (print_dirnames) {
            printf(" %s", *dirnames);
        }
        printf("\n");
    }
    return 0;
}
