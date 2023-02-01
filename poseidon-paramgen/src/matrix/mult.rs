use anyhow::{anyhow, Result};
use ark_ff::PrimeField;
use ark_std::{ops::Mul, vec::Vec};
use poseidon_parameters::MatrixOperations;

use crate::{Matrix, SquareMatrix};

/// Compute vector dot product
pub fn dot_product<F: PrimeField>(a: &[F], b: &[F]) -> F {
    if a.len() != b.len() {
        panic!("vecs not same len")
    }

    a.iter().zip(b.iter()).map(|(x, y)| *x * *y).sum()
}

/// Multiply two matrices
pub fn mat_mul<F: PrimeField, M: MatrixOperations<F> + MatrixOperations<F>>(
    lhs: &M,
    rhs: &M,
) -> Result<M> {
    if lhs.n_cols() != rhs.n_rows() {
        return Err(anyhow!(
            "matrix dimensions do not allow matrix multiplication"
        ));
    }

    let rhs_T = rhs.transpose();

    Ok(M::new(
        lhs.n_rows(),
        rhs.n_cols(),
        lhs.iter_rows()
            .flat_map(|row| {
                // Rows of the transposed matrix are the columns of the original matrix
                rhs_T
                    .iter_rows()
                    .map(|column| dot_product(row, column))
                    .collect::<Vec<F>>()
            })
            .collect(),
    ))
}

/// Multiply scalar by Matrix
impl<F: PrimeField> Mul<F> for Matrix<poseidon_parameters::Matrix<F>> {
    type Output = poseidon_parameters::Matrix<F>;

    fn mul(self, rhs: F) -> Self::Output {
        let elements = self.0.elements();
        let new_elements: Vec<F> = elements.iter().map(|element| *element * rhs).collect();
        poseidon_parameters::Matrix::new(self.0.n_rows, self.0.n_cols, new_elements)
    }
}

/// Multiply scalar by SquareMatrix
impl<F: PrimeField> Mul<F> for SquareMatrix<poseidon_parameters::SquareMatrix<F>> {
    type Output = poseidon_parameters::SquareMatrix<F>;

    fn mul(self, rhs: F) -> Self::Output {
        let elements = self.0.elements();
        let new_elements: Vec<F> = elements.iter().map(|element| *element * rhs).collect();
        poseidon_parameters::SquareMatrix::from_vec(new_elements)
    }
}
