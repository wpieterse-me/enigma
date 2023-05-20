#!/bin/bash

clear

cargo build

rm -f tester/main

gcc -L$(pwd)/target/debug -o tester/main tester/main.c -lgl

LB_LIBRARY_PATH=$(pwd)/target/debug:$LD_LIBRARY_PATH ./tester/main
