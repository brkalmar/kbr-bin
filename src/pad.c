#define _POSIX_C_SOURCE 2

#include <ctype.h>
#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <unistd.h>

#define EXIT_STATUS_ERR    0x01
#define EXIT_STATUS_ARG    0x02
#define EXIT_STATUS_MALLOC 0x40

#define OPTIONS "chlp:r"

static void print_usage(FILE *out)
{
	fprintf(out, "usage: pad [OPTION]... WIDTH [STRING]\n");
}

static void print_help(FILE *out)
{
	print_usage(out);
	fprintf(out, "\
Pad STRING to WIDTH with spaces and write it to stdout, with no trailing \
newline.\n\
\n\
Arguments:\n\
  WIDTH\tThe width of the padded string (non-negative integer).  If less than \
or equal to the width of STRING, STRING is output unchanged.\n\
  STRING\tThe string to be padded.  If not given, everything read from stdin \
is used.\n\
\n\
Options:\n\
  -c\tCenter align: pad on both sides.  If an odd number of padding characters \
are needed, use 1 more on the left than on the right.\n\
  -h\tPrint this help message and exit.\n\
  -l\tLeft pad: pad on the left.  This is the default.\n\
  -p CHAR\tUse CHAR (a single character) as padding instead of space.\n\
  -r\tRight pad: pad on the right.\n\
\n\
Options -c, -l and -r are mutually exclusive.\n\
");
}

enum pad_type {
	PAD_TYPE_CENTER,
	PAD_TYPE_LEFT,
	PAD_TYPE_RIGHT,
};

#define ARGS_PAD_TYPE_DEFAULT PAD_TYPE_LEFT
#define ARGS_PAD_CHAR_DEFAULT ' '

struct args {
	size_t width;

	char *string;
	size_t string_len;
	bool string_dynamic;

	enum pad_type pad_type;

	char pad_char;
};


static void args_error_clr(FILE *out)
{
	fprintf(out, "error: -c, -l and -r are mutually exclusive\n");
	exit(EXIT_STATUS_ARG);
}

static void args_error_p(FILE *out, const char *optarg)
{
	fprintf(out, "error: -p requires single character: '%s'\n", optarg);
	exit(EXIT_STATUS_ARG);
}

static void args_error_width(FILE *out, const char *arg)
{
	fprintf(out, "error: WIDTH must be a non-negative integer: '%s'\n",
	        arg);
	exit(EXIT_STATUS_ARG);
}

static void args_parse(struct args *args, int argc, char *argv[])
{
	args->pad_type = ARGS_PAD_TYPE_DEFAULT;
	bool pad_type_set = false;
	args->pad_char = ARGS_PAD_CHAR_DEFAULT;

	int c;
	while ((c = getopt(argc, argv, OPTIONS)) != -1) {
		switch (c) {
		case 'c':
			if (pad_type_set) {
				args_error_clr(stderr);
			}
			args->pad_type = PAD_TYPE_CENTER;
			pad_type_set = true;
			break;
		case 'h':
			print_help(stdout);
			exit(EXIT_SUCCESS);
			break;
		case 'l':
			if (pad_type_set) {
				args_error_clr(stderr);
			}
			args->pad_type = PAD_TYPE_LEFT;
			pad_type_set = true;
			break;
		case 'p':
			if (optarg[0] == '\0' || optarg[1] != '\0') {
				args_error_p(stderr, optarg);
			}
			args->pad_char = optarg[0];
			break;
		case 'r':
			if (pad_type_set) {
				args_error_clr(stderr);
			}
			args->pad_type = PAD_TYPE_RIGHT;
			pad_type_set = true;
			break;
		case '?':
			exit(EXIT_STATUS_ERR);
		}
	}

	args->string = NULL;

	switch (argc - optind) {
	case 2:
		args->string = argv[optind + 1];
		args->string_len = strlen(args->string);
		args->string_dynamic = false;
		// NOTE: fall-through
	case 1:
		if (argv[optind][0] == '\0' || isspace(argv[optind][0])) {
			args_error_width(stderr, argv[optind]);
		}
		char *end;
		long width = strtol(argv[optind], &end, 0);
		if (errno == ERANGE || *end != '\0' || width < 0) {
			args_error_width(stderr, argv[optind]);
		}
		args->width = width;
		break;
	default:
		print_usage(stderr);
		exit(EXIT_STATUS_ARG);
		break;
	}

	if (args->string == NULL) {
		size_t string_size = (args->width < 1024) ? args->width : 1024;
		args->string = malloc(string_size);
		if (args->string == NULL) {
			exit(EXIT_STATUS_MALLOC);
		}
		int c;
		size_t i;
		for (i = 0; (c = getc(stdin)) != EOF; i++) {
			if (i >= string_size) {
				string_size *= 2;
				args->string = realloc(args->string,
				                      string_size);
				if (args->string == NULL) {
					exit(EXIT_STATUS_MALLOC);
				}
			}
			args->string[i] = c;
		}
		args->string_len = i;
		args->string_dynamic = true;
	}
}

int main(int argc, char *argv[])
{
	int r = EXIT_SUCCESS;

	struct args args;
	args_parse(&args, argc, argv);

	if (args.width <= args.string_len) {
		fputs(args.string, stdout);
		goto cleanup;
	}

	const size_t padding = args.width - args.string_len;
	switch (args.pad_type) {
	case PAD_TYPE_LEFT:
		for (size_t i = 0; i < padding; i++) {
			putc(args.pad_char, stdout);
		}
		fputs(args.string, stdout);
		break;
	case PAD_TYPE_RIGHT:
		fputs(args.string, stdout);
		for (size_t i = 0; i < padding; i++) {
			putc(args.pad_char, stdout);
		}
		break;
	case PAD_TYPE_CENTER:
		;
		const size_t right = padding / 2;
		const size_t left = padding - right;
		for (size_t i = 0; i < left; i++) {
			putc(args.pad_char, stdout);
		}
		fputs(args.string, stdout);
		for (size_t i = 0; i < right; i++) {
			putc(args.pad_char, stdout);
		}
		break;
	}

cleanup:
	if (args.string_dynamic) {
		free(args.string);
	}
	return r;
}
