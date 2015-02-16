#![feature(core, io, plugin)]
#![plugin(peg_syntax_ext)]

use std::iter::FromIterator;
use std::old_io::{Reader, Writer};
use std::string::String;

use Command::{Loop, Move, OffsetAdd, Output, Input};

enum Command {
    Loop(Vec<Command>),
    Move(isize),
    OffsetAdd(isize, i32),
    Output,
    Input,
}

peg! grammar(r#"
use super::Command;
use Command::{Loop, Move, OffsetAdd, Output, Input};
#[pub]
program -> Vec<Command>
    = command*
command -> Command
    = "[" c:command* "]" { Loop(c) }
    / s:shift { Move(s) }
    / a:add { OffsetAdd(0is, a) }
    / "." { Output }
    / "," { Input }
shift -> isize
    = ">" s:shift { s + 1is }
    / ">" { 1is }
    / "<" s:shift { s - 1is }
    / "<" { -1is }
add -> i32
    = "+" a:add { a + 1i32 }
    / "+" { 1i32 }
    / "-" a:add { a - 1i32 }
    / "-" { -1i32 }
"#);

pub struct Interpreter<R, W> where R: Reader, W: Writer {
    // the source of input and destination of output for the program
    input: R,
    output: W,
    // the array of byte cells
    tape: Vec<u8>,
    pos: usize,
}

impl<R, W> Interpreter<R, W> where R: Reader, W: Writer {
    pub fn new(input: R, output: W) -> Self {

        let mut tape = Vec::with_capacity(100us);
        tape.push(0u8);
        Interpreter {
            input: input,
            output: output,
            tape: tape,
            pos: 0us,
        }
    }

    fn execute(&mut self, command: &Command) {
        match command {
            &Loop(ref cs) => {
                while self.tape[self.pos] != 0u8 {
                    for c in cs.iter() {
                        self.execute(&c);
                    }
                }
            }
            &Move(s) => {
                let pos = self.pos as isize + s;
                let pos = if pos < 0 { 0 } else { pos };
                self.pos = pos as usize;
                while self.tape.len() <= self.pos {
                    self.tape.push(0u8);
                }
            }
            &OffsetAdd(s, a) => {
                let ipos = self.pos as isize + s;
                let pos = if ipos < 0 { 0us } else { ipos as usize };
                self.tape[pos] += a as u8;
            }
            &Output => {
                self.output.write_u8(self.tape[self.pos]).unwrap();
            }
            &Input => {
                self.tape[self.pos] = self.input.read_u8().unwrap();
            }
        }
    }

    pub fn interpret(&mut self, src: String) {
        let filtered: String = FromIterator::from_iter(src.as_slice().chars()
            .filter(|&c| c == '[' || c == ']' || c == '<' || c == '>' ||
                         c == '-' || c == '+' || c == '.' || c == ','));
        let program = grammar::program(filtered.as_slice()).unwrap();
        for c in program.iter() {
            self.execute(&c);
        }
    }
}
