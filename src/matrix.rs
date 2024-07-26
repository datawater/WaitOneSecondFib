use ndarray::Array2;
use rug::Integer;

use rayon::prelude::*;

use std::sync::Mutex;

pub fn get_nth_fib_rug(n: u64) -> Integer {
    if n == 0 {
        return Integer::ZERO;
    } else if n == 1 {
        return Integer::ONE.clone();
    }

    let base_matrix = Array2::from(vec![
        [Integer::ONE.clone(), Integer::ONE.clone()],
        [Integer::ONE.clone(), Integer::ZERO],
    ]);

    let result_matrix = matrix_power(&base_matrix, n - 1);

    result_matrix[[0, 0]].clone()
}

fn matrix_power(matrix: &Array2<Integer>, power: u64) -> Array2<Integer> {
    if power == 1 {
        return matrix.clone();
    }

    let mut result = Array2::from_shape_vec(
        (2, 2),
        vec![
            Integer::ONE.clone(),
            Integer::ZERO.clone(),
            Integer::ZERO.clone(),
            Integer::ONE.clone(),
        ],
    )
    .unwrap();
    let mut base = matrix.clone();
    let mut exponent = power;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = matrix_multiply(&result, &base);
        }
        base = matrix_multiply(&base, &base);
        exponent /= 2;
    }

    result
}

fn matrix_multiply(a: &Array2<Integer>, b: &Array2<Integer>) -> Array2<Integer> {
    let product = Mutex::new(Array2::from_shape_vec((2, 2), vec![Integer::from(0); 4]).unwrap());

    (0..2).into_par_iter().for_each(|i| {
        (0..2).into_par_iter().for_each(|j| {
            let mut sum = Integer::from(0);
            for k in 0..2 {
                sum += &a[[i, k]] * &b[[k, j]];
            }
            let mut product = product.lock().unwrap();
            product[[i, j]] = sum;
        });
    });

    Mutex::into_inner(product).unwrap()
}
