fn s() -> i32 {
    let mut n = 1;
    while n < 1_000_000_000 {
        n += 1;
    }
    n
}

fn main() {
    print!("{}", s());
}
