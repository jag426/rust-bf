#![feature(core, int_uint, old_io, plugin)]
#![plugin(peg_syntax_ext)]

pub mod ast;
pub mod ir;

pub fn interpret<R: Reader, W: Writer>(src: String, input: &mut R, output: &mut W) {
    let ast = ast::parse(src);
    let ir = ir::Program::from_ast(&ast, true);
    ir.execute(input, output);
}

