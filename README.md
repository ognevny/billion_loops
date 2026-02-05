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
1/4 billion_loops:Bench C                 OK              0.56s
2/4 billion_loops:Bench C++               OK              0.53s
3/4 billion_loops:Bench Rust              OK              0.94s
4/4 billion_loops:Bench Rust with no_core OK              0.45s

# speedometer.py
real	0m38.543s
user	0m38.533s
sys	0m0.007s

# numbed.py
real	0m0.560s
user	0m0.765s
sys	0m0.060s
```

Ubuntu (aarch64)

```
1/4 billion_loops:Bench C                 OK              1.77s
2/4 billion_loops:Bench C++               OK              1.77s
3/4 billion_loops:Bench Rust              OK              1.77s
4/4 billion_loops:Bench Rust with no_core OK              1.77s

# speedometer.py
real	0m32.853s
user	0m32.847s
sys	0m0.003s

# numbed.py
real	0m0.424s
user	0m0.574s
sys	0m0.051s
```

MacOS (x86_64)

```
1/4 billion_loops:Bench C                 OK              1.85s
2/4 billion_loops:Bench C++               OK              1.72s
3/4 billion_loops:Bench Rust              OK              1.72s
4/4 billion_loops:Bench Rust with no_core OK              1.69s

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
1/4 billion_loops:Bench C                 OK              0.78s
2/4 billion_loops:Bench C++               OK              0.85s
3/4 billion_loops:Bench Rust              OK              0.82s
4/4 billion_loops:Bench Rust with no_core OK              0.42s

# speedometer.py
real	0m26.407s
user	0m25.896s
sys	0m0.183s

# numbed.py
real	0m1.719s
user	0m0.552s
sys	0m0.239s
```

Windows-gnu

```
1/4 billion_loops:Bench C                 OK              0.59s
2/4 billion_loops:Bench C++               OK              0.58s
3/4 billion_loops:Bench Rust              OK              0.64s
4/4 billion_loops:Bench Rust with no_core OK              0.66s

# speedometer.py
real	0m58.413s
user	0m0.031s
sys	0m0.000s

# numbed.py
real	0m0.776s
user	0m0.000s
sys	0m0.000s
```

Windows-gnullvm (x86_64)

```
1/4 billion_loops:Bench C                 OK              0.51s
2/4 billion_loops:Bench C++               OK              0.51s
3/4 billion_loops:Bench Rust              OK              0.96s
4/4 billion_loops:Bench Rust with no_core OK              0.47s

# speedometer.py
real	0m20.623s
user	0m0.015s
sys	0m0.000s

# numbed.py
real	0m0.683s
user	0m0.015s
sys	0m0.000s
```

Windows-gnullvm (aarch64)

```
1/4 billion_loops:Bench C                 OK              1.81s
2/4 billion_loops:Bench C++               OK              1.82s
3/4 billion_loops:Bench Rust              OK              1.80s
4/4 billion_loops:Bench Rust with no_core OK              1.80s

# speedometer.py
real	0m22.762s
user	0m0.016s
sys	0m0.015s

# numbed.py
real	0m0.794s
user	0m0.015s
sys	0m0.015s
```

Windows-msvc (x86_64)

```
1/4 billion_loops:Bench C                 OK              0.36s
2/4 billion_loops:Bench C++               OK              0.34s
3/4 billion_loops:Bench Rust              OK              0.65s
4/4 billion_loops:Bench Rust with no_core OK              0.64s

# speedometer.py
Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 34
Milliseconds      : 515
Ticks             : 345153309
TotalDays         : 0.000399482996527778
TotalHours        : 0.00958759191666667
TotalMinutes      : 0.575255515
TotalSeconds      : 34.5153309
TotalMilliseconds : 34515.3309

# numbed.py
Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 0
Milliseconds      : 669
Ticks             : 6690935
TotalDays         : 7.74413773148148E-06
TotalHours        : 0.000185859305555556
TotalMinutes      : 0.0111515583333333
TotalSeconds      : 0.6690935
TotalMilliseconds : 669.0935
```

Windows-msvc (aarch64)

```
1/4 billion_loops:Bench C                 OK              1.81s
2/4 billion_loops:Bench C++               OK              1.80s
3/4 billion_loops:Bench Rust              OK              1.80s
4/4 billion_loops:Bench Rust with no_core OK              1.80s

# speedometer.py
Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 30
Milliseconds      : 489
Ticks             : 304892764
TotalDays         : 0.000352885143518519
TotalHours        : 0.00846924344444444
TotalMinutes      : 0.508154606666667
TotalSeconds      : 30.4892764
TotalMilliseconds : 30489.2764
```
