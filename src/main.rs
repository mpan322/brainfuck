use core::panic;
use std::{fs::File, io::Read, path::Path, process::exit};

mod interpreter;
mod tokens;
mod compiler;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: <binary> <path to file>.");
        exit(1);
    }

    let path_str = &args[1];
    let path = Path::new(path_str);
    if !path.is_file() {
        println!("Input must be the path to a file.");
        exit(1)
    }

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("Failed to open file {}: {}", path_str, err),
    };

    let mut str = String::new();
    match file.read_to_string(&mut str) {
        Ok(_) => (),
        Err(err) => panic!("Failed to read file {}: {}", path_str, err),
    }

    let tokens = tokens::Tokenizer::new(&str).tokenize();
    // let mut interpreter = interpreter::Interpreter::new(1000, tokens);
    // interpreter.exec();
    compiler::compile(tokens, 10);
}
