use crate::scanner::Scanner;

use std::fs;
use std::process;
use std::io::{self, BufRead, Write};
use std::path::Path;

pub struct Lox {
    has_error: bool,
}

impl Lox {
    fn default() -> Self {
        Lox { has_error: false }
    }
    
    pub fn run_file(path: &str) -> io::Result<()> {
        let lox = Lox::default();
        let bytes = fs::read(Path::new(path))?;
        let content = String::from_utf8(bytes).expect("Failed to parse bytes");
        Lox::run(content);
        if lox.has_error {
            process::exit(65);
        }
        Ok(())
    }

    pub fn run_prompt() -> io::Result<()> {
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        let mut lox = Lox::default();
        loop {
            print!("> ");
            io::stdout().flush()?;
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read == 0 {
                break;
            }
            Lox::run(line.trim().to_string());
            lox.has_error = false;
        }

        Ok(())
    }

    fn run(source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        // For now, just print the tokens.
        for token in tokens {
            println!("{:?}", token);
        }
    }

    pub fn error(line: usize, message: &str) {
        Lox::report(line, "", message);
    }

    fn report(line: usize, position: &str, message: &str) {
        eprintln!("[Line {}] Error {}: {}", line, position, message);
    }
}
  
