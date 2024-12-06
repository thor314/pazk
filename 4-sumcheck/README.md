# Sumcheck, in Rust and Python
If you came here from Twitter, or elsewhere on the interwebz, welcome! This is my repo for notes from the
Proofs, Arguments, and Zero Knowledge Study group. What's that you asked? It's a
study group that meets Wednesdays and Fridays to talk about learning
cryptographic engineering, and zero knowledge theory! If that floats your boat,
check out our discord group! 

Why both Rust and Python? Because, [I've
found](https://github.com/thor314/euler-rs-py) that it's more efficient to
prototype in a flexible language like Python, and subsequently translate my
programs to Rust! Prototyping in Python allows for faster iterations, quicker
debugging, and a better final code base when I translate to Rust.

## To run python tests:
```
$ pytest python/sumcheck.py
```
Note to self: I want to also try the nosetest and unittest testing frameworks in Python, but this seems pretty ergonomic; any function starting with the word "test_" gets interpreted as a test. Can also group tests into a class starting with the word "Test". Deets: https://docs.pytest.org/en/7.1.x/

## To run Rust tests:
```
$ cargo test
```
