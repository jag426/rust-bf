#![feature(core, io, plugin)]
#![plugin(peg_syntax_ext)]

pub use ast::Ast;
pub use ir::Ir;

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

// IR. For optimizations.
pub mod ir {
    use std::fmt;
    use super::ast;

    pub use self::LoopOrSeq::{Loop, Seq};
    #[derive(Debug)]
    pub enum LoopOrSeq {
        Loop(Vec<LoopOrSeq>),
        Seq(Vec<Offset>, Move),
    }

    impl LoopOrSeq {
        pub fn pretty_indent(&self, indent: &String) -> String {
            let mut ret = String::new();
            match self {
                &Loop(ref loss) => {
                    ret.push_str(indent.as_slice());
                    ret.push_str("while(p[0]) {\n");
                    let mut new_indent = indent.clone();
                    new_indent.push_str("  ");
                    for los in loss.iter() {
                        ret.push_str(los.pretty_indent(&new_indent).as_slice());
                    }
                    ret.push_str(indent.as_slice());
                    ret.push_str("}\n");
                }
                &Seq(ref os, ref m) => {
                    for o in os.iter() {
                        ret.push_str(o.pretty_indent(indent).as_slice());
                    }
                    ret.push_str(m.pretty_indent(indent).as_slice());
                }
            }
            ret
        }
    }

    #[derive(Debug)]
    pub struct Move(isize);

    impl Move {
        pub fn pretty_indent(&self, indent: &String) -> String {
            let &Move(s) = self;
            let mut ret = String::new();
            if s == 0is {
                return ret;
            }
            ret.push_str(indent.as_slice());
            ret.push_str(format!("p += {}\n", s).as_slice());
            ret
        }
    }

    #[derive(Clone, Debug)]
    pub enum Offset {
        Add(isize, i32),
        Output(isize),
        Input(isize),
    }

    impl Offset {
        pub fn pretty_indent(&self, indent: &String) -> String {
            use self::Offset::{Add, Output, Input};

            let mut ret = String::new();
            ret.push_str(indent.as_slice());
            match self {
                &Add(o, a) => {
                    ret.push_str(format!("p[{}] += {}\n", o, a).as_slice());
                }
                &Output(o) => {
                    ret.push_str(format!("putchar(p[{}])\n", o).as_slice());
                }
                &Input(o) => {
                    ret.push_str(format!("p[{}] = getchar()\n", o).as_slice());
                }
            }
            ret
        }
    }

    #[derive(Debug)]
    pub struct Ir {
        commands: Vec<LoopOrSeq>,
    }

    impl Ir {
        fn ir_seq_from_ast_seq(aseq: &Vec<ast::Command>) -> Vec<LoopOrSeq> {
            let mut commands = Vec::new();
            let mut simpleseq = Vec::new();
            let mut offset = 0is;
            for n in aseq.iter() {
                match n {
                    &ast::Command::Loop(ref aloop) => {
                        commands.push(Seq(simpleseq, Move(offset)));
                        simpleseq = Vec::new();
                        offset = 0is;
                        commands.push(Loop(Ir::ir_seq_from_ast_seq(aloop)));
                    }
                    &ast::Command::Move(s) => {
                        offset += s;
                    }
                    &ast::Command::Add(a) => {
                        simpleseq.push(Offset::Add(offset, a));
                    }
                    &ast::Command::Output => {
                        simpleseq.push(Offset::Output(offset));
                    }
                    &ast::Command::Input => {
                        simpleseq.push(Offset::Input(offset));
                    }
                }
            }
            if simpleseq.len() > 0us || offset != 0is {
                commands.push(Seq(simpleseq, Move(offset)));
            }
            commands
        }

        pub fn from_ast(ast: ast::Ast) -> Self {
            let ast::Ast(commands) = ast;
            Ir {
                commands: Ir::ir_seq_from_ast_seq(&commands),
            }
        }

        pub fn reorder_adds(&mut self) {
            fn reorder_offsets(seq: &mut Vec<Offset>) {
                let mut i = 0us;
                while i < seq.len() {
                    match seq[i] {
                        Offset::Add(oi, mut ai) => {
                            let mut j = i + 1us;
                            while j < seq.len() {
                                match seq[j] {
                                    Offset::Add(oj, aj) if oi == oj => {
                                        ai += aj;
                                        seq.remove(j);
                                    }
                                    Offset::Add(_, _) => {
                                        j += 1us;
                                    }
                                    _ => {
                                        break;
                                    }
                                }
                            }
                            seq[i] = Offset::Add(oi, ai);
                        }
                        _ => {}
                    }
                    i += 1us;
                }
            }

            fn reorder_loss(loss: &mut Vec<LoopOrSeq>) {
                for los in loss.iter_mut() {
                    match los {
                        &mut Loop(ref mut v) => {
                            reorder_loss(v);
                        }
                        &mut Seq(ref mut v, _) => {
                            reorder_offsets(v);
                        }
                    }
                }
            }

            reorder_loss(&mut self.commands);
        }

        fn pretty(&self) -> String {
            let mut ret = String::new();
            for los in self.commands.iter() {
                ret.push_str(los.pretty_indent(&"".to_string()).as_slice());
            }
            ret
        }
    }

    impl fmt::Display for Ir {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            write!(f, "{}", self.pretty())
        }
    }
}

/*
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

}
*/
