#![feature(core, io, plugin)]
#![plugin(peg_syntax_ext)]

use std::iter::FromIterator;
use std::old_io::{Reader, Writer};
use std::string::String;

enum Command {
    Loop(Vec<Command>),
    Shift(isize),
    Add(i32),
    Output,
    Input,
}

peg! grammar(r#"
use super::Command;
#[pub]
program -> Vec<Command>
    = command*
command -> Command
    = "[" c:command* "]" { Command::Loop(c) }
    / s:shift { Command::Shift(s) }
    / a:add { Command::Add(a) }
    / "." { Command::Output }
    / "," { Command::Input }
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
            &Command::Loop(ref cs) => {
                while self.tape[self.pos] != 0u8 {
                    for c in cs.iter() {
                        self.execute(&c);
                    }
                }
            }
            &Command::Shift(s) => {
                let pos = self.pos as isize + s;
                let pos = if pos < 0 { 0 } else { pos };
                self.pos = pos as usize;
                while self.tape.len() <= self.pos {
                    self.tape.push(0u8);
                }
            }
            &Command::Add(a) => {
                self.tape[self.pos] += a as u8;
            }
            &Command::Output => {
                self.output.write_u8(self.tape[self.pos]).unwrap();
            }
            &Command::Input => {
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
