This is like a "book" about this "billion loops" implementation. First of all you should understand
how it works.

#### Attributes block

```rust
#![allow(non_camel_case_types, internal_features, reason = "to remove annoying warnings")]
#![feature(arbitrary_self_types, lang_items, no_core)]
#![no_core]
```

These attributes enable 3 features: `no_core`[^1], `lang_items`[^2] and `arbitrary_self_types`[^3].
The first one is enabled to allow `#![no_core]`, which removes implicit linking of core and std. The
second one is enabled to allow manual specifying of each language items (without core there are not
any). The third one allows manual implementing of PartialEq. Also 2 warnings are disabled.

[^1]: https://doc.rust-lang.org/beta/unstable-book/language-features/no-core.html

[^2]: https://doc.rust-lang.org/beta/unstable-book/language-features/lang-items.html

[^3]: https://doc.rust-lang.org/beta/unstable-book/language-features/arbitrary-self-types.html

#### c_char

```rust
#[cfg(all(not(windows), not(target_vendor = "apple"), target_arch = "aarch64"))]
pub type c_char = u8;
#[cfg(any(windows, target_vendor = "apple", not(target_arch = "aarch64")))]
pub type c_char = i8;
```

Properly define c_char type for `printf` function depending on OS.

#### Exporting core libs

```rust
#[cfg_attr(
    any(target_os = "linux", target_os = "openbsd", target_os = "freebsd"),
    link(name = "c")
)]
#[cfg_attr(target_os = "macos", link(name = "System"))]
#[cfg_attr(all(windows, not(target_env = "msvc")), link(name = "msvcrt"))]
#[cfg_attr(
    all(windows, target_env = "msvc"),
    link(name = "msvcrt"),
    link(name = "legacy_stdio_definitions")
)]
unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> i32;
}
```

To run the program the start point is needed. These start points are defined in core libraries. For
Linux and BSDs it's `libc`, for Mac it's `System`, on Windows it's `msvcrt`. Also to make `printf`
work on MSVC it's needed to link `legacy_stdio_definitions`.

#### Defining language items

```rust
#[lang = "pointee_sized"]
pub trait PointeeSized {}

#[lang = "meta_sized"]
pub trait MetaSized: PointeeSized {}

#[lang = "sized"]
pub trait Sized: MetaSized {}

#[lang = "copy"]
pub trait Copy {}
impl Copy for i32 {}

#[lang = "add"]
pub trait Add<Rhs = Self> {
    type Output;

    #[must_use]
    fn add(self, rhs: Rhs) -> Self::Output;
}

impl Add for i32 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output { self + rhs }
}

#[lang = "eq"]
pub trait PartialEq<Rhs: PointeeSized = Self>: PointeeSized {
    #[must_use]
    fn eq(&self, other: &Rhs) -> bool;
}

impl PartialEq for i32 {
    #[inline]
    fn eq(&self, other: &Self) -> bool { *self == *other }
}
```

To work with any integer, `Copy`, `Sized`, `MetaSized` and `PointeeSized` traits are rquired. To
implement `Add` trait `impl Copy for i32 {}` line is required, otherwise compilation fails with ICE.
`PartialEq` is used for `n != 1_000_000_000` so the only required method is `ne`. For each trait
`#[lang_item]` is used so compiler knows, that these are the same traits as from libcore. I don't
know why but some machines require `Receiver` item.

#### Start function

```rust
#[lang = "start"]
fn start<T>(_main: fn() -> T, _argc: isize, _argv: *const *const u8, _: u8) -> isize {
    unsafe {
        printf(b"%d" as *const u8 as *const c_char, s());
    }
    0
}
```

Defines a start point of program (and of course `#[lang_item]` is required). The only thing start
function does is printing a billion with `printf` function and returning 0 as a signal of
successful execution.

## Performance boost?

According to the [table](../README.md#Benchmarking) this code isn't faster than the std one. But
look at binary size (running on my Arch machine)

```shell
$ find build -type f -executable -exec stat -c '%n: %s' {} \;
...
build/bl_c: 15480
build/bl_rs-no-core: 5592
build/bl_rs: 3982480
build/bl_cpp: 15648
```

"No core" binary is nearly x712 less than std binary! But what about to strip `bl_rs`?

```shell
$ strip build/bl_rs
$ stat -c '%n: %s' build/bl_rs
build/bl_rs: 357664
```

It's still almost x64 bigger than "no core" one. So such method is great for making very small
executables, while it requires much more time to write though.

Also you can use some compiler flags to reduce file size, so try to use them

```shell
$ time rustc -Cpanic=abort -Coverflow-checks=false -Cdebuginfo=0 -Cstrip=symbols -Copt-level=s \
-Clto=fat -Ccodegen-units=1 -Zlocation-detail=none -Zfmt-debug=none rust/src/main.rs -o build/bl_rs

________________________________________________________
Executed in    4.16 secs    fish           external
   usr time    3.95 secs    1.09 millis    3.95 secs
   sys time    0.15 secs    0.33 millis    0.15 secs
```

Four seconds to compile is wild, but what is the result?

```shell
$ stat -c '%n: %s' build/bl_rs
build/bl_rs: 310688
```

55 times bigger than no_core one... and there is nothing else to do with that! That's actually a
huge disadvantage of rustc, it produces huge binaries, and the only way to fix it is forget about
std and even core.

In case you really don't want to spend time on learning how to write code with `#![no_core]`, there
is another approach to reduce binary size: using `-Zbuild-std`. Of course it requires some extra
options, so what is the final cargo command to build bl_rs:

```shell
$ RUSTFLAGS='-Copt-level=s -Ccodegen-units=1 -Zlocation-detail=none -Zfmt-debug=none' cargo b \
-Zbuild-std=std,panic_abort -Zbuild-std-features=optimize_for_size,panic_immediate_abort -p bl_rs
```

And the binary has a size of 24656 bytes, which is not ideal (compared to C and C++), but good
enough to go

By the way, what if to do the same with `bl_rs-no-core`? It will have a size of 4552 bytes, which is
actually achieved by stripping binary.

## Supported Platforms

| Arch/OS | Linux | Windows | Macos | OpenBSD | FreeBSD |
| ------- | ----- | ------- | ----- | ------- | ------- |
| Aarch64 | +     | +       | +     | +-      | +-      |
| x86_64  | +     | +       | +     | +-      | +       |

+: Supported and tested
+-: Probably supported, not tested
-: Not supported

## TODO

Test OpenBSD and probably support more OS
