#include<stdio.h>

void print_file(char *file_contents, size_t file_size) {
    for (size_t i = 0; i < file_size; i++)
        printf("%c", *(file_contents + i));
}