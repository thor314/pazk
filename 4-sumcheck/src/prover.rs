use crate::{
    utils::{to_bits, FArity},
    verifier::Verifier,
    F,
};
use std::{iter::once, rc::Rc};

pub(crate) struct Prover {
    random_challenges: Vec<usize>,
    cached_polynomial: Option<FArity>,
    round: usize,
    h_claim: usize,
}

impl Prover {
    pub(crate) fn new(g: FArity ) -> Self {
        
        let h_claim = (0..2usize.pow(g.arity() as u32))
            .map(|i| g.call_f(to_bits(i, g.arity())))
            .sum();
        Self {
            random_challenges: vec![],
            cached_polynomial: Some(g),
            round: 1,
            h_claim,
        }
    }
    pub(crate) fn h_claim(&self) -> usize {
        self.h_claim
    }

    /// Need the static lifetime, so that Prover lives as long as FArity
    /// Logicking about closures is hard man
    pub(crate) fn compute_and_send_next_polynomial(&mut self, v: &mut Verifier) {
        // hand cached polynomial off to next closure
        let pad_to_len = self.cached_polynomial.as_ref().unwrap().arity() - self.round;
        let poly = self.cached_polynomial.clone().unwrap();

        let g_j: Rc<F> = Rc::new(move |X_j| {
            (1..2usize.pow(pad_to_len as u32))
                .map(|i| {
                    let args = X_j
                        .iter()
                        .cloned() // can't move out out X_j, X_j does not implement copy
                        .chain(to_bits(i, pad_to_len).into_iter())
                        .collect::<Vec<usize>>();
                    poly.call_f(args)
                })
                .sum::<usize>()
        });
        // note that g_j will always have arity 1
        let f_j = FArity::new(g_j, 1);
        v.receive_polynomial(f_j);
        self.round += 1;
        println!("PROVER: advancing to round {}", self.round);
    }
    pub(crate) fn receive_challenge(&mut self, challenge: usize) {
        self.random_challenges.push(challenge);
        self.cache_next(challenge);
    }

    fn cache_next(&mut self, challenge: usize) {
        let f = self.cached_polynomial.take().unwrap();
        let next_arity = f.arity() - 1;
        let next_f: Rc<F> = Rc::new(move |args: Vec<usize>| {
            f.call_f(once(challenge).chain(args.into_iter()).collect())
        });
        let next_farity = FArity::new(next_f, next_arity);
        self.cached_polynomial = Some(next_farity);
    }
}
