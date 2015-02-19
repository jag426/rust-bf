use super::ast;

#[derive(Clone, Debug)]
pub enum Node {
    // while (p[0]) { <contents> }
    Loop(Vec<Node>),
    // (o) => p += o
    Move(isize),
    // (o, a) => p[o] += a
    AddConst(isize, i32),
    // (o, m, a) => p[o] += p[m]*a
    AddMult(isize, isize, i32),
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

    pub fn execute<R: Reader, W: Writer>(&self, input: &mut R, output: &mut W) {
        use self::Node::{Loop, Move, AddConst, AddMult, Zero, Output, Input};
        let &Program(ref ns) = self;
        let mut tape = vec![0u8];
        let mut pos = 0us;

        fn offset(tape: &mut Vec<u8>, pos: &mut usize, offset: isize) -> usize {
            let ret = *pos as isize + offset;
            if ret < 0is {
                panic!("Tried to move pointer to the left of cell 0");
            }
            let ret = ret as usize;
            while tape.len() <= ret {
                tape.push(0u8);
            }
            ret
        }

        fn step<R: Reader, W: Writer>(n: &Node,
                                      tape: &mut Vec<u8>,
                                      pos: &mut usize,
                                      input: &mut R,
                                      output: &mut W) -> () {
            match n {
                &Loop(ref ns) => {
                    offset(tape, pos, 0is);
                    while tape[*pos] != 0u8 {
                        ns.iter().fold((), |_, ref n| step(&n, tape, pos, input, output));
                    }
                }
                &Move(o) => *pos = offset(tape, pos, o),
                &AddConst(o, a) => {
                    let o = offset(tape, pos, o);
                    tape[o] += a as u8
                }
                &AddMult(o, m, a) => {
                    let o = offset(tape, pos, o);
                    let m = offset(tape, pos, m);
                    tape[o] += tape[m] * a as u8
                }
                &Zero(o) => {
                    let o = offset(tape, pos, o);
                    tape[o] = 0u8
                }
                &Output(o) => {
                    let o = offset(tape, pos, o);
                    output.write_u8(tape[o]).unwrap()
                }
                &Input(o) => {
                    let o = offset(tape, pos, o);
                    tape[o] = input.read_u8().unwrap()
                }
            }
        }

        ns.iter().fold((), |_, ref n| step(&n, &mut tape, &mut pos, input, output));
    }
}

