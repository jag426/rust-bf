# rust-bf
Brainf&$# interpreter in Rust. Uses rust-peg for parsing.

Current optimizations include folding together sequences of moves (</>) and sequences of increments/decrements (+/-) into single commands at parse-time.

Will eventually provide a compiler.
