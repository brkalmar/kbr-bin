#!/usr/bin/env python3

# Build the projects in each subdirectory of this script's directory, and create
# links to the built executables in the parent of the script's directory.

import logging
import os
from pathlib import Path
import subprocess
import sys

FILE = Path(__file__).resolve()
DIR = FILE.parent
DIR_PARENT = DIR.parent


def build_cargo():
    logging.info(f"building cargo {str(Path.cwd())!r}")
    try:
        process = subprocess.run(["cargo", "build", "--release"], check=True)
    except CalledProcessError as e:
        logging.error(f"cannot build: command failed {e}")
        sys.exit()
    return Path("target/release").resolve() / Path.cwd().name

def build_make():
    logging.info(f"building make {str(Path.cwd())!r}")
    try:
        process = subprocess.run(["make"], check=True)
    except CalledProcessError as e:
        logging.error(f"cannot build: command failed {e}")
        sys.exit()
    return Path(Path.cwd().name).resolve()

def build_project():
    if Path("Cargo.toml").resolve().exists():
        executable = build_cargo()
    elif Path("Makefile").resolve().exists():
        executable = build_make()
    else:
        logging.error(f"cannot build: unknown project type {str(Path.cwd())!r}")
        sys.exit()

    link = DIR_PARENT / executable.name
    logging.info(f"linking executable {str(link)!r} -> {str(executable)!r}")
    try:
        if link.is_symlink():
            link.unlink()
        link.symlink_to(executable)
    except OSError as e:
        logging.error(f"cannot link executable: {e}")
        sys.exit()


def main():
    logging.getLogger().setLevel(logging.INFO)
    for project in DIR.iterdir():
        if project == FILE:
            continue
        try:
            os.chdir(project)
        except OSError as e:
            logging.warning(f"ignoring file: {e}")
            continue
        build_project()

if __name__ == "__main__":
    try:
        main()
    except SystemExit as e:
        sys.exit(1)
