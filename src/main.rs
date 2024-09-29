use clap::Parser;
use core::panic;
use std::{
    fs::File,
    io::{BufWriter, Read, Write},
    path::Path,
    process::exit,
};

mod compiler;
mod interpreter;
mod tokens;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// the path to the file to run
    path: String,

    /// the path to save the output when compiling
    save_path: Option<String>,

    /// Whether to compile the program
    #[arg(short, long, default_value_t = false)]
    compile: bool,
}

fn main() {
    let args = Args::parse();

    // parse args
    let path_str = &args.path;
    let path = Path::new(path_str);
    if !path.is_file() {
        println!("Input must be the path to a file.");
        exit(1)
    }

    // read input programme
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("Failed to open file {}: {}", path_str, err),
    };
    let mut str = String::new();
    match file.read_to_string(&mut str) {
        Ok(_) => (),
        Err(err) => panic!("Failed to read file {}: {}", path_str, err),
    }

    // execute + output
    let tokens = tokens::Tokenizer::new(&str).tokenize();
    if args.compile {
        let result = compiler::compile(tokens, 1000);

        // save to file
        let save_path_str = &args.save_path.unwrap_or("./a.ll".to_string());
        let save_path = Path::new(save_path_str);
        let valid_extension = save_path
            .extension()
            .and_then(|x| x.to_str())
            .unwrap_or("")
            .eq("ll");
        if !valid_extension {
            panic!("The file provided is invalid, it must have a '.ll' extension")
        }

        let exists = save_path.exists();
        let save_file = match exists {
            true => {
                if !save_path.is_file() {
                    panic!("Cannot write to non-regular-file ar {:?}", save_path_str);
                }
                File::open(save_path).expect("Failed to open output file")
            }
            false => File::create(save_path).expect("Failed to create output file"),
        };
        BufWriter::new(save_file)
            .write_all(result.as_bytes())
            .expect("Failed to write to output file");
    } else {
        let mut interpreter = interpreter::Interpreter::new(1000, tokens);
        interpreter.exec();
    }
}
