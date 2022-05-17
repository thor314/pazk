use std::error::Error;

struct SumcheckProtocol {
    g_arity: usize,
    p: Prover,
    v: Verifier,
    round: usize,
    done: bool,
}

pub fn arity<T, S>(f: impl Fn(T) -> S) -> usize {
    todo!();
}

impl SumcheckProtocol {
    pub fn new(g: impl Fn(&[usize]) -> usize + Copy) -> Result<Self, Box<dyn Error>> {
        let g_arity = arity(&g);
        if g_arity < 1 {
            return Err(Box::from("arity less than 1"));
        }
        let p = Prover::new(&g, g_arity);
        let v = Verifier::new(&g, g_arity, p.h_claim);

        Ok(Self {
            g_arity,
            p,
            v,
            round: 1,
            done: false,
        })
    }

    pub fn advance_round(&mut self) {
        assert!(!self.done);
        self.p.compute_and_send_next_polynomial(&self.v);
        self.v.check_latest_polynomial();
        if self.round == self.g_arity {
            self.done = self.v.evaluate_and_check_g_v();
        } else {
            self.v.get_new_random_value_and_send(&self.p);
            self.round += 1;
        }
    }

    // pub fn advance_to_end(&mut self, verbose: bool) {
    //     while !self.done {
    //         if verbose {
    //             println!("ADVANCE OUTPUT: {:?}", &self);
    //         }
    //         self.advance_round();
    //     }
    // }
}

struct Prover {
    g_arity: usize,
    random_challenges: Vec<usize>,
    cached_polynomials: Vec<Box<dyn Fn(&[usize]) -> usize>>,
    round: usize,
    h_claim: usize,
}
impl Prover {
    pub(crate) fn new(g: &impl Fn(&[usize]) -> usize, g_arity: usize) -> Self {
        todo!()
    }

    pub(crate) fn compute_and_send_next_polynomial(&mut self, v: &Verifier) {
        todo!();
    }
}
struct Verifier {
    g: Box<dyn Fn([usize]) -> usize>,
    g_arity: usize,
    h_claim: usize,
    random_challenges: Vec<usize>,
    round: usize,
    polynomials: Vec<Box<dyn Fn(&[usize]) -> usize>>,
}
impl Verifier {
    pub(crate) fn new(g: &impl Fn(&[usize]) -> usize, g_arity: usize, h_claim: usize) -> Self {
        todo!();
    }

    pub(crate) fn check_latest_polynomial(&self)  {
        todo!()
    }

    pub(crate) fn evaluate_and_check_g_v(&self) -> bool {
        todo!()
    }

    pub(crate) fn get_new_random_value_and_send(&self, p: &Prover)   {
        todo!()
    }
}
