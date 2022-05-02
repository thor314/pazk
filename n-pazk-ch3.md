#note #pazk #cryptography 
A short chapter covering preliminaries like the Schwartz Zippel Lemma, multilinear functions, low degree extensions, and several conceptual clarifications.
## key stuff
- interactive proof system definition: a pair of algorithms V,P, with sufficiently smol completeness and soundness error
- Argument system definition - an IP that is sound against polynomial time prover adversaries (computational soundness)
- Schwartz zippel lemma - gives upper bounds on multivariate polynomial collisions
- Multilinear polynomials - our main character in the next chapter
- 3.8 gives an efficient algorithm for computing multinomial extensions.

## notes
![[Pasted image 20220502135014.png]]

We tend to care mostly about these two properties, plus runtimes. Space usage, number of rounds, and communication cost are also worth analyzing.

Argument systems take IPs and specify that superpolynomial time prover strategies aren't worth caring about, presumably since we can simply raise our security parameter.

Section 3.3 clarifies several definitions. 
- All our protocols will naturally have perfect completeness, though we could just as easily use imperfectly complete ones.
- the soundness error convention of 1/3 is merely convention; soundness error in practice will be proportional to $1/|\mathbb F|$ 
- public coin randomness is what we will almost always use, so as to take advantage of the Fiat Shamir transformation. Any private coin IP can be simulated by a public coin IP.
- Deterministic malicious Prover strategies will naturally be stronger adversaries than Probalistic malicious prover strategies.

A passing, skimmable set of remarks is made on Complexity Theoretic notation's equivalence.

**Schwartz-Zippel**
the heavy lifting polynomial collision lemma. 
![[Pasted image 20220502140050.png]]

[The Lemma looks like the FT of Algebra to me](https://en.wikipedia.org/wiki/Fundamental_theorem_of_algebra),  but expanded to multiple variables. The lemma can concisely match the expectation that, if a polynomial $g$ over some space $\mathbb F$ is reduced to some set $S$, then $S$ will contain at most $deg(g)$ roots, so the probability that an arbitrary point in $S$ is a root of $g$ is at most $deg(g) / |S|$. 

The lemma will be applied to give upper bounds on the probability of a polynomial collisions, which we want to avoid.

## Multilinear extensions
Extensions should be thought of as distance amplifiers between functions that are "close", in the sense that they might differ at only one coefficient. Eg, take two "close" functions:

$$f_1(a,b) = a + b + ab$$
$$f_2(a,b) = a+b$$

| $f_1 - f_2 \in \mathbb F_2$ | 0   | 1   |
| --------------------------- | --- | --- |
| 0                           | 0   | 0   |
| 1                           | 0   | 1   | 

| $f_1 - f_2 \in \mathbb F_5$ | 0   | 1   | 2   | 3   | 4   |
| --------------------------- | --- | --- | --- | --- | --- |
| 0                           | 0   | 0   | 0   | 0   | 0   |
| 1                           | 0   | 1   | 2   | 3   | 4   |
| 2                           | 0   | 2   | 4   | 1   | 3   |
| 3                           | 0   | 3   | 1   | 4   | 2   |
| 4                           | 0   | 4   | 3   | 2   | 1   | 

Why did we do all that? To observe that, although $f_1$ and $f_2$ agree at most values in $\mathbb F_2$, they disagree at most values in $\mathbb F_5$. In that sense, the extension is **distance amplifying**.

A **multilinear** function is any polynomial whose terms are at most degree-1 in each variable.
Eg: $F(a,b,c) = a + b + ab + abc$

But not: $G(a,b,c) = a^2+b^2+c^2$

Fact 3.5 is reasonably intuitive, that there should only be a single multilinear extension for a function $f$, extending it from $\mathbb F$ to $\mathbb F'$. Multilinear extensions can be naturally calculated by **Lagrange Interpolation**.

Lemma 3.6 gives the defn for **Lagrange**. 

Lemmas 3.7 and 3.8 give methods for computing the Lagrange efficiently. 3.8 gives an $O(n)$ time and space algorithm, with dynamic programming and memoization.