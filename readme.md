![](https://travis-ci.org/jag426/rust-bf.svg?branch=master)
# rust-bf
Brainf*** interpreter in Rust. Uses rust-peg for parsing.

Current optimizations include folding together sequences of moves (</>) and sequences of increments/decrements (+/-) into single commands at parse-time, detecting and converting multiplication loops, and removing most moves by adding offsets to commands in the IR.

Will eventually provide a compiler.
