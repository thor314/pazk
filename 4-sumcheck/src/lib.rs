use std::rc::Rc;

use prover::Prover;
use utils::FArity;
use verifier::Verifier;

mod prover;
mod utils;
mod verifier;

pub(crate) type F = dyn Fn(Vec<usize>) -> usize;

struct SumcheckProtocol {
    farity: FArity,
    p: Prover,
    v: Verifier,
    round: usize,
    verbose: bool,
    done: bool,
}

impl SumcheckProtocol {
    pub fn new(g: Rc<F>, arity: usize, verbose: bool) -> Self {
        let farity = FArity::new(g, arity);
        let p = Prover::new(farity.clone(), verbose);
        let v = Verifier::new(farity.clone(), p.h_claim());

        Self {
            farity,
            p,
            v,
            round: 1,
            verbose: false,
            done: false,
        }
    }

    pub fn advance_round(&mut self) {
        assert!(!self.done);
        self.p.compute_and_send_next_polynomial(&mut self.v);
        self.v.check_latest_polynomial();
        if self.round == self.farity.arity() {
            self.done = self.v.evaluate_and_check_g_v();
        } else {
            self.v.get_new_random_value_and_send(&mut self.p);
            self.round += 1;
        }
    }

    pub fn advance_to_end(&mut self) {
        while !self.done {
            self.advance_round();
        }
    }
}
