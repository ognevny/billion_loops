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

To run benches yourself, you can use meson to build and run executables

```shell
$ meson setup build
$ meson compile -C build
$ meson test --benchmark -C build
```

To bench python scripts, you can use `time` command for Unix-like shells or `Measure-Command` for
PowerShell.

### Results for various OS

These tables are taken from CI output

Ubuntu (x86_64)

```
1/4 billion_loops:Bench C                 OK              2.25s
2/4 billion_loops:Bench C++               OK              2.26s
3/4 billion_loops:Bench Rust              OK              2.30s
4/4 billion_loops:Bench Rust with no_core OK              1.28s

# speedometer.py
real	0m30.785s
user	0m30.782s
sys	0m0.001s

# numbed.py
real	0m0.512s
user	0m0.709s
sys	0m0.037s
```

Ubuntu (aarch64)

```
1/4 billion_loops:Bench C                 OK              1.77s
2/4 billion_loops:Bench C++               OK              1.77s
3/4 billion_loops:Bench Rust              OK              1.77s
4/4 billion_loops:Bench Rust with no_core OK              1.77s

# speedometer.py
real	0m31.740s
user	0m31.738s
sys	0m0.002s

# numbed.py
real	0m0.405s
user	0m0.557s
sys	0m0.048s
```

MacOS (x86_64)

```
1/4 billion_loops:Bench C                 OK              1.46s
2/4 billion_loops:Bench C++               OK              1.50s
3/4 billion_loops:Bench Rust              OK              1.46s
4/4 billion_loops:Bench Rust with no_core OK              1.44s

# speedometer.py
real	1m46.605s
user	1m45.919s
sys	0m0.306s

# numbed.py
real	0m0.746s
user	0m0.628s
sys	0m0.102s
```

MacOS (aarch64)

```
1/4 billion_loops:Bench C                 OK              0.67s
2/4 billion_loops:Bench C++               OK              0.68s
3/4 billion_loops:Bench Rust              OK              0.66s
4/4 billion_loops:Bench Rust with no_core OK              0.35s

# speedometer.py
real	0m18.718s
user	0m18.499s
sys	0m0.049s

# numbed.py
real	0m0.629s
user	0m0.287s
sys	0m0.088s
```

Windows-gnu

```
1/4 billion_loops:Bench C                 OK              0.58s
2/4 billion_loops:Bench C++               OK              0.58s
3/4 billion_loops:Bench Rust              OK              0.64s
4/4 billion_loops:Bench Rust with no_core OK              0.64s

# speedometer.py
real	0m25.345s
user	0m0.031s
sys	0m0.015s

# numbed.py
real	0m0.641s
user	0m0.000s
sys	0m0.015s
```

Windows-gnullvm (x86_64)

```
1/4 billion_loops:Bench C                 OK              0.51s
2/4 billion_loops:Bench C++               OK              0.51s
3/4 billion_loops:Bench Rust              OK              0.96s
4/4 billion_loops:Bench Rust with no_core OK              0.46s

# speedometer.py
real	0m22.047s
user	0m0.000s
sys	0m0.000s

# numbed.py
real	0m0.664s
user	0m0.000s
sys	0m0.000s
```

Windows-gnullvm (aarch64)

```
1/4 billion_loops:Bench C                 OK              1.80s
2/4 billion_loops:Bench C++               OK              1.87s
3/4 billion_loops:Bench Rust              OK              1.79s
4/4 billion_loops:Bench Rust with no_core OK              1.79s

# speedometer.py
real	0m22.938s
user	0m0.015s
sys	0m0.000s

# numbed.py
real	0m0.718s
user	0m0.031s
sys	0m0.000s
```

Windows-msvc (x86_64)

```
1/4 billion_loops:Bench C                 OK              0.33s
2/4 billion_loops:Bench C++               OK              0.35s
3/4 billion_loops:Bench Rust              OK              0.66s
4/4 billion_loops:Bench Rust with no_core OK              0.64s

# speedometer.py
7
Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 35
Milliseconds      : 535
Ticks             : 355354164
TotalDays         : 0.000411289541666667
TotalHours        : 0.009870949
TotalMinutes      : 0.59225694
TotalSeconds      : 35.5354164
TotalMilliseconds : 35535.4164

# numbed.py
Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 0
Milliseconds      : 671
Ticks             : 6718508
TotalDays         : 7.77605092592593E-06
TotalHours        : 0.000186625222222222
TotalMinutes      : 0.0111975133333333
TotalSeconds      : 0.6718508
TotalMilliseconds : 671.8508
```

Windows-msvc (aarch64)

```
1/4 billion_loops:Bench C                 OK              1.79s
2/4 billion_loops:Bench C++               OK              1.79s
3/4 billion_loops:Bench Rust              OK              1.79s
4/4 billion_loops:Bench Rust with no_core OK              1.79s

# speedometer.py
Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 30
Milliseconds      : 436
Ticks             : 304365149
TotalDays         : 0.000352274478009259
TotalHours        : 0.00845458747222222
TotalMinutes      : 0.507275248333333
TotalSeconds      : 30.4365149
TotalMilliseconds : 30436.5149
```
