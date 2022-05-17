use std::ops::Deref;
/// Helper struct to allow arity methods
pub(crate) struct FArity {
    f: Box<dyn Fn(&Vec<usize>) -> usize>,
    arity: usize,
}

impl FArity {
    pub(crate) fn new(f: Box<dyn Fn(&Vec<usize>) -> usize>, arity: usize) -> Self {
        Self { f, arity }
    }
    pub(crate) fn arity(&self) -> usize {
        self.arity
    }
    pub(crate) fn f(&self, v: &Vec<usize>) -> usize {
        assert!(v.len() == self.arity);
        self.f.deref()(v)
    }
    /// warning: home cooked. Assume non-negative integer power less than 10
    pub(crate) fn deg_j(&self, j: usize) -> usize {
        assert!(self.arity > j);
        let mut exp = 1u32;
        loop {
            let (args1, args2) = self.gen_args(j);
            let (out1, out2) = (self.f(&args1) as isize / 10, self.f(&args2) as isize / 100);
            if (out1.pow(exp) - out2.pow(exp)).abs() <= 1 {
                return exp as usize;
            } else if exp <= 10 {
                exp += 1;
            } else {
                panic!("exp grew larger than 10");
            }
        }
    }
    fn gen_args(&self, j: usize) -> (Vec<usize>, Vec<usize>) {
        let args1 = std::iter::repeat(1)
            .take(j)
            .chain(std::iter::once(10))
            .chain(std::iter::repeat(1).take(self.arity - j - 1))
            .collect();
        let args2 = std::iter::repeat(1)
            .take(j)
            .chain(std::iter::once(100))
            .chain(std::iter::repeat(1).take(self.arity - j - 1))
            .collect();
        (args1, args2)
    }
}

pub(crate) fn to_bits(n: usize, pad_to_len: usize) -> Vec<usize> {
    assert!(2usize.pow(pad_to_len as u32) > n);
    let pad = get_pad_len(n, pad_to_len);
    std::iter::repeat(0usize)
        .take(pad)
        .chain(format!("{:b}", n).chars().map(|c| (c == '1').into()))
        .collect()
}

/// warning: home cooked
fn get_pad_len(n: usize, pad_to_len: usize) -> usize {
    let min = 2usize.pow(pad_to_len as u32 - 1);
    let max = min * 2;
    (0u32..)
        .map(|i| max - 2usize.pow(i) * n)
        .position(|v| v <= min)
        .unwrap()
}

#[test]
fn test_arity() {
    let f = |arr: &Vec<usize>| -> usize { arr.iter().sum() };
    let g = |arr: &Vec<usize>| -> usize { arr.iter().sum() };
    let F = FArity::new(Box::new(f), 3);
    let G = FArity::new(Box::new(g), 4);
    assert_eq!(F.arity(), 3);
    assert_eq!(G.arity(), 4);
    assert_eq!(F.f(&vec![1, 2, 3]), 6);
    assert_eq!(G.f(&vec![1, 1, 2, 3]), 7);
}

#[test]
fn test_to_bits() {
    assert_eq!(to_bits(17, 7), vec!(0, 0, 1, 0, 0, 0, 1));
    assert_eq!(to_bits(16, 7), vec!(0, 0, 1, 0, 0, 0, 0));
    assert_eq!(to_bits(15, 7), vec!(0, 0, 0, 1, 1, 1, 1));
}
