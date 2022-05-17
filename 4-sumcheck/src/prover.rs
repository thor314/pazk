use crate::{
    utils::{to_bits, FArity},
    verifier::Verifier,
};

pub(crate) struct Prover<'a> {
    arity: usize,
    random_challenges: Vec<usize>,
    cached_polynomials: Vec<&'a FArity>,
    round: usize,
    h_claim: usize,
}
impl<'a> Prover<'a> {
    pub(crate) fn new(g: &'a FArity, arity: usize) -> Self {
        let h_claim = (0..2usize.pow(arity as u32))
            .map(|i| g.f(&to_bits(i, arity)))
            .sum();
        Self {
            arity,
            random_challenges: vec![],
            cached_polynomials: vec![g],
            round: 1,
            h_claim,
        }
    }

    pub(crate) fn compute_and_send_next_polynomial(&mut self, v: &Verifier) {
        todo!();
    }
}
