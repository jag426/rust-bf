#![feature(io)]

use std::collections::DList;
use std::old_io::{Reader, Writer};

pub struct Interpreter<R, W> where R: Reader, W: Writer {
    // the program's source code and program counter
    program: Vec<u8>,
    pc: usize,
    // the source of input and destination of output for the program
    input: R,
    output: W,
    // an infinite array of bytes implemented as two stacks. current stores the
    // current cell's value.
    // e.g. movement to the left means pushing current onto stack2 and popping
    // from stack1 onto current
    stack1: DList<u8>,
    current: u8,
    stack2: DList<u8>,
}

impl<R, W> Interpreter<R, W> where R: Reader, W: Writer {
    pub fn new(program: Vec<u8>, input: R, output: W) -> Self {
        Interpreter {
            program: program,
            pc: 0us,
            input: input,
            output: output,
            stack1: DList::new(),
            current: 0u8,
            stack2: DList::new(),
        }
    }

    pub fn step(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }

        match self.program[self.pc] {
            62u8 => {
                // > 
                self.stack1.push_front(self.current);
                self.current = match self.stack2.pop_front() {
                    Some(c) => c,
                    None => 0u8,
                };
                self.pc += 1us;
            }
            60u8 => {
                // <
                // TODO noop if the pointer is all the way to the left already
                match self.stack1.pop_front() {
                    Some(c) => {
                        self.stack2.push_front(self.current);
                        self.current = c;
                    }
                    None => {}
                }
                self.pc += 1us;
            }
            43u8 => {
                // +
                self.current += 1u8;
                self.pc += 1us;
            }
            45u8 => {
                // -
                self.current -= 1u8;
                self.pc += 1us;
            }
            46u8 => {
                // .
                self.output.write_u8(self.current).unwrap();
                self.pc += 1us;
            }
            44u8 => {
                // ,
                self.current = self.input.read_u8().unwrap();
                self.pc += 1us;
            }
            91u8 => {
                // [
                if self.current == 0u8 {
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
                if self.current != 0u8 {
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
