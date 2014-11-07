/* Usage:
     padright STRING WIDTH [PADDING]

   Command-line tool to add padding on the right of a given string to expand it
   to a width.  The output is written to stdout.

   Arguments:
   STRING -- Must be the string to be padded.
   WIDTH -- Must be the width of the padded string.  It must be an integer.  If
   it is less than or equal to the length of STRING, the output is equivalent to
   STRING.
   PADDING -- If given, must be a string with length greater than 0. It will be
   repeatedly written enough times after STRING to achieve the given WIDTH.  If
   the last repetition of PADDING "overflows" the length of the padding, it is
   cut after the appropriate number of characters.  By default, it is a single
   space character: ' '.

   Examples:

   padright "hello" 8
     "hello   "

   padright "hello" 12 "abc"
     "helloabcabca"

   padright "hello" -9 "x"
     "hello"

   2014-01-03 / 2014-01-03
   AlbusDrachir  */

#include <stdio.h>
#include <stdarg.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>

static const char prefix[] = "padright: ";
static const char postfix[] = "\n";

/* Write 'message' to stderr, format it and '...' as if it they passed to
   'printf()'.  Other format operations may also be done to 'message'.  Exit
   with the exit code 'exit_code'.  */
void error(int exit_code, char *message, ...) {
    va_list ap;
    char fmt[sizeof(prefix) + strlen(message) + sizeof(postfix)];

    strcpy(fmt, prefix);
    strcat(fmt, message);
    strcat(fmt, postfix);
    va_start(ap, message);
    vfprintf(stderr, fmt, ap);
    va_end(ap);
    exit(exit_code);
}

/* Do the padding and write it to stdout */
void pad(char *string, int width, char *padding) {
    int string_len;
    int pad_len;
    int i;
    char *p;

    string_len = strlen(string);
    if (width <= string_len) {
	fputs(string, stdout);
	return;
    }
    fputs(string, stdout);
    p = padding;
    pad_len = width - string_len;
    for (i = 0; i < pad_len; i++) {
	putchar(*p++);
	if (*p == '\0') {
	    p = padding;
	}
    }
}

int main(int argc, char *argv[]) {
    int width;
    char *endp;
    char *padding;
    
    if (argc < 3 || argc > 4) {
	error(1, "invalid number of arguments: %d", argc - 1);
    }
    width = strtol(argv[2], &endp, 10);
    if (*endp != '\0' || errno == ERANGE) {
	/* printf("#endp %d\n", (int)*endp); */
	error(1, "invalid width: `%s'", argv[2]);
    }
    if (argc == 4) {
	if (strlen(argv[3]) == 0) {
	    error(1, "invalid padding: `%s'", argv[3]);
	}
	padding = argv[3];
    } else {
	padding = " ";
    }
    
    pad(argv[1], width, padding);
    exit(0);
}
