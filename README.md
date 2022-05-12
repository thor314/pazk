# What is this?
This is where I'll be collecting resources related to the Study Group on Dr. Justin Thaler's [Proofs
Arguments And Zero Knowledge
Book](https://people.cs.georgetown.edu/jthaler/ProofsArgsAndZK.pdf). The notes
posted here will be copied out of my personal Obsidian note client; certain
features like image and latex embedding will not render on Github, though should
still be readable. If you would like to read them with latex rendering, a
hack solution might be to copy them to a HackMD document. 

The group open to everyone, and runs out of the [ZK hack discord
server](https://www.zkhack.dev/) by myself and the nice folks at the Zero Knowledge
Podcast.

Have a nice repo of your own? I'd love to link to it!
Questions about this resource? DM me on
[Twitter](https://twitter.com/cryptograthor).

## Links
- Bryan's explainer on [multilinear Lagrange interpolation](https://gist.github.com/bgillesp/4d020dde5bc04995ce21c5d5af3b55f3)
- A [Zoo](https://complexityzoo.net/Complexity_Zoo) to visit weird and wonderful Complexity classes (note that complexity theory is not required or even core knowledge for this book, but exists as one of many rabbit holes to dive into)

## Protocols to be implemented in the PAZK study group
A list of protocols to be implemented in Rust and Python by me over the course of the Thaler Study club, run out of the ZK hack discord server, published at github.com/thor314/pazk, in the (approximate) period April 2022-December 2022. The list is subject to change.

- Freivalds Algorithm
- Multilinear Lagrange Interpolation
- Sumcheck
- GKR
- Fiat Shamir transformation outline
- R1CS Front end over GKR
- A Succinct Circuit Satisfiability Interactive Proof
- Several Implementations of Polynomial Commitment schemes over:
	- Merkle Trees
	- Low degree tests
	- Both of the above
- A Multi-Prover Interactive Interactive prover for Circuit satisfiability
- A PCP of Quasilinear length for Circuit Sat
- A Polynomial IOP for R1CS-sat via Univariate sumcheck
- A Polynomial IOP from GKR
- A Polynomial IOP from Clover
- Minimal implementations of:
	- FRI
	- Ligero
- Schnorr's sigma protocol for knowledge of Discrete Log
- Sigma protocols, including Pedersen commitments
- Fiat Shamir applied to the above sigma protocols
- a Zero knowledge circuit via masking polynomials
- several Polynomial commitments from hardness of discrete log
- a naive bulletproof implementation
- KZG univariate polynomial commitments from pairings and a trusted setup, and extensions to multilinear polynomials
- The transparent Dory scheme
- Hydrax
- GGPR13
- Groth16
- Plonk, and modifications
