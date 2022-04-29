#note #book #pazk 

## Key stuff
- Examples of using randomness to create a proof: *same file check*, Freivalds algorithm for Matrix Mult.
- Important properties to understand how to check: Completeness, Soundness, Communication cost, Cost to compute

## Notes
Chapter focuses on 2 examples of verifiable computation.

### Problem 1: do we have the same file? 
#### let's get naive
A naive approach might be for Alice to send her n-byte file to Bob, and bob to do something like:
`$ cmp myFile aliceFile`

Problems:
- Alice trusts Bob
- Alice send all n bytes
- Bob gets Alice's file, even if he didn't have it

#### Less naive:
Alice sends the hash of her file, $h(file)$ to Bob. Bob checks if the hash of his file is the same as Alice's.

Now Alice only sends a constant number of bytes, not the whole file, and Bob can't recreate the file from Alice's message.

Problems:
- Alice still trusts Bob to do the comparison
- If Bob wants to convince Carol that they share the same file, he can lie, and just pass along the hash he got from Alice

#### How could we solve Alice trusting Bob?
Alice and Bob independently select random hashes (not the same hash). Alice sends Bob the hash of her message, and what the hash is, Bob does likewise with his hash.

Alice now no longer has to trust that Bob tells her something honestly, but...

Bob could still be passing along a hash he got from someone else (and so could Alice). There's also added communication complexity here for Alice and Bob to describe their hash.

#### an actual solution, using random hashes
Alice needs to randomly select a hash function, $h$, from some family $\mathcal H$. 
Alice will send:
- $h$
- $h(file_{alice})$
Bob will check:
- $h(f_a) == h(f_b)$ ? 
False -> Bob's file is different from Alice's, output `NOT-EQUAL`.
True: do the other thing.

This can be repeated in the opposite direction as with the prior protocol to convince Alice that Bob has the same file. 

Note that if Alice and Bob share a trusted source of randomness, all is well and good BUT AGAIN, Bob might have sent Alice a hash that he got from someone else (*bad*), so each party would have to prove that they did in fact choose a random hash, or else, add an initial step to challenge the other party to use a particular hash function.

#### What analysis do we do?
We want to show things. What things matter to show, when talking about VC's? 
- Completeness
- Soundness
- Communication cost 

**Completeness** follows naturally, as it usually will. If Bob does honest stuff, Alice will be convinced.

**Soundness:**
The book chooses a particular class of hash function, but the problem reduces to:
How likely is it that Bob can guess a hash collision with Alice's input? 

So choose a hash function with a sufficiently large digest, and the probability will be low.

**Communication cost**
Alice sends a hash function and a hash function evaluation. If these are elements in some field $\mathbb F_p$, then the cost is however many field elements the hash takes to represent, and one element for the output. The book gives a hash function that takes only one field element to represent, so the total cost is 2 field elements.

#### Emphasis Aside
Brief note: The above checks are the most important thing to internalize about this chapter. We check an algorithm's effectiveness by checking how well it posseses desireable properties, which in this case were *soundness, completeness, and communication cost.*

### Freivald's Algorithm
Given 2 $n \times n$ matrices, A and B, over $\mathbb F_p$, we want a Prover to compute the product, C=AB (naively O(n^3) time, most efficiently O(n^2.4) time),.
We want the Verifier to be able to check the product quickly (less than O(n^2.4) time, in this case, O(n^2) time).

How do?
1. Verifier says, "hey prover, what's AB?"
2. Prover says, "C"
3. Verifer randomly selects r, calculates a vector: $x=(r,r^2..,r^n)$
Checks that:

$$Cx== (AB)x$$ 

If Prover computed C correctly, 
Noting that the product of the vector and a matrix is an `O(n^2)` operation.

$(AB)x = A(Bx) = Cx$  is therefore relatively efficient to compute.

#### What do we need to check? 
Soundness, Completeness, Communication Cost, cost to compute.

Completeness was just shown.

Cost to compute: described above, the Prover makes no modification to computing C, and may compute in any way. 
The Verifier runs two `O(n^2)` checks, which is a plausible improvement over one `O(n^(2.4))` evaluation.

**Soundness** 
What's the probability that a cheating prover gets lucky? Eg, suppose some row, $i$ of $C'\ne C$, but the rest is identical. Express this row $c'_i, c_i$. We want to find the probability:

$$Pr(c'_i*x = c_i *x)$$

Optimistically, this probability is $1/p$.
Pessimistically, this could be as great as $n/p$, where recall that n is the length of the vector, which can be shown by applying Fact 2.1. $c_i*x=c'_i*x$ will hold for n 
choices of $r$, therefore there will be at most $n$ collisions; therefore the probability is at most $n/p$.

**Communication complexity**
The Verifier sends 2  n x n matrices, so $2n^2$ elements.
The Prover sends back 1 n x n matrix, so $n$ elements.