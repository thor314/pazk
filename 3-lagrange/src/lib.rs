use ark_ff::PrimeField;
type Fr = u8;
// bit field
type B = u8;

/// compute f^~(x), see eqn 3.1
fn lemma_36<F>(f: F, x: &[B]) -> Fr
where
    F: Fn(Vec<B>) -> Fr,
{
    // todo: how to generate w, all bitvectors of arbitrary length
    let w: Vec<Vec<B>> = vec![vec![]];
    w.into_iter().map(|w_i| f(w_i.clone()) * X_w(&w_i, x)).sum()
}

// see eqn 3.2, w'th Lagrange basis
fn X_w(w: &[B], x: &[B]) -> Fr {
    assert!(w.len() == x.len());
    let fr = x
        .iter()
        .enumerate()
        .map(|(i, &x_i)| x_i * w[i] + (1 - x_i) * (1 - w[i]))
        .fold(1, |acc, a| acc * a);
    Fr::from(fr)
}

/// Compute f^~(r) in O(n log n) time and O(log n) space with a streaming pass
fn lemma_37<F>(f: F, r: Rvec)
where
    F: Fn(Bitvec) -> Bit,
{
    let mut f_out = |x: Bitvec| 0;
    for bit in r {
        f_out = f_out(r) + f(bit) * lagrange(w, r)
    }
    todo!();
}

// fn lemma_38<F>(f: F, r: Bitvec)
// where
//     F: Fn(Bitvec) -> Bit,
// {
//     todo!();
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_lemma36() {
        // let out = lemma_36(f), x)
        todo!()
    }

    fn test_exercise_34() {
        todo!()
    }
}
