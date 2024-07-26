#![feature(portable_simd, bigint_helper_methods)]

mod binet;
mod fast_double;
mod linear;
mod matrix;
mod time_it;

use std::{
    io::{BufWriter, Write},
    time::SystemTime,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let took;
    let n;

    let limit = 120_000_000;
    time_it!(took, n, fast_double::get_nth_fib_rug(limit));

    let file_name = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => format!("logs/{limit}_{}.txt", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    eprintln!("Calculating took {took:?}. Writing results to {file_name}");

    let file = std::fs::File::create(file_name)?;
    let mut bufwriter = BufWriter::new(file);

    bufwriter.write_fmt(format_args!("{:?} {}", took, n))?;

    Ok(())
}
