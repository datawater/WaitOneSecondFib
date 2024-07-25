pub use bnum::BIntD32 as BigNum;

pub fn get_nth_fib(n: u32) -> BigNum<16392> {
    let mut a = BigNum::ZERO;
    let mut b = BigNum::ONE;

    let mut c = b;

    for _ in 0..n {
        c = a + b;
        a = b;
        b = c;
    }

    return c;
}