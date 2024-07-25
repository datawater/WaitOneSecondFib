mod bignum;
mod linear;
mod time_it;

fn main() {    
    let took;
    let n;

    time_it!(took, n, {
        linear::get_nth_fib(100_000)
    });

    println!("{took:?} {n}");
}
