from typing import Callable
from prover import Prover
from utils import arity
from verifier import Verifier


class SumcheckProtocol:
    """The sumcheck protocol, as defined in Proofs Arguments and Zero Knowledge ch 4.1, 
    defined over polynomials of arbitrary acidity over {0,1}"""

    def __init__(self, g: Callable) -> None:
        g_arity = arity(g)
        if g_arity <= 1:
            raise ValueError(
                "function acidity must be greater than or equal to 1")

        self.g_acidity = g_arity
        # for simplicity, have prover compute H on initialization
        self.p = Prover(g, self.g_acidity)
        self.v = Verifier(g, self.g_acidity, self.p.H)
        self.round = 1
        self.done = False

    def __repr__(self) -> str:
        return f'Protocol(round: "{self.round}", H: "{self.p.H}", challenges: "{self.p.random_challenges}")'

    def advance_round(self):
        if not self.done:
            # Prover: compute next polynomial and send it to verifier
            self.p.compute_and_send_next_polynomial(self.v)
            self.v.check_latest_polynomial()
            if self.round == self.g_acidity:
                # final round
                self.done = self.v.evaluate_and_check_g_v()
            else:
                self.v.get_new_random_value_and_send(self.p)
                self.round += 1
        else:
            raise RuntimeError("Sumcheck protocol has finished")

    def advance_to_end(self, verbose: bool = False):
        while not self.done:
            if verbose:
                print("ADVANCE OUTPUT:", self)
            self.advance_round()


def test_sumcheck():
    def g(a, b, c): return a + b + a*b + c
    protocol = SumcheckProtocol(g)
    protocol.advance_to_end(True)

    def f(a, b, c): return a*b*c+b+c
    protocol = SumcheckProtocol(f)
    protocol.advance_to_end(True)

    def ff(a, b, c, d): return a*b*c+b+c+c*d
    protocol = SumcheckProtocol(ff)
    protocol.advance_to_end(True)
