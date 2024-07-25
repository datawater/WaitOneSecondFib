#![feature(portable_simd, bigint_helper_methods)]

mod linear;
mod matrix;
mod time_it;

fn main() {
    let took;
    let n;

    time_it!(took, n, matrix::get_nth_fib_rug(12_049_900 * 2));

    println!("{took:?} {n}");
}
