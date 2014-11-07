#!/bin/bash

# Display the number of files in the given directories.
#
# 2013-04-27 / 2014-02-25
# Bence Kalmar

exit_value=0
START_DIRECTORY="${PWD}"
SELF_NAME="$(basename "${0}")"

#### Functions

usage() {
    local HELP_MESSAGE="\
Prints the number of items (files & directories) in DIRECTORY.
If DIRECTORY is not given, uses the current working directory."
    local USAGE_MESSAGE="\
Usage:
  ${SELF_NAME} [DIRECTORY]..."

    case "${1}" in
	argument) 
	    echo "${SELF_NAME}: invalid arguments" >&2
	    echo "${USAGE_MESSAGE}" >&2
	    exit 1
	    ;;
	help)
	    echo -e "${USAGE_MESSAGE}\n"
	    echo "${HELP_MESSAGE}"
            exit 0
	    ;;
    esac
}

#### Parsing arguments

((${#} < 1)) && directories=("${START_DIRECTORY}")

while ((${#} > 0)); do
    if [[ -d "${1}" ]] && ls "${1}" &>/dev/null; then
	directories+=("${1}")
    else
	echo "${SELF_NAME}: cannot access directory: ${1}" >&2
	usage argument
    fi
    shift
done

if ((${#directories[@]} > 1)); then
    echo_directory="true"
else
    echo_directory="false"
fi

####

for i in "${directories[@]}"; do
    nr=$(ls -1A "${i}" | wc -l)
    if ${echo_directory}; then
	for ((j=0; j < $((6 - ${#nr})); j++)); do
	    echo -n " "
	done
    fi
    echo -n "${nr}"
    ${echo_directory} && echo -n " ${i}"
    echo
done

exit ${exit_value}
