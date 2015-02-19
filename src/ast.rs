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

