// a programm to test speed of each language (C implementation)
// originally written for https://github.com/ognevny/my-code

#include <stdio.h>

static inline int sum(void) {
  int n = 1;
  while (n < 1000000000)
    n++;
  return n;
}

int main(void) {
  printf("%d", sum());
  return 0;
}
