#![feature(collections, io, os, path)]
#![allow(deprecated)]

extern crate brainfuck;

extern crate getopts;

fn main() {
    let args: Vec<String> = std::os::args();
    let ref program = args[0];

    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "display this help message and exit");

    let matches = match opts.parse(args.tail()) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("help") || matches.free.is_empty() {
        println!("{} - a brainfuck interpreter (written in Rust)", program);
        println!("");
        let brief = format!("Usage: {} [FILENAME]", program);
        print!("{}", opts.usage(brief.as_slice()));
        return;
    }

    let srcfile = matches.free[0].clone();
    let mut interpreter = brainfuck::Interpreter::new(
        std::old_io::File::open(&Path::new(srcfile)).read_to_end().unwrap(),
        std::old_io::stdio::stdin(),
        std::old_io::stdio::stdout()
    );
    interpreter.run();
}
