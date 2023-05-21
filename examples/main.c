#include <stdio.h>

void bar() {
    printf("bar");
}

void foo() {
    printf("foo");
}

int main() {
    printf("Hello World!");
    foo();
    bar();
    return 0;
}
