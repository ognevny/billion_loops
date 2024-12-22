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

Ubuntu

```
1/4 Bench C                 OK              0.45s
2/4 Bench C++               OK              0.54s
3/4 Bench Rust              OK              0.94s
4/4 Bench Rust with no_core OK              0.94s

# speedometer.py
real	0m25.212s
user	0m25.200s
sys	0m0.011s
```

MacOS (x86_64)

```
1/4 Bench C                 OK              1.41s
2/4 Bench C++               OK              1.42s
3/4 Bench Rust              OK              1.36s
4/4 Bench Rust with no_core OK              1.31s

# speedometer.py
real	0m50.419s
user	0m50.116s
sys	0m0.180s

# numbed.py
real	0m0.688s
user	0m0.788s
sys	0m0.109s
```

MacOS (aarch64)

```
1/4 Bench C                 OK              0.63s
2/4 Bench C++               OK              0.64s
3/4 Bench Rust              OK              0.66s
4/4 Bench Rust with no_core OK              0.64s

# speedometer.py
real	0m21.721s
user	0m21.643s
sys	0m0.038s

# numbed.py
real	0m0.671s
user	0m0.250s
sys	0m0.078s
```

Windows-gnu

```
1/4 Bench C                 OK              0.56s
2/4 Bench C++               OK              0.58s
3/4 Bench Rust              OK              0.63s
4/4 Bench Rust with no_core OK              0.95s

# speedometer.py
real	0m55.740s
user	0m0.000s
sys	0m0.015s
```

Windows-gnullvm

```
1/4 Bench C                 OK              0.50s
2/4 Bench C++               OK              0.50s
3/4 Bench Rust              OK              0.62s
4/4 Bench Rust with no_core OK              0.66s

# speedometer.py
real	0m32.270s
user	0m0.000s
sys	0m0.000s
```

Windows-msvc

```
1/4 Bench C                 OK              0.33s
2/4 Bench C++               OK              0.33s
3/4 Bench Rust              OK              0.95s
4/4 Bench Rust with no_core OK              0.64s

# speedometer.py
Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 30
Milliseconds      : 830
Ticks             : 308306109
TotalDays         : 0.000356835774305556
TotalHours        : 0.00856405858333333
TotalMinutes      : 0.513843515
TotalSeconds      : 30.8306109
TotalMilliseconds : 30830.6109

# numbed.py
Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 0
Milliseconds      : 627
Ticks             : 6271566
TotalDays         : 7.25875694444444E-06
TotalHours        : 0.000174210166666667
TotalMinutes      : 0.01045261
TotalSeconds      : 0.6271566
TotalMilliseconds : 627.1566
```
