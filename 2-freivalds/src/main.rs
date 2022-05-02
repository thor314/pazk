#![allow(non_snake_case)]
use nalgebra::{SMatrix, SVector};
use rand::{self, Rng};

const SIZE: usize = 7;
const START: usize = 0;
const STOP: usize = 10;
type Matrix = SMatrix<usize, SIZE, SIZE>;
type Vector = SVector<usize, SIZE>;

fn main() {
    let A = random_n_by_n();
    let B = random_n_by_n();

    println!("V: hey prover, multiply {:?} and {:?}", A, B);
    let C = honest_mat_mul(A, B);
    // let C = dishonest_mat_mul(A,B);
    println!("P: claim AB={:?}", C);

    println!("V: Verifying ABx = Cx");

    let b_output = freivalds_verifier(A, B, C);
    println!("output: {:?}", b_output);
}

fn random_n_by_n() -> Matrix {
    let mut rng = rand::thread_rng();
    Matrix::from_fn(|_, _| rng.gen_range(START..=STOP))
}

fn random_vector() -> Vector {
    let mut rng = rand::thread_rng();
    Vector::from_fn(|_, _| rng.gen_range(START..=STOP))
}

#[allow(dead_code)]
fn honest_mat_mul(a: Matrix, b: Matrix) -> Matrix {
    a * b
}

#[allow(dead_code)]
fn dishonest_mat_mul(_a: Matrix, _b: Matrix) -> Matrix {
    random_n_by_n()
}

fn freivalds_verifier(a: Matrix, b: Matrix, c: Matrix) -> bool {
    let x = random_vector();
    let cx = c * x;
    let abx = a * (b * x);

    abx == cx
}
