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
        use self::Node::*;
        match c {
            &ast::Node::Move(o)      => Move(o),
            &ast::Node::Add(a)       => AddConst(0is, a),
            &ast::Node::Output       => Output(0is),
            &ast::Node::Input        => Input(0is),
            &ast::Node::Loop(ref ns) => Loop(
                ns.iter().map(|n| Node::from_ast_node(n)).collect()),
        }
    }

    pub fn is_multiply_loop(&self) -> bool {
        use self::Node::*;

        fn is_simple_loop(ns: &Vec<Node>) -> bool {
            ns.iter().fold(true, |acc, n| acc && match n {
                &Move(_) | &AddConst(_, _) => true,
                _ => false,
            })
        }

        fn net_move(ns: &Vec<Node>) -> isize {
            ns.iter().fold(0is, |acc, n| acc + match n {
                &Move(m) => m,
                _ => 0is,
            })
        }

        fn net_zero_inc(ns: &Vec<Node>) -> i32 {
            let mut offset = 0is;
            let mut zero_inc = 0i32;
            for n in ns.iter() {
                match n {
                    &Move(m) => {
                        offset += m;
                    }
                    &AddConst(o, a) => {
                        if offset + o == 0is {
                            zero_inc += a;
                        }
                    }
                    _ => {}
                }
            }
            zero_inc
        }

        match self {
            &Loop(ref ns) => {
                is_simple_loop(ns) &&
                net_move(ns) == 0is &&
                net_zero_inc(ns) as u8 == -1 as u8
            }
            _ => false
        }
    }

    pub fn convert_multiply_loop(&self) -> Node {
        use self::Node::*;
        match self {
            &Loop(ref ns) if self.is_multiply_loop() => {
                let mut ret = Vec::new();
                let mut offset = 0is;
                for n in ns.iter() {
                    match n {
                        &Move(m) => {
                            ret.push(Move(m));
                            offset += m;
                        }
                        &AddConst(o, a) => {
                            if offset + o != 0is {
                                ret.push(AddMult(o, -offset, a));
                            }
                        }
                        _ => {
                            panic!("This is supposed to be a simple loop.");
                        }
                    }
                }
                assert!(offset == 0is);
                ret.push(Zero(0is));
                Loop(ret)
            }
            &Loop(ref ns) => Loop(ns.iter().map(|n| n.convert_multiply_loop()).collect()),
            _ => self.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Program(pub Vec<Node>);

impl Program {
    pub fn from_ast(a: &ast::Program, opt: bool) -> Self {
        let mut p = Program(a.0.iter().map(|n| Node::from_ast_node(n)).collect());
        if opt {
            p.opt_multiply();
            p.opt_moves();
        }
        p
    }

    pub fn opt_multiply(&mut self) {
        self.0 = self.0.iter().map(|n| n.convert_multiply_loop()).collect();
    }

    pub fn opt_moves(&mut self) {
        use self::Node::*;
        fn helper(ns: &Vec<Node>) -> Vec<Node> {
            let mut ret = Vec::new();
            let mut offset = 0is;
            for n in ns.iter() {
                match n {
                    &Loop(ref ns) => {
                        if offset != 0is {
                            ret.push(Move(offset));
                        }
                        ret.push(Loop(helper(ns)));
                        offset = 0is;
                    }
                    &Move(o) => {
                        offset += o;
                    }
                    &AddConst(o, a) => {
                        ret.push(AddConst(o + offset, a));
                    }
                    &AddMult(o, m, a) => {
                        ret.push(AddMult(o + offset, m + offset, a));
                    }
                    &Zero(o) => {
                        ret.push(Zero(o + offset));
                    }
                    &Output(o) => {
                        ret.push(Output(o + offset));
                    }
                    &Input(o) => {
                        ret.push(Input(o + offset));
                    }
                }
            }
            if offset != 0is {
                ret.push(Move(offset));
            }
            ret
        }

        *self = Program(helper(&self.0));
    }

    pub fn execute<R: Reader, W: Writer>(&self, input: &mut R, output: &mut W) {
        use self::Node::*;
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

        self.0.iter().fold((), |_, ref n| step(&n, &mut tape, &mut pos, input, output));
    }
}

#[cfg(test)]
mod test {
    use super::Node::{AddConst, AddMult, Input, Loop, Move, Output, Zero};

    #[test]
    fn test_is_multiply_loop() {
        assert!(!Loop(vec![]).is_multiply_loop());
        assert!(Loop(vec![AddConst(0, -1)]).is_multiply_loop());
        assert!(Loop(vec![Move(1), AddConst(-1, -1), Move(-1)]).is_multiply_loop());
        assert!(!Loop(vec![Move(1), AddConst(0, -1), Move(-1)]).is_multiply_loop());
        assert!(!Loop(vec![Move(1), AddConst(-1, -1)]).is_multiply_loop());
        assert!(!Loop(vec![AddMult(0, 0, 0), AddConst(0, -1)]).is_multiply_loop());
        assert!(!Loop(vec![Input(0), AddConst(0, -1)]).is_multiply_loop());
        assert!(!Loop(vec![Output(0), AddConst(0, -1)]).is_multiply_loop());
        assert!(!Loop(vec![Zero(0), AddConst(0, -1)]).is_multiply_loop());
        assert!(!Loop(vec![Loop(vec![]), AddConst(0, -1)]).is_multiply_loop());
        assert!(!Loop(vec![Move(1), AddConst(0, 1), Move(1), AddConst(0, 2), Move(-2)]).is_multiply_loop());
        assert!(Loop(vec![Move(1), AddConst(0, 1), AddConst(-1, -1), Move(1), AddConst(0, 2), Move(-2)]).is_multiply_loop());
        assert!(!Loop(vec![AddConst(0, -1), Move(1), AddConst(0, 1), AddConst(-1, 1), Move(-1)]).is_multiply_loop());
        assert!(!Loop(vec![AddConst(0, -2)]).is_multiply_loop());
    }
}

