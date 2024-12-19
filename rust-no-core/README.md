This is like a "book" about this "billion loops" implementation. First of all you should understand
how iy works.

First lines helps me to remove implicit dependency on std and core, enable internal features and
arbitrary_self_types.

Then there is a huge `extern` blocks, which link to OS-specific "main" library (libc.so on Linux,
for example). That's required to have a working _program_, otherwise there could be only a library
with a single function

Main part (funniest thing) happens after `extern`s. There I _explicitly_ tell the compiler what I
want to use in code. That's still require core library, but only needed things will be used.
`lang_items` is perma-unstable feature that gives a "superpower" for defining things that are
common for std and core. Also `printf` function is used from main library because there is no
`print!` and others obviously

In this case there are 4 things needed:

`Copy` and `Sized` is required because I work with integers, which are sized values and can be
safely copied. `Copy` has explicit impl for i32 to make `Add` trait possible

`Add` is used for `n = n + 1` part for obvious reasons

`PartialEq` is used because there is no easier way to stop loop rather than check
`n != 1_000_000_000` condition.

In the end there is `fn main() {}` which is here because it must be here. Actual main function is
`start`, because `lang_item = "start"` is put so compiler knows that program starts here. Function
signature that's just what rustc wants to see for start function.

## TODO

Support Windows, BSDs and something more

Better description
