fn add_one(x: i16) -> i32 {
    (x + 1) as i32
}

fn factorial(n: u32) -> u32 {
    if dbg!(n <= 1) {
        dbg!(1)
    } else {
        dbg!(n * factorial(n - 1))
    }
}

fn main() {
    // 1. Add 1 by local function.
    let num: i8 = 10;
    println!("{} plus one is {}!", num, add_one(num as i16));

    // 2. Return factorial of `3` result with debugging.
    dbg!(factorial(3));
}
