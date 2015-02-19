#![feature(core, io, plugin)]
#![plugin(peg_syntax_ext)]

pub use ast::Ast;

// AST. For parsing.
pub mod ast {
    use std::fmt;
    use std::iter::FromIterator;

    use self::Command::{Loop, Move, Add, Output, Input};
    pub enum Command {
        Loop(Vec<Command>),
        Move(isize),
        Add(i32),
        Output,
        Input,
    }

    impl Command {
        pub fn pretty_indent(&self, indent: &String) -> String {
            let mut ret = String::new();
            match self {
                &Loop(ref cs) => {
                    ret.push_str(indent.as_slice());
                    ret.push_str("[\n");
                    let mut new_indent = indent.clone();
                    new_indent.push_str("  ");
                    for c in cs.iter() {
                        ret.push_str(c.pretty_indent(&new_indent).as_slice());
                    }
                    ret.push_str(indent.as_slice());
                    ret.push_str("]\n");
                }
                &Move(s) => {
                    ret.push_str(indent.as_slice());
                    ret.push_str(format!("m({})\n", s).as_slice());
                }
                &Add(a) => {
                    ret.push_str(indent.as_slice());
                    ret.push_str(format!("a({})\n", a).as_slice());
                }
                &Output => {
                    ret.push_str(indent.as_slice());
                    ret.push_str(".\n");
                }
                &Input => {
                    ret.push_str(indent.as_slice());
                    ret.push_str(",\n");
                }
            }
            ret
        }
    }

    pub struct Ast(pub Vec<Command>);

    impl Ast {
        pub fn parse(src: String) -> Self {
            Ast::from_source(FromIterator::from_iter(src.as_slice().chars()
                .filter(|&c| c == '[' || c == ']' || c == '<' || c == '>' ||
                             c == '-' || c == '+' || c == '.' || c == ',')))
        }

        pub fn from_source(src: String) -> Self {
            grammar::program(src.as_slice()).unwrap()
        }

        pub fn pretty(&self) -> String {
            let &Ast(ref cs) = self;
            let mut ret = String::new();
            for c in cs.iter() {
                ret.push_str(c.pretty_indent(&"".to_string()).as_slice());
            }
            ret
        }
    }

    impl fmt::Display for Ast {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            write!(f, "{}", self.pretty())
        }
    }

    peg! grammar(r#"
    use super::{Ast, Command};
    #[pub]
    program -> Ast
        = cs:command* { Ast(cs) }
    command -> Command
        = "[" cs:command* "]" { Command::Loop(cs) }
        / s:shift { Command::Move(s) }
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
}

pub mod ir {
    use super::ast;

    #[derive(Clone, Debug)]
    pub enum Node {
        // while (p[0]) { <contents> }
        Loop(Vec<Node>),
        // (o) => p += o
        Move(isize),
        // (o, a) => p[o] += a
        AddConst(isize, i32),
        // (o, m, a) => p[o] += m*a
        AddMult(isize, isize, i32),
        // (o, m) => p[o] += p[m]
        AddOffset(isize, isize),
        // (o) => p[o] = 0
        Zero(isize),
        // (o) => putchar(p[o])
        Output(isize),
        // (o) => p[o] = getchar()
        Input(isize),
    }

    impl Node {
        pub fn from_ast_command(c: &ast::Command) -> Self {
            use ast::Command::{Loop, Move, Add, Output, Input};
            match c {
                &Loop(ref cs) => Node::Loop(
                    cs.iter().map(|c| Node::from_ast_command(c)).collect()),
                &Move(o) => Node::Move(o),
                &Add(a) => Node::AddConst(0is, a),
                &Output => Node::Output(0is),
                &Input => Node::Input(0is),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Program(Vec<Node>);

    impl Program {
        pub fn from_ast(a: &ast::Ast) -> Self {
            let &ast::Ast(ref cs) = a;
            Program(cs.iter().map(|c| Node::from_ast_command(c)).collect())
        }
    }
}

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

    pub fn interpret(src: String, input: R, output: W) {
        // TODO
    }

    fn offset_pos(&mut self, offset: isize) -> usize {
        let ret = if offset >= 0is { self.pos + (  offset  as usize) }
                  else             { self.pos - ((-offset) as usize) };
        while self.tape.len() <= ret {
            self.tape.push(0u8);
        }
        ret
    }

    pub fn execute(&mut self, p: ir::Program) {
        // TODO
    }
}

