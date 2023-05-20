#!/bin/bash

clear

cargo build

rm -f tester/main

gcc -L$(pwd)/target/debug -o tester/main tester/main.c -legl -lgles

./tester/main
