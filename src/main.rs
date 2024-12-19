mod ast;
mod lox;
mod parser;
mod scanner;
mod tokens;

use std::env::args;
use std::process;
use lox::Lox;

fn main() {
    let args: Vec<String> = args().collect();
    let mut lox = Lox::new();

    if args.len() > 2 {
        eprintln!("Usage: lox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        // Run a file
        let filename = &args[1];
        if let Err(err) = lox.run_file(filename) {
            eprintln!("Error running file: {}", err);
            process::exit(1);
        }
    } else {
        // Start REPL
        if let Err(err) = lox.run_prompt() {
            eprintln!("Error in REPL: {}", err);
            process::exit(1);
        }
    }
}