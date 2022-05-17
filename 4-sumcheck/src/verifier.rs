use crate::{prover::Prover, utils::FArity};

pub(crate) struct Verifier<'a> {
    g: &'a FArity,
    g_arity: usize,
    h_claim: usize,
    random_challenges: Vec<usize>,
    round: usize,
    polynomials: Vec<FArity>,
}
impl<'a> Verifier<'a> {
    pub(crate) fn new(g: &'a FArity, g_arity: usize, h_claim: usize) -> Self {
        Self {
            g,
            g_arity,
            h_claim,
            random_challenges: vec![],
            round: 1,
            polynomials: vec![],
        }
    }

    pub(crate) fn receive_polynomial(&mut self, polynomial: FArity){
        todo!();
    }

    pub(crate) fn check_latest_polynomial(&self) {
        todo!()
    }

    pub(crate) fn evaluate_and_check_g_v(&self) -> bool {
        todo!()
    }

    pub(crate) fn get_new_random_value_and_send(&self, p: &Prover) {
        todo!()
    }

}
