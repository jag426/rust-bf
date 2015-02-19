#![feature(core, io, plugin)]
#![plugin(peg_syntax_ext)]

// AST. For parsing.
pub mod ast {
    use std::iter::FromIterator;

    #[derive(Clone, Debug)]
    pub enum Node {
        Loop(Vec<Node>),
        Move(isize),
        Add(i32),
        Output,
        Input,
    }

    #[derive(Clone, Debug)]
    pub struct Program(pub Vec<Node>);

    impl Program {
        pub fn from_source_string(src: String) -> Self {
            grammar::program(src.as_slice()).unwrap()
        }
    }

    pub fn parse(src: String) -> Program {
        Program::from_source_string(FromIterator::from_iter(src.as_slice().chars()
            .filter(|&c| c == '[' || c == ']' || c == '<' || c == '>' ||
                         c == '-' || c == '+' || c == '.' || c == ',')))
    }

    peg! grammar(r#"
    use super::{Node, Program};
    #[pub]
    program -> Program
        = ns:node* { Program(ns) }
    node -> Node
        = "[" ns:node* "]" { Node::Loop(ns) }
        / s:shift { Node::Move(s) }
        / a:add { Node::Add(a) }
        / "." { Node::Output }
        / "," { Node::Input }
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
        pub fn from_ast_node(c: &ast::Node) -> Self {
            match c {
                &ast::Node::Move(o)      => Node::Move(o),
                &ast::Node::Add(a)       => Node::AddConst(0is, a),
                &ast::Node::Output       => Node::Output(0is),
                &ast::Node::Input        => Node::Input(0is),
                &ast::Node::Loop(ref ns) => Node::Loop(
                    ns.iter().map(|n| Node::from_ast_node(n)).collect()),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Program(pub Vec<Node>);

    impl Program {
        pub fn from_ast(a: &ast::Program) -> Self {
            let &ast::Program(ref ns) = a;
            Program(ns.iter().map(|n| Node::from_ast_node(n)).collect())
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

