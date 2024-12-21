This is like a "book" about this "billion loops" implementation. First of all you should understand
how it works.

#### Attributes block

```rust
#![feature(no_core)]
#![allow(non_camel_case_types)]
#![allow(internal_features)]
#![feature(lang_items)]
#![feature(arbitrary_self_types)]
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
#[cfg(target_os = "linux")]
#[link(name = "c")]
unsafe extern {}
#[cfg(target_os = "macos")]
#[link(name = "System")]
unsafe extern {}
#[cfg(target_os = "windows")]
#[link(name = "msvcrt")]
unsafe extern {}
```

To run the program the start point is needed. These start points are defined in core libraries. For
Linux it's `libc`, for Mac it's `System`, on Windows it's `msvcrt`.

#### c_char

```rust
#[cfg(all(not(windows), not(target_vendor = "apple"), any(target_arch = "aarch64")))]
pub type c_char = u8;
#[cfg(not(all(not(windows), not(target_vendor = "apple"), any(target_arch = "aarch64"))))]
pub type c_char = i8;
```

Properly define c_char type for `printf` function depending on OS.

#### Defining language items

```rust
#[lang = "sized"]
pub trait Sized {}

#[lang = "copy"]
pub trait Copy {}
impl Copy for i32 {}

#[lang = "add"]
pub trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

impl Add for i32 {
    type Output = i32;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self + rhs
    }
}

#[lang = "eq"]
pub trait PartialEq<Rhs = Self> {
    fn ne(&self, other: &Rhs) -> bool;
}

impl PartialEq for i32 {
    fn ne(&self, other: &Self) -> bool {
        *self != *other
    }
}
```

To work with any integer, `Copy` and `Sized` traits are rquired. To implement `Add` trait `impl
Copy for i32 {}` line is required, otherwise compilation fails with ICE. `PartialEq` is used for
`n != 1_000_000_000` so the only required method is `ne`. For each trait `#[lang_item]` is used so
compiler knows, that these are the same traits as from libcore.

#### Start function

```rust
#[lang = "start"]
fn start<T>(_main: fn() -> T, _argc: isize, _argv: *const *const u8, _: u8) -> isize {
    unsafe { printf(b"%d" as *const u8 as *const c_char, s()) };
    0
}
```

Defines a start point of program (and of course `#[lang_item]` is required). The only thing start
function does is printing a billion with `printf` function and returning 0 as a signal of
successful execution.

## TODO

Support Windows, BSDs and something more
