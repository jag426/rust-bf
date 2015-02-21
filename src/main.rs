#![feature(collections, int_uint, old_io, old_path, os)]
#![allow(deprecated)]

extern crate brainfuck;
extern crate getopts;

use std::old_io::{File, stdio};

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
    let src = File::open(&Path::new(srcfile)).read_to_string().unwrap();

    brainfuck::interpret(src, &mut stdio::stdin(), &mut stdio::stdout());
}

