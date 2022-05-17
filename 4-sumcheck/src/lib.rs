use std::error::Error;

struct SumcheckProtocol {
    g_arity: usize,
    p: Prover,
    v: Verifier,
    round: usize,
    done: bool,
}

fn arity<T, S>(f: impl Fn(T) -> S) -> usize {
    todo!();
}

impl SumcheckProtocol {
    pub fn new(g: impl Fn(&[usize]) -> usize+Copy) -> Result<Self, Box<dyn Error>> {
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
}

struct Prover {
    g_arity: usize,
    random_challenges: Vec<usize>,
    cached_polynomials: Vec<Box<dyn Fn(&[usize]) -> usize>>,
    round: usize,
    h_claim: usize,
}
impl Prover {
    pub fn new(g: &impl Fn(&[usize]) -> usize, g_arity: usize) -> Self {
        todo!()
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
    pub fn new(g: &impl Fn(&[usize]) -> usize, g_arity: usize, h_claim: usize) -> Self {
        todo!();
    }
}
