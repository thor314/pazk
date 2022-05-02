# Used to generate random matrices, and select a random point `r` to generate
# a vector

import random
import numpy as np
SIZE = 7  # size matrix to use
START = 0  # minimum int value in matrix
STOP = 10  # maximum int value in matrix


def main():
    A = random_n_by_n(SIZE)
    B = random_n_by_n(SIZE)

    print("V: hey prover, multiply %s and %s" % (A, B))
    # Swap the comment of the next two lines to choose honest or dishonest mul
    C = honest_mat_mul(A, B)
    # C = dishonest_mat_mul(A,B)
    print("P: claim AB=%s" % C)

    print("V: Verifying ABx = Cx")

    b_output = freivalds_verifier(A, B, C)
    print("output: %s" % b_output)


def random_n_by_n(size):
    """generate a SIZE x SIZE matrix with random integer values between START
    and STOP.
    """
    return np.matrix([random_vector(size) for i in range(size)])


def random_vector(size):
    """generate a SIZE x 1 vector with random integer values between START
    and STOP"""
    return [random.randrange(START, STOP) for i in range(SIZE)]


def honest_mat_mul(A, B):
    """multiply two matrices, using the numpy library"""
    return A*B


def dishonest_mat_mul(A, B):
    """randomly select an output n x n matrix"""
    return random_n_by_n(len(A))


def freivalds_verifier(A, B, C):
    """verify in O(n^2) that AB == C by selecting a random vector"""
    # print("debug: A: %s, \nB: %s, \nC: %s, \nAB: %s" %
    #      (A, B, C, np.matrix(A)*np.matrix(B)))

    # vstack because transpose doesn't do what I thought it would
    x = np.vstack(np.array(random_vector(SIZE)))
    Cx = C*x
    ABx = A*(B*x)
    # print("debug: ABx: %s \nCx: %s" % (ABx, Cx))
    return (Cx == ABx).all()


if __name__ == "__main__":
    main()
