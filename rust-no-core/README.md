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

#### Exporting core libs

```rust
#[cfg_attr(
    any(target_os = "linux", target_os = "openbsd", target_os = "freebsd"),
    link(name = "c")
)]
#[cfg_attr(target_os = "macos", link(name = "System"))]
#[cfg_attr(windows, link(name = "msvcrt"))]
unsafe extern "C" {}
#[cfg_attr(all(windows, target_env = "msvc"), link(name = "legacy_stdio_definitions"))]
unsafe extern "C" {}
```

To run the program the start point is needed. These start points are defined in core libraries. For
Linux and BSDs it's `libc`, for Mac it's `System`, on Windows it's `msvcrt`. Also to make `printf`
work on MSVC it's needed to link `legacy_stdio_definitions`.

#### c_char

```rust
#[cfg(all(not(windows), not(target_vendor = "apple"), target_arch = "aarch64"))]
pub type c_char = u8;
#[cfg(any(windows, target_vendor = "apple", not(target_arch = "aarch64")))]
pub type c_char = i8;
```

Properly define c_char type for `printf` function depending on OS.

#### Defining language items

```rust
#[lang = "sized"]
pub trait Sized {}

#[lang = "receiver"]
pub trait Receiver {}

#[lang = "copy"]
pub trait Copy {}
impl Copy for i32 {}

#[lang = "add"]
pub trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

impl Add for i32 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output { self + rhs }
}

#[lang = "eq"]
pub trait PartialEq<Rhs = Self> {
    fn ne(&self, other: &Rhs) -> bool;
}

impl PartialEq for i32 {
    #[inline]
    fn ne(&self, other: &Self) -> bool { *self != *other }
}
```

To work with any integer, `Copy` and `Sized` traits are rquired. To implement `Add` trait `impl
Copy for i32 {}` line is required, otherwise compilation fails with ICE. `PartialEq` is used for
`n != 1_000_000_000` so the only required method is `ne`. For each trait `#[lang_item]` is used so
compiler knows, that these are the same traits as from libcore. I don't know why but some machines
require `Receiver` item.

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
$ find build -type f -executable -exec dust -b {} \;
16K ┌── sanitycheckc.exe
16K ┌── sanitycheckcpp.exe
3.8M ┌── rusttest
8.0K ┌── bl_rs-no-core
16K ┌── bl_c
3.8M ┌── bl_rs
16K ┌── bl_cpp
```

"No core" binary is nearly x475 less than std binary! But what about to strip `bl_rs`?

```shell
$ strip build/bl_rs
$ dust -b build/bl_rs
352K ┌── bl_rs
```

It's still x44 bigger than "no core" one. So such method is great for making very small executables,
while it requires much more time to write though.

## Supported Platforms

| Arch/OS | Linux | Windows | Macos | OpenBSD | FreeBSD |
| ------- | ----- | ------- | ----- | ------- | ------- |
| Aarch64 | +-    | +-      | +     | +-      | +-      |
| x86_64  | +     | +       | +     | +-      | +       |

+: Supported and tested
+-: Probably supported, not tested
-: Not supported

## TODO

Test OpenBSD and probably some more OS
