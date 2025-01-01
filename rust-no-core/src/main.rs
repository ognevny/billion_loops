#![allow(non_camel_case_types, internal_features, reason = "to remove annoying warnings")]
#![feature(arbitrary_self_types, lang_items, no_core)]
#![no_core]

#[cfg_attr(
    any(target_os = "linux", target_os = "openbsd", target_os = "freebsd"),
    link(name = "c")
)]
#[cfg_attr(target_os = "macos", link(name = "System"))]
#[cfg_attr(windows, link(name = "msvcrt"))]
unsafe extern "C" {}
#[cfg_attr(all(windows, target_env = "msvc"), link(name = "legacy_stdio_definitions"))]
unsafe extern "C" {}

#[cfg(all(not(windows), not(target_vendor = "apple"), target_arch = "aarch64"))]
pub type c_char = u8;
#[cfg(any(windows, target_vendor = "apple", not(target_arch = "aarch64")))]
pub type c_char = i8;

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> i32;
}

#[lang = "sized"]
pub trait Sized {}

#[lang = "receiver"]
pub trait Receiver {}

#[lang = "copy"]
pub trait Copy {}
impl Copy for i32 {}
impl Copy for bool {}

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

#[lang = "not"]
pub trait Not {
    type Output;

    #[must_use]
    fn not(self) -> Self::Output;
}

impl Not for bool {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output { !self }
}

#[lang = "eq"]
pub trait PartialEq<Rhs = Self> {
    #[must_use]
    fn eq(&self, other: &Rhs) -> bool;
}

impl PartialEq for i32 {
    #[inline]
    fn eq(&self, other: &Self) -> bool { *self == *other }
}

fn s() -> i32 {
    let mut n: i32 = 0;
    while !(n == 1_000_000_000) {
        n = n + 1;
    }
    n
}

#[lang = "start"]
fn start<T>(_main: fn() -> T, _argc: isize, _argv: *const *const u8, _: u8) -> isize {
    unsafe {
        printf(b"%d" as *const u8 as *const c_char, s());
    }
    0
}

fn main() {}
