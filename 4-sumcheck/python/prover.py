from random import randrange
from typing import Callable
from utils import arity, to_bits
from verifier import Verifier


class Prover:
    """This prover uses a function currying cache to improve its runtime"""

    def __init__(self, g, g_arity) -> None:
        """Initialize prover, compute the witness H"""
        self.g_arity = g_arity
        self.random_challenges = []
        self.cached_polynomials = [g]
        self.round = 1

        argsv = [to_bits(i, self.g_arity) for i in range(2**self.g_arity)]
        self.H = sum([self.cached_polynomials[0](*args) for args in argsv])

    def compute_and_send_next_polynomial(self, v: Verifier):
        """compute the next polynomial, append to self.polynomials, send it, update the round"""
        # make this constant, or we'll have a moving round called from within g_j
        round = self.round
        poly = self.cached_polynomials[-1]

        def g_j(X_j: int) -> int:
            pad = self.g_arity - round
            argsv = [[X_j] + to_bits(i, pad) for i in range(2**pad)]
            return sum([poly(*args) for args in argsv])
        v.receive_polynomial(g_j)
        self.round += 1

    def receive_challenge(self, challenge: int):
        """receive the latest challenge from the verifier"""
        self.random_challenges.append(challenge)
        self.cache_next(challenge)
        print("received challenge {}, initiating round {}".format(
            challenge, self.round))

    def cache_next(self, challenge: int):
        """Use the latest challenge to cache the next polynomial. Eg:
        if the last polynomial in cached polynomials is g_i(a,b,c,d)
        and the verifier just sent challenge r_i, the next cached polynomial will be
        g_{i+1}(b,c,d) = g_i(r_i,b,c,d)
        """
        poly = self.cached_polynomials[-1]

        def next_poly(*args):
            print("got args:", args)
            return poly(challenge, *args)
        self.cached_polynomials.append(next_poly)


class InefficientProver:
    def __init__(self, g, g_arity) -> None:
        """Initialize prover, compute the witness H"""
        self.g = g
        self.g_arity = g_arity
        self.random_challenges = []
        self.polynomials = []
        self.round = 1

        argsv = [to_bits(i, self.g_arity) for i in range(2**self.g_arity)]
        self.H = sum([self.g(*args) for args in argsv])

    def compute_and_send_next_polynomial(self, v: Verifier):
        """compute the next polynomial, append to self.polynomials, send it, update the round"""
        # make this constant, or we'll have a moving round called from within g_j
        round = self.round

        def g_j(X_j: int) -> int:
            args_init = self.random_challenges[:round-1] + [X_j]
            pad_len = self.g_arity - len(args_init)
            argsv = [args_init + to_bits(i, pad_len)
                     for i in range(2**pad_len)]
            # print("argsv", argsv)
            return sum([self.g(*args) for args in argsv])
        self.polynomials.append(g_j)
        v.receive_polynomial(g_j)
        self.round += 1

    def receive_challenge(self, challenge: int):
        """receive the latest challenge from the verifier"""
        self.random_challenges.append(challenge)
        print("received challenge {}, initiating round {}".format(
            challenge, self.round))
