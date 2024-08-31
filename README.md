# Brainfuck CLI

## Description

This project is a CLI tool written in rust containing a iterpreter and compiler targeting LLVM IR for the [brainfuck](https://en.wikipedia.org/wiki/Brainfuck) esolang.
The goals in making this project were two fold.
1) Learn more about writing compilers.
2) Learn rust, this is my first project in the langauge.

## Usage
To run this programme you will need to have the rust toolchain installed.

### Building
* Go to the root of the project and execute.
```
cargo build -r
```
* This should create a new folder `target` in the project root.
* The binary can be found at `target/release/brainfuck`
* For more information on how to use the CLI run `target/release/brainfuck --help`



