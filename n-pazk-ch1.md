#note #book #pazk 

## Key stuff
- **verifiable computing** enables a prover to guarantee to a verifier that the prover performed the requested computation correctly 
- each letter in **zk-SNARK**
- Want: Completeness, Soundness, Efficiency

## Notes
An example of VC in practice: 
Let Prover be supercomputer; Verifer be phone.
Verifier wants expensive computation performed, but Prover has incentive to cheat Verifier. Prover has a lot of resources with which to do so. Prover can submit a *proof* to Verifier, which is cheap for Verifier to check, relative to actually doing the computation.

**Succinct** means the proofs are short. 

If the proof is **Non-Interactive**, the Prover sends/publishes a single message about the proof, as opposed to 2 or more messages back and forth between P and V.

Interactive Proofs (IPs) were developed in the 80's and 90's. A neat theoretical result is that **IP=PSPACE**, implying that all NP-complete problems (and more) can be expressed with IP's. 

How **Argument Systems** are constructed:
1. An Information-theoretically secure protocol is developed for 1 or more provers
2. Cryptography *forces* the prover to behave in the restricted way, often with some zk tech.

Eg, two paths to a *zk-SNARK*:
| Protocol   | Cryptography                           |
| ---------- | -------------------------------------- |
| IP/MIP/PCP | polynomial commitment, Fiat-Shamir, zk |
| Linear PCP | Hmo. Encryption or Pairing crypto      |

Which is to emphasize that zk isn't nec. an inherent property of an Argument system, but *a property that can be added onto a protocol*.

Properties we care about:
- **Completeness** - honest P always convinces V
- **Soundness** - dishonest P only convinces V with negl Pr: $\epsilon$
- Some other nice things: efficiency to compute and verify proofs
Note about soundness: we only care about *Cheating P running in Polynomial Time*. Intuitively, this means that if the Cheating prover strategy runs in exponential time, we can increase our security param to blow up the prover time.

Why are these proofs *non-trad?*
-> they're probabalistic, using some source of randomness. A cheating P will be able to fool V with some negl. Pr.

