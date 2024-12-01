mod lox;
mod scanner;
mod tokens;

use std::env::args;
use std::process;
use std::io::Result;
use lox::Lox;

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        eprintln!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        Lox::run_file(&args[1])?;
    } else {
        Lox::run_prompt()?;
    }

    Ok(())
}
