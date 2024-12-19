// a programm to test speed of each language (C++ implementation)
// originally written for https://github.com/ognevny/my-code

#include <iostream>

static inline int s() {
  int n = 1;
  while (n < 1000000000)
    n++;
  return n;
}

int main() {
  std::cout << s();
  return 0;
}
