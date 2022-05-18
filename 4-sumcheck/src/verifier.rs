use crate::{prover::Prover, utils::FArity};

pub(crate) struct Verifier {
    g: FArity,
    h_claim: usize,
    random_challenges: Vec<usize>,
    round: usize,
    second_last_poly: Option<FArity>,
    last_polynomial: Option<FArity>,
}
impl Verifier {
    pub(crate) fn new(g: FArity, h_claim: usize) -> Self {
        Self {
            g,
            h_claim,
            random_challenges: vec![],
            round: 1,
            second_last_poly: None,
            last_polynomial: None,
        }
    }

    pub(crate) fn receive_polynomial(&mut self, polynomial: FArity) {
        self.second_last_poly = self.last_polynomial.take();
        self.last_polynomial = Some(polynomial);
    }

    pub(crate) fn check_latest_polynomial(&self) {
        let last_polynomial = self.last_polynomial.as_ref().unwrap();
        let deg_latest = last_polynomial.deg_j(0);
        let deg_max = self.g.deg_j(self.round - 1);

        if deg_latest > deg_max {
            panic!(
                "Prover sent polynomial of degree {} greater than expected {}",
                deg_latest, deg_max
            );
        }
        let a = last_polynomial.call_f(vec![0]);
        let b = last_polynomial.call_f(vec![1]);
        dbg!(a,b);

        let new_sum = last_polynomial.call_f(vec![0]) + last_polynomial.call_f(vec![1]);
        let check = match self.round {
            1 => self.h_claim,
            _ => self
                .second_last_poly
                .as_ref()
                .unwrap()
                .call_f(vec![*self.random_challenges.last().unwrap()]),
        };
        if check != new_sum {
            panic!(
                "Prover sent incorrect polynomial value: {}, expected: {}",
                new_sum, check
            );
        }
    }

    pub(crate) fn get_new_random_value_and_send(&mut self, p: &mut Prover) {
        let new_challenge: usize = rand::random::<bool>().into();
        self.random_challenges.push(new_challenge);
        p.receive_challenge(new_challenge);
        self.round += 1;
        println!("VERIFIER: advancing to round {}", self.round);
    }

    pub(crate) fn evaluate_and_check_g_v(&mut self) {
        println!("VERIFIER: final round");
        let new_challenge: usize = rand::random::<bool>().into();
        self.random_challenges.push(new_challenge);
        let g_final = self.g.call_f(self.random_challenges.clone());
        let check = self
            .last_polynomial
            .take()
            .unwrap()
            .call_f(vec![*self.random_challenges.last().unwrap()]);
        if g_final != check {
            panic!(
                "Prover sent incorrect final polynomial value: {}, expected: {}",
                g_final, check
            );
        } else {
            println!("VERIFIER ACCEPTS")
        }
    }
}
