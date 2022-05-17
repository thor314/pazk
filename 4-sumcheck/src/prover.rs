use std::iter::once;

use crate::{
    utils::{to_bits, FArity},
    verifier::Verifier,
};

pub(crate) struct Prover {
    arity: usize,
    random_challenges: Vec<usize>,
    cached_polynomials: Vec<FArity>,
    round: usize,
    h_claim: usize,
    verbose: bool,
}

impl Prover {
    pub(crate) fn new(g: FArity, arity: usize, verbose: bool) -> Self {
        let h_claim = (0..2usize.pow(arity as u32))
            .map(|i| g.call_f(to_bits(i, arity)))
            .sum();
        Self {
            arity,
            random_challenges: vec![],
            cached_polynomials: vec![g],
            round: 1,
            h_claim,
            verbose,
        }
    }

    /// Need the static lifetime, so that Prover lives as long as FArity
    /// Logicking about closures is hard man
    pub(crate) fn compute_and_send_next_polynomial(&'static mut self, v: &mut Verifier) {
        let pad_to_len = self.arity - self.round;
        let poly = self.cached_polynomials.last().unwrap();
        self.round += 1;
        let g_j: Box<dyn Fn(Vec<usize>) -> usize> = Box::new(move |X_j| {
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
    }
    pub(crate) fn receive_challenge(&'static mut self, challenge: usize) {
        self.random_challenges.push(challenge);
        let round = self.round;
        let verbose = self.verbose;
        // let s = self.cache_next(challenge);
        if verbose {
            println!(
                "received challenge {}, initiating round {}",
                challenge, round
            );
        }
    }

    // fn cache_next(&'static mut self, challenge: usize) {
    //     let poly = self.cached_polynomials.last().unwrap();
    //     let next_poly = move |args: Vec<usize>| {
    //         if self.verbose {
    //             println!("got args: {:?}", &args);
    //         }
    //         let new_args: Vec<usize> = once(challenge).chain(args.into_iter()).collect();
    //         poly.call_f(new_args)
    //     };
    //     let next_farity = FArity::new(Box::new(next_poly), poly.arity() - 1);
    //     self.cached_polynomials.push(next_farity);
    // }
}
