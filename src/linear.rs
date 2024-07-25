use rug::Integer;

pub fn get_nth_fib_rug(n: u32) -> Integer {
    let mut a = Integer::ZERO;
    let mut b = Integer::ONE.clone();

    let mut c = b.clone();

    for _ in 0..(n - 1) {
        c = a + b.clone();
        a = b;
        b = c.clone();
    }

    return c;
}
