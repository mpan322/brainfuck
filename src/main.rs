use clap::Parser;
use core::panic;
use std::{fs::File, io::Read, path::Path, process::exit};

mod compiler;
mod interpreter;
mod tokens;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the file to compile
    #[arg(short, long)]
    path: String,

    /// Whether to compile the program
    #[arg(short, long, default_value_t = false)]
    compile: bool,
}

fn main() {
    let args = Args::parse();

    let path_str = &args.path;
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

    if args.compile {
        compiler::compile(tokens, 10);
    } else {
        let mut interpreter = interpreter::Interpreter::new(1000, tokens);
        interpreter.exec();
    }
}
