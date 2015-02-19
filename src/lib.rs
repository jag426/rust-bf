#![feature(core, io, plugin)]
#![plugin(peg_syntax_ext)]

pub mod ast;
pub mod ir;

pub fn interpret<R: Reader, W: Writer>(src: String, input: &mut R, output: &mut W) {
    let ast = ast::parse(src);
    let ir = ir::Program::from_ast(&ast);
    ir.execute(input, output);
}

