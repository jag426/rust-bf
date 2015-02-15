#![feature(io)]

use std::old_io::{Reader, Writer};

pub struct Interpreter<R, W> where R: Reader, W: Writer {
    // the program's source code and program counter
    program: Vec<u8>,
    pc: usize,
    // the source of input and destination of output for the program
    input: R,
    output: W,
    // the array of byte cells
    tape: Vec<u8>,
    pos: usize,
}

impl<R, W> Interpreter<R, W> where R: Reader, W: Writer {
    pub fn new(program: Vec<u8>, input: R, output: W) -> Self {
        let mut tape = Vec::with_capacity(100us);
        tape.push(0u8);
        Interpreter {
            program: program,
            pc: 0us,
            input: input,
            output: output,
            tape: tape,
            pos: 0us,
        }
    }

    pub fn step(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }

        match self.program[self.pc] {
            62u8 => {
                // > 
                self.pos += 1us;
                if self.pos == self.tape.len() {
                    self.tape.push(0u8);
                }
                self.pc += 1us;
            }
            60u8 => {
                // <
                // noop if the pointer is all the way to the left already
                if self.pos != 0us {
                    self.pos -= 1us;
                }
                self.pc += 1us;
            }
            43u8 => {
                // +
                self.tape[self.pos] += 1u8;
                self.pc += 1us;
            }
            45u8 => {
                // -
                self.tape[self.pos] -= 1u8;
                self.pc += 1us;
            }
            46u8 => {
                // .
                self.output.write_u8(self.tape[self.pos]).unwrap();
                self.pc += 1us;
            }
            44u8 => {
                // ,
                self.tape[self.pos] = self.input.read_u8().unwrap();
                self.pc += 1us;
            }
            91u8 => {
                // [
                if self.tape[self.pos] == 0u8 {
                    let mut open_brackets = 1us;
                    while open_brackets > 0us {
                        self.pc += 1us;
                        if self.pc >= self.program.len() {
                            return false;
                        }
                        match self.program[self.pc] {
                            91u8 => { open_brackets += 1us; }
                            93u8 => { open_brackets -= 1us; }
                            _    => {                       }
                        }
                    }
                }
                self.pc += 1us;
            }
            93u8 => {
                // ]
                if self.tape[self.pos] != 0u8 {
                    let mut open_brackets = 1us;
                    while open_brackets > 0us {
                        if self.pc == 0us {
                            return false;
                        }
                        self.pc -= 1us;
                        match self.program[self.pc] {
                            93u8 => { open_brackets += 1us; }
                            91u8 => { open_brackets -= 1us; }
                            _    => {                       }
                        }
                    }
                }
                self.pc += 1us;
            }
            _ => {
                // not a command; skip
                self.pc += 1us;
            }
        }

        true
    }

    pub fn run(&mut self) {
        while self.step() {}
    }
}
