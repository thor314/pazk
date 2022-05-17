from inspect import signature
from typing import Callable


def arity(g: Callable) -> int:
    """return the number of arguments taken by g"""
    return len(signature(g).parameters)


def to_bits(n: int, pad_to_len: int) -> list[int]:
    """return n as a binary vector, front-padded to pad_to_len"""
    v = [1 if x == '1' else 0 for x in bin(n).lstrip('0b')]
    diff = pad_to_len - len(v)
    return [0 for _ in range(diff)] + v


def deg_j(g: Callable, j: int) -> int:
    """return the degree of the j'th variable in g. Assume a non-negative integer power less than 10"""
    arity_g = arity(g)
    assert arity_g > j

    exp = 1
    while True:
        args = [1 for _ in range(j)]+[100] + \
            [1 for _ in range(arity_g - j - 1)]
        out_1 = g(*args)
        args = [1 for _ in range(j)]+[1000] + \
            [1 for _ in range(arity_g - j - 1)]
        out_2 = g(*args)
        # print(j, exp, out_1, out_2)
        if abs(out_1/100**exp-out_2/1000**exp) < 1:
            return exp
        elif exp > 10:
            raise ValueError("exp grew larger than 10")
        else:
            exp += 1


def test_degj():
    def f(a, b, c): return a*b*b*c+b+c*c*c
    def ff(a, b, c, d): return a*b+b+d**3
    assert deg_j(f, 0) == 1
    assert deg_j(f, 1) == 2
    assert deg_j(f, 2) == 3
    assert deg_j(ff, 0) == 1
    assert deg_j(ff, 1) == 1
    assert deg_j(ff, 2) == 0
    assert deg_j(ff, 3) == 3
    def fff(a, b, c, d): return a*b*c+b+c+c*d
    assert deg_j(fff, 0) == 1
    assert deg_j(fff, 1) == 1
    assert deg_j(fff, 2) == 1
    assert deg_j(fff, 3) == 1
