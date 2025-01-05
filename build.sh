#! /bin/sh

cd c
gcc -c print_file.c -o printer.o
gcc printer.o -shared -o libprinter.so

rm printer.o

mv libprinter.so ../target/release/

cd ..

cargo r --release