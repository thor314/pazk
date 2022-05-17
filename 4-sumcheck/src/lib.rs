use std::error::Error;

use prover::Prover;
use verifier::Verifier;

mod prover;
mod utils;
mod verifier;

struct SumcheckProtocol<'a>{
    g_arity: usize,
    p: Prover<'a>,
    v: Verifier<'a>,
    round: usize,
    done: bool,
}

// impl SumcheckProtocol {
//     pub fn new(g: impl Fn(&[usize]) -> usize + Copy) -> Result<Self, Box<dyn Error>> {
//         let g_arity = arity(&g);
//         if g_arity < 1 {
//             return Err(Box::from("arity less than 1"));
//         }
//         let p = Prover::new(&g, g_arity);
//         let v = Verifier::new(&g, g_arity, p.h_claim);

//         Ok(Self {
//             g_arity,
//             p,
//             v,
//             round: 1,
//             done: false,
//         })
//     }

//     pub fn advance_round(&mut self) {
//         assert!(!self.done);
//         self.p.compute_and_send_next_polynomial(&self.v);
//         self.v.check_latest_polynomial();
//         if self.round == self.g_arity {
//             self.done = self.v.evaluate_and_check_g_v();
//         } else {
//             self.v.get_new_random_value_and_send(&self.p);
//             self.round += 1;
//         }
//     }

//     // pub fn advance_to_end(&mut self, verbose: bool) {
//     //     while !self.done {
//     //         if verbose {
//     //             println!("ADVANCE OUTPUT: {:?}", &self);
//     //         }
//     //         self.advance_round();
//     //     }
//     // }
// }
