from random import randrange
from typing import Callable
from utils import deg_j


class Verifier:
    def __init__(self, g, g_arity, H) -> None:
        """Initialize verifier with the claimed witness H"""
        self.g = g
        self.g_arity = g_arity
        self.H = H
        self.random_challenges = []
        self.round = 1
        self.polynomials = []

    def receive_polynomial(self, polynomial: Callable):
        """receive the latest polynomial from the Prover"""
        self.polynomials.append(polynomial)

    def check_latest_polynomial(self):
        """Verify that self.latest_polynomial is a univariate polynomial of at most deg_j(g) and 
        that g_{j-1}(r_{j-i}) = g_j(0)+g_j(1)"""
        latest_poly = self.polynomials[-1]

        # print("debug %s" % len(self.polynomials))
        # print("round", self.round, "poly:", latest_poly(0), latest_poly(1))
        deg_latest = deg_j(latest_poly, 0)
        deg_max = deg_j(self.g, self.round-1)
        if deg_latest > deg_max:
            raise ValueError("Prover sent polynomial of degree {} greater than expected: {}".format(
                deg_latest, deg_max))

        new_sum = latest_poly(0)+latest_poly(1)
        if self.round == 1:
            check = self.H
        else:
            check = self.polynomials[-2](self.random_challenges[-1])
            # print("check", check)
        if check != new_sum:
            raise ValueError(
                "Prover sent incorrect polynomial: {},expected: {}".format(new_sum, check))

    def get_new_random_value_and_send(self, p):
        """Get a new random value, append it to random challenges, send it to the prover, and update the round"""
        self.random_challenges.append(randrange(2))
        p.receive_challenge(self.random_challenges[-1])
        self.round += 1

    def evaluate_and_check_g_v(self):
        assert len(self.random_challenges) == self.g_arity-1
        self.random_challenges.append(randrange(2))
        g_final = self.g(*self.random_challenges)
        check = self.polynomials[-1](self.random_challenges[-1])

        if g_final != check:
            raise ValueError(
                "Prover sent incorrect final polynomial: {},expected: {}".format(g_final, check))
        else:
            print("VERIFIER ACCEPTS")
            return True
