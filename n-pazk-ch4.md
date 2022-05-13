#note #pazk #cryptography 

Interactive proofs, sumcheck, GKR.

## key stuff
- Model of a strong prover and weak verifier
- analyzing sumcheck prover and verifier time
- Some minor applications of sumcheck
- The GKR protocol
- Cost frame: think of the cost of a protocol as on the order of $S$, where S is the number of gates. We want $S=O(p(n))$ with $p$ a small degree polynomia, wit with $p$ a small degree polynomial, ideally.

## sumcheck
The protocol: Given a $v$ variate polynomial $g$ over $\mathbb F$ , the *weak Verifier* wants a *strong Prover* to compute all evaluations of $g$ over $\mathbb F$:

$$ H:= \sum_{b\in \mathbb F} g(b)$$

Where we're going to interpret b as all possibly binary strings. To constrain the Prover to only a correct output for $H$, it would be nice if the Prover had only to do minimally more work than normal, and the Verifier could run in less than exponential time, and that's what the sumcheck protocol is. See book for the protocol description.

But in a nutshell, the Verifier challenges the Prover to compute a Currying of $g$ in each round, reducing it's multivariate-ness by one, by passing a random element in. 
The Prover is then bound to each of its previous answers, and could only arrive at a correct final answer with probability $dv/| \mathbb F|$. More concretely:

### Proof of Soundness of sumcheck (completeness follows naturally)
Suppose the Prover diverges at round $i$, and sends some other polynomial $s_i(X_i)\ne g_i(X_i)$, but $g_i(r_i) = s_i(r_i)$.  By a restriction on the degree $d$ of the polynomial $s_i$, we can exploit the FToAlgebra to obtain the probability of that a collision, $d/| \mathbb F|$ . For the sake of proof convenience, we allow that the prover could have sent the false polynomial in any round, we arrive at a final soundness bound of $dv/| \mathbb F|$.

## Costs
- The verifier sends a random field element in each round
- the Prover responds with the ith polynomial, expressible in $deg_i(g) +1$ elements. 
- The Verifier twice evaluates a polynomial $g_i$ in each round, verifying it against the previous round, and finally takes one evaluation of $g(r_1..r_v)$
- The Prover can curry $g_i$ from $g_{i-1}$ in $deg_i(g)2^{v-i}$ time in each round. $O(\sum_i deg_i(g)2^{v-i}T)=O(2^{v+1}T)$ , assuming $deg_i(g)$ is O(1)

| Communication                   | Rounds | V time              | P time |
| ------------------------------- | ------ | ------------------- | ------ |
| O$(2v+\sum deg_i(g))=^1O(v)$ field elems | $v$    | $O(T+2*\sum deg_i(g))=^1 O(T+v)$ | $O(\sum deg_i(g)2^{v-i}T) =^1 O(T*2^{v})$       |

Where $T$ is the time to evaluate $g$ once, and the equality $=^1$ holds if $deg_i(g)=O(1)$.

This is pretty good: We get a linear time algorithm for V, and a factor of 2 blowup for P, IF WE ASSUME 
- that $deg_i(g)$ is $O(1)$ 
- that the time for the verifier to take a SINGLE evaluation of $g$ ain't too bad. Remember that P took $2^{v+1}$ evaluations of $g$. Our verifier is lazy AF.

## Some Applications
- `#SAT`
- Counting triangles in a graph
- Matrix multiplication verification (skipped)

### `#SAT`
Or in other words, computing the number of satisfying arguments to some boolean formula $\phi$. We need to turn the boolean formula $\phi$ into an arithmatic circuit $\psi$ over $\mathbb F$ that computes the polynomial extension $g$ of $\phi$.  

In simpler english, we have a boolean circuit $\phi$. We want a polynomial expression $g$ to express it, to run our sumcheck on. We can construct $g$ by translating our boolean formula to an arithmetic circuit and Lagrange-interpolating $g$ out of that. We map (all variables in {0,1}):
- $\phi \rightarrow \psi$ 
- AND(a,b) => ab (1 gate)
- OR(a,b) => a+b-ab (3 gates)
- NOT(a) => 1-a (1 gate)

Therefore, if $\phi$ was $S$ gates, $\phi$ is at most $3S$ gates. We can exploit an intuitive upper bound, that $\sum_i deg_i(g)\le 2S$, explained in a footnote, to compute our cost.
For unknown reasons, we switch from $v$ to $n$ notationally, but they are equivalent.

| Communication                            | Rounds | V time                           | P time                                    |
| ---------------------------------------- | ------ | -------------------------------- | ----------------------------------------- |
| O$(2v+\sum deg_i(g))=^1O(v)$ field elems | $v$    | $O(T+2*\sum deg_i(g))=^1 O(T+v)$ | $O(\sum deg_i(g)2^{v-i}T) =^1 O(T*2^{v})$ |
| $O(2v+2S)$                               | $v$    | $O(T+4S)$                        | $O(T*2^v *2S)$                            |
| $O(v+S)$                                 | $v$    | $O(n+S)$                         | $O(2^v *S^2)$                            |

Noting that $g(r)$ is evaluatable gate-by-gate in time $T=O(n+S)$. Finally, the soundness error is $dv/| \mathbb F|= 2Sv/| \mathbb F|$ .

The remark about IP and PSPACE is a bit abstract, but I interpret it as relying on the statement:
> PSPACE is equivalent to `#SAT`

which the majority of the remark attempts to justify. If we take that for granted, we've shown $IP\supseteq \#SAT$ => IP=PSPACE.

### Counting triangles in Graphs
Gives a brute force algorithm to count triangles in graphs, and a verifying algorithm to check it.  Taking our graph as an adjacency matrix (see book), where we define $f_A(x,y)$ = "is there an edge between vertices (x,y)". Then a natural BF to count triangles would be:

$$\Delta = \frac 1 6 \sum_{x,y,z} f_A(x,y)*f_A(y,z)*f_A(x,z)$$

where $x,y,z\in \{0,1\}^{\log n}$, so we have our function to sumcheck in $\Delta$. 

#### Analysis
Let n be the dimension of the Adjacency graph. The degree of each $g_i$ will be 3. A single evaluation of $g$ will run in $O(n^2)$ time, which will dominate the Verifier's time as we compute next.
| Communication                            | Rounds      | V time                           | P time                                    |
| ---------------------------------------- | ----------- | -------------------------------- | ----------------------------------------- |
| O$(2v+\sum deg_i(g))=^1O(v)$ field elems | $v=3\log n$ | $O(T+2*\sum deg_i(g))=^1 O(T+v)$ | $O(\sum deg_i(g)2^{v-i}T) =^1 O(T*2^{v})$ |
| O$(6\log n+3v) = O(\log n)$               | $3\log n$         | $O(n^2+2*9\log n)=O(n^2+\log n)=O(n^2)$ | $O(\sum deg_i(g)2^{v-i}T) = O(2^{3 \log n}*n^2)=O(n^5)$ |

Our soundness bound will be $dv/| \mathbb F|=9\log n / | \mathbb F|$. 
