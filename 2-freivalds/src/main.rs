#![allow(non_snake_case)]
//! An implementation of Freivalds algorithm over a Prime Field, using Arkworks' Prime Library.
// Choose an arbitrary field for our driver:
use ark_bls12_381::Fr;
use ark_ff::PrimeField;
use nalgebra::{DMatrix, DVector};
use rand::{self, RngCore,rngs::ThreadRng};

fn main() {
    let mut rng = rand::thread_rng();
    // size of our matrix:
    let n = 10usize;
    let A = random_matrix::<Fr, ThreadRng>(&mut rng, n);
    let B = random_matrix::<Fr, ThreadRng>(&mut rng, n);

    println!("V: hey prover, multiply {:?} and {:?}", A, B);
    // swap these lines to change the honesty of the prover
    // let C = honest_mat_mul(&A, &B);
    let C = dishonest_mat_mul(&A,&B);
    println!("P: claim AB={:?}", C);

    println!("V: Verifying ABx = Cx");

    let b_output = freivalds_verifier(&A, &B, &C, &mut rng);
    println!("output: {:?}", b_output);
}

//
fn random_matrix<F: PrimeField, R: RngCore>(rng: &mut R, n: usize) -> DMatrix<F> {
    DMatrix::from_fn(n, n, |_, _| F::rand(rng))
}

fn random_vector<F: PrimeField, R: RngCore>(rng: &mut R, n: usize) -> DVector<F> {
    DVector::from_fn(n, |_, _| F::rand(rng))
}

#[allow(dead_code)]
fn honest_mat_mul<F: PrimeField>(a: &DMatrix<F>, b: &DMatrix<F>) -> DMatrix<F> {
    a * b
}

#[allow(dead_code)]
fn dishonest_mat_mul<F: PrimeField>(_a: &DMatrix<F>, _b: &DMatrix<F>) -> DMatrix<F> {
    let n = _a.ncols();
    let mut rng = rand::thread_rng();    
    random_matrix(&mut rng,n)
}

fn freivalds_verifier<F: PrimeField, R: RngCore>(a: &DMatrix<F>, b: &DMatrix<F>, c: &DMatrix<F>,rng: &mut R) -> bool {
    let x = random_vector( rng, a.ncols());
    let cx = c * x.clone();
    let abx = a * (b * x);

    abx == cx
}
