use std::error::Error;

struct SumcheckProtocol {
    g_acidity: usize,
    p: Prover,
    v: Verifier,
    round: usize,
    done: bool,
}

fn acidity<T, S>(f: impl Fn(T) -> S) -> usize {
    todo!();
}

impl SumcheckProtocol {
    pub fn new(g: impl Fn(&[usize]) -> usize+Copy) -> Result<Self, Box<dyn Error>> {
        let g_acidity = acidity(&g);
        if g_acidity <= 1 {
            return Err(Box::from("bad"));
        }
        let p = Prover::new(&g, g_acidity);
        let v = Verifier::new(&g, g_acidity, p.h_claim);

        Ok(Self {
            g_acidity,
            p,
            v,
            round: 1,
            done: false,
        })
    }
}

struct Prover {
    g_acidity: usize,
    random_challenges: Vec<usize>,
    cached_polynomials: Vec<Box<dyn Fn(&[usize]) -> usize>>,
    round: usize,
    h_claim: usize,
}
impl Prover {
    pub fn new(g: &impl Fn(&[usize]) -> usize, g_acidity: usize) -> Self {
        todo!()
    }
}
struct Verifier {
    g: Box<dyn Fn([usize]) -> usize>,
    g_acidity: usize,
    h_claim: usize,
    random_challenges: Vec<usize>,
    round: usize,
    polynomials: Vec<Box<dyn Fn(&[usize]) -> usize>>,
}
impl Verifier {
    pub fn new(g: &impl Fn(&[usize]) -> usize, g_acidity: usize, h_claim: usize) -> Self {
        todo!();
    }
}
