from typing import Callable  # lol python, you tried


def to_bits(n: int, pad_to_len: int) -> list[bool]:
    v = [x == '1' for x in bin(n).lstrip('0b')]
    diff = pad_to_len - len(v)
    return [False for _ in range(diff)] + v


def lemma_37(f: Callable, x: list[bool]) -> int:
    xlen = len(x)
    acc = 0
    for i in range(2**xlen):
        w_i = to_bits(i, xlen)
        if X_w(w_i,x):
            out_i = f(w_i)
        else:
            out_i=0
        # print(f(w_i),w_i, out_i, acc, X_w(w_i,x))
        acc += out_i
    return acc


def X_w(w: list[bool], x: list[bool]) -> bool:
    assert(len(w) == len(x))
    acc = True
    for (i, x_i) in enumerate(x):
        out_i = (x_i and w[i]) or (not x_i and not w[i])
        acc &= out_i
    return acc


def test_lemma37():
    def f(bv: list[bool]):
        return bv[2]

    x = to_bits(7, 3)
    out = lemma_37(f, x)
    assert(out == 1)


def test_to_bits():
    bv = 2
    assert(to_bits(bv, 2)== [True, False])
    bv = 2
    assert(to_bits(bv, 3)==[False, True, False])
    bv = 7
    assert(to_bits(bv, 3)==[True, True,True])

if __name__ == "__main__":
    test_lemma37()
    test_to_bits()
    print("tests passed")