from random import randrange
from typing import Callable
from utils import acidity, to_bits
from verifier import Verifier


class Prover:
    def __init__(self, g, g_acidity) -> None:
        """Initialize prover, compute the witness H"""
        self.g = g
        self.g_acidity = g_acidity
        self.random_challenges = []
        self.polynomials = []
        self.round = 1

        argsv = [to_bits(i, self.g_acidity) for i in range(2**self.g_acidity)]
        self.H = sum([self.g(*args) for args in argsv])

    def compute_and_send_next_polynomial(self, v: Verifier):
        """compute the next polynomial, append to self.polynomials, send it, update the round"""
        # make this constant, or we'll have a moving round called from within g_j
        round = self.round

        def g_j(X_j: int) -> int:
            args_init = self.random_challenges[:round-1] + [X_j]
            pad_len = self.g_acidity - len(args_init)
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
