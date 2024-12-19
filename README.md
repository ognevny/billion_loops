## Where it all started

These codes are inspired by The Builder's [video](https://www.youtube.com/watch?v=VioxsWYzoJk) on
YouTube. In November 2024 someone (TODO: add link) counted time of work for program with billion
loops. I didn't see the program, but that could be something like `number = number + i`

## What programs do?

A function `s()` is defined. It takes no arguments, it only returns 32-bit int. Inside of the
function a variable `n` is created with value 1, then while loop increments it up too 1 billion.
Then the billion (function result) is printed in the terminal

## About implementations

C and C++ don't differ from each other. This code was compiled with clang with `-Weverything` so
it's just the perfect variant that could be

For Python there are basic implementation and implementation that uses numba module, which boosts
whole thing a lot

Rust also has 2 implementation, but the second one is very unique - that's why nightly compiler is
forced. Stable compilers don't support it! Read more in [relevant README](rust-no-core/README.md)

## Benchmarking

TODO
