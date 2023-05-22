#include <stdio.h>

void bar() {
    printf("bar");
}

void foo() {
    printf("foo");
}

int main() {
    printf("Hello World!\n");
    foo();
    bar();
    printf("main is located at %#010x\n", &main);
    return 0;
}
