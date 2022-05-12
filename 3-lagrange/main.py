from typing import Callable # lol python, you tried

def to_bits(n: int, pad_to_len: int) -> list[bool]:
    v = [x == '1' for x in bin(n).lstrip('0b')]
    diff = pad_to_len - len(v)
    return [False for _ in range(diff)] + v


def lemma_37(f: Callable, x: list[bool]) -> int:
    xlen= len(x)
    acc = 0
    for i in range(2**xlen):
        w_i = to_bits(i, xlen)
        out_i = f(w_i) if X_w(w_i, x) else 0
        acc += out_i
    return acc
        

def X_w(w: list[bool], x: list[bool]) -> bool:
    assert(len(w) == len(x))
    acc = True
    for (i,x_i) in enumerate(x):
        out_i = (x_i and w[i]) or (not x_i and not w[i])
        acc = acc and out_i
    return acc
# 