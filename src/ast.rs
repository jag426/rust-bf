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

#[cfg(test)]
mod test {
    use super::parse;

    #[test]
    fn test_parser() {
        use std::old_io::File;
        
        let src = File::open(&Path::new("test/hello.bf")).read_to_string().unwrap();
        let ast = parse(src);
        let aststring = format!("{:?}", ast);
        let expected = "Program([Loop([Input, Loop([Output]), Input, Output, Output, Input, Input, Input, Add(1), Input, Add(-1), Input, Move(0), Input, Loop([]), Output]), Add(8), Loop([Move(1), Add(4), Loop([Move(1), Add(2), Move(1), Add(3), Move(1), Add(3), Move(1), Add(1), Move(-4), Add(-1)]), Move(1), Add(1), Move(1), Add(1), Move(1), Add(-1), Move(2), Add(1), Loop([Move(-1)]), Move(-1), Add(-1)]), Move(2), Output, Move(1), Add(-3), Output, Add(7), Output, Output, Add(3), Output, Move(2), Output, Move(-1), Add(-1), Output, Move(-1), Output, Add(3), Output, Add(-6), Output, Add(-8), Output, Move(2), Add(1), Output, Move(1), Add(2), Output])";
        assert!(aststring == expected);
    }
}

