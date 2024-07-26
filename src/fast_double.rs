use rug::{Complete, Integer};

pub fn get_nth_fib_rug(n: u64) -> Integer {
    if n == 0 {
        return Integer::ZERO;
    }

    return fib(n).0;
}

fn fib(n: u64) -> (Integer, Integer) {
    if n == 0 {
        return (Integer::ZERO, Integer::ONE.clone());
    }

    let (a, b) = fib(n >> 1);
    let c = &a * ((&b << 1u32).complete() - &a);
    let d = (&a * &a).complete() + &b * &b;

    return if n & 1 == 0 {
        (c, d)
    } else {
        (d.clone(), c + d)
    };
}
