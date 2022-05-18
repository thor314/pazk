#![allow(non_snake_case)]
#![allow(dead_code)]

use prover::Prover;
use std::rc::Rc;
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
    done: bool,
}

impl SumcheckProtocol {
    pub fn new(g: Rc<F>, arity: usize) -> Self {
        let farity = FArity::new(g, arity);
        let p = Prover::new(farity.clone());
        let v = Verifier::new(farity.clone(), p.h_claim());

        Self {
            farity,
            p,
            v,
            round: 1,
            done: false,
        }
    }

    pub fn advance_round(&mut self) {
        assert!(!self.done);
        self.p.compute_and_send_next_polynomial(&mut self.v);
        self.v.check_latest_polynomial();
        if self.round == self.farity.arity() {
            self.v.evaluate_and_check_g_v();
            self.done = true;
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

#[test]
fn test_sumcheck() {
    let g: Rc<F> = Rc::new(|v: Vec<usize>| {
        let a = v[0];
        let b = v[1];
        let c = v[2];
        a + b + a * b + c
    });
    let f: Rc<F> = Rc::new(|v: Vec<usize>| {
        let a = v[0];
        let b = v[1];
        let c = v[2];
        a * b * c + a * b + c
    });
    let h: Rc<F> = Rc::new(|v: Vec<usize>| {
        let a = v[0];
        let b = v[1];
        let c = v[2];
        let d = v[3];
        a * b * c + b + c + c * d
    });
    let mut p1 = SumcheckProtocol::new(g, 3);
    let mut p2 = SumcheckProtocol::new(f, 3);
    let mut p3 = SumcheckProtocol::new(h, 4);

    p1.advance_to_end();
    p2.advance_to_end();
    p3.advance_to_end();
}
