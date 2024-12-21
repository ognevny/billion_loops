#![feature(no_core)]
#![allow(non_camel_case_types)]
#![allow(internal_features)]
#![feature(lang_items)]
#![feature(arbitrary_self_types)]
#![no_core]

#[cfg(target_os = "linux")]
#[link(name = "c")]
unsafe extern {}
#[cfg(target_os = "macos")]
#[link(name = "System")]
unsafe extern {}
#[cfg(target_os = "windows")]
#[link(name = "kernel32")]
unsafe extern {}
#[cfg(target_os = "windows")]
#[link(name = "user32")]
unsafe extern {}
#[cfg(target_os = "windows")]
#[link(name = "msvcrt")]
unsafe extern {}

#[cfg(target_os = "linux")]
pub type c_char = i8;
#[cfg(not(target_os = "linux"))]
pub type c_char = u8;

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

fn s() -> i32 {
    let mut n: i32 = 0;
    while n != 1_000_000_000 {
        n = n + 1;
    }
    n
}

#[lang = "start"]
fn start<T>(_main: fn() -> T, _argc: isize, _argv: *const *const u8, _: u8) -> isize {
    unsafe { printf(b"%d" as *const u8 as *const c_char, s()) };
    0
}

fn main() {}
