mod lox;
mod scanner;
mod token;

use std::env;
use std::process;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        lox::Lox::run_file(&args[1])?;
    } else {
        lox::Lox::run_prompt()?;
    }

    Ok(())
}
