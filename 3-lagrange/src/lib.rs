#![allow(non_snake_case)]
#![allow(dead_code)]
use ark_ff::PrimeField;
type Bit = bool;
type Bv = Vec<Bit>;

/// convert n to an unpadded vector of bits
fn to_bits(n: usize, pad_to_len: usize) -> Bv {
    let mut v: Bv = format!("{:b}", n).chars().map(|c| c == '1').collect();
    let diff = pad_to_len - v.len();
    let mut vo = vec![false; diff];
    vo.append(&mut v);
    vo
}

/// Functional equivalent to how the book presents Lemma 3.7 (imperatively)
fn lemma_37<F, Fr: PrimeField>(f: F, x: &Bv) -> Fr
where
    F: Fn(&Bv) -> Fr,
{
    let xlen = x.len();
    (0..(2usize.pow(x.len() as u32)))
        .map(|i| to_bits(i, xlen))
        // a  cop-out: don't return a function, but its evaluation, see bottom
        .map(|w_i| if X_w(&w_i, x) { f(&w_i) } else { Fr::zero() })
        .sum()
}

// see eqn 3.2, w'th Lagrange basis
fn X_w(w: &Bv, x: &Bv) -> Bit
where
{
    assert!(w.len() == x.len());
    x.iter()
        .enumerate()
        .map(|(i, x_i)| (*x_i && w[i]) || (!x_i) && (!w[i]))
        .fold(true, |acc, a| acc && a)
}

// lemma 3.8 elided.

#[cfg(test)]
mod test {
    use super::*;
    // use ark_bls12_381::Fr;
    // or make your own field
    use ark_ff::{fields::Fp64, MontBackend, MontConfig};

    #[derive(MontConfig)]
    #[modulus = "17"]
    #[generator = "3"]
    pub struct FqConfig;
    pub type Fq = Fp64<MontBackend<FqConfig, 1>>;

    #[test]
    fn t_lemma37() {
        let f = |bv: &Bv| Fq::from(bv[2]);
        let x = to_bits(7, 3);
        let out = lemma_37(f, &x);
        assert_eq!(out, Fq::from(1));
    }

    #[test]
    fn test_to_bits() {
        let bv = 2usize;
        assert_eq!(to_bits(bv, 2), [true, false]);
        let bv = 2usize;
        assert_eq!(to_bits(bv, 3), [false, true, false]);
        let bv = 7;
        assert_eq!(to_bits(bv, 3), [true, true, true]);
    }
}

// compute f^~(x), see eqn 3.1
// fn lemma_37_wrong<F, G, Fr: PrimeField>(f: F, x: &Bv) -> G
// where
//     F: Fn(Vec<&Bv>) -> Fr,
//     G: Fn(Vec<&Fr>) -> Fr,
// {
//     let xlen = x.len();
//     let w: Vec<Bv> = (0..(2usize.pow(x.len() as u32)))
//         .map(|w_i| to_bits(w_i, xlen)).collect();
//     // a  cop-out: don't return a function, but its evaluation, see bottom
//     // the following fails: closures have distinct types, so cannot match generic parameter G
//     // my type theory isn't good enough for this API
//     let g: G = |fr_v: Vec<&Fr>| -> Fr {
//         w.into_iter().map(|w_i| fr_v(w_i)*X_w(&w_i,x))
//     };
// }
