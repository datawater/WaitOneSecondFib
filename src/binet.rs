use rug::{ops::Pow, Float};

const BASE_PRECISION: u32 = 65536;

pub fn get_nth_fib_rug(n: u64) -> Float {
    let precision = BASE_PRECISION + (n as u32 / 10);
    let root_five = Float::with_val(precision, 5).sqrt();
    let phi = (root_five.clone() + Float::with_val(precision, 1)) / Float::with_val(precision, 2);
    let one_minus_phi = Float::with_val(precision, 1) - &phi;

    (phi.pow(n) - one_minus_phi.pow(n)) / root_five
}
