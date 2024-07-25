use rug::Integer;
use ndarray::Array2;

pub fn get_nth_fib_rug(n: u32) -> Integer {
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

fn matrix_power(matrix: &Array2<Integer>, power: u32) -> Array2<Integer> {
    if power == 1 {
        return matrix.clone();
    }

    let mut result = Array2::from_shape_vec((2, 2), vec![Integer::ONE.clone(), Integer::ZERO.clone(), Integer::ZERO.clone(), Integer::ONE.clone()]).unwrap();
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
    let mut product = Array2::from_shape_vec((2, 2), vec![Integer::from(0); 4]).unwrap();

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                product[[i, j]] += &a[[i, k]] * &b[[k, j]];
            }
        }
    }

    product
}
