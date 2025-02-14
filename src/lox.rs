use std::io::{ stdin, stdout, BufRead, Write, Result };
use std::fs;
use std::path::Path;


pub struct Lox {
    has_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { has_error: false, }
    }

    pub fn run_file(&self, path: &str) -> Result<()> {
        let bytes = fs::read(Path::new(path))?;
        let contents = String::from_utf8(bytes)?;

        self.run(contents);

        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<()> {
        let stdin = stdin();
        let mut reader = stdin.lock();
    
        loop {
            print!("> ");
            stdout().flush()?;
    
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read == 0 {
                break;
            }
    
            let trimmed_line = line.trim();
            if !trimmed_line.is_empty() {
                self.run(trimmed_line.to_string());
            }
        }
        Ok(())
    }

    pub fn run(&self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
        }
    }

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, pos: &str, message: &str) {
        eprintln!("[Line {}] Error {}: {}", line, pos, message);

        self.has_error = true;
    }
}





























// use crate::scanner::Scanner;
// use crate::tokens::{ Token, TokenType };

// use std::fs;
// use std::io::{BufRead, Write, stdin, stdout, Result, Error, ErrorKind};
// use std::process;
// use std::path::Path;

// #[derive(Debug, Clone)]
// pub struct Lox {
//     has_error: bool,
// }

// impl Lox {
//     pub fn new() -> Self {
//         Lox { 
//             has_error: false,
//         }
//     }

//     pub fn run_file(&self, path: &str) -> Result<()> {
//         if !path.ends_with(".lox") {
//             return Err(Error::new(ErrorKind::InvalidInput, "Error: Only files with '.Lox' extension are supported."));
//         }
    
//         let bytes  = fs::read(Path::new(path))?;
//         let content = String::from_utf8(bytes).map_err(|_| {
//             Error::new(ErrorKind::InvalidData, "Failed to parse bytes into a valid UTF-8 string")
//         })?;
    
//         Lox::run(content);
    
//         if self.has_error {
//             process::exit(65);
//         }
    
//         Ok(())
//     }

//     pub fn run_prompt(&mut self) -> Result<()> {
//         let stdin = stdin();
//         let mut reader = stdin.lock();
    
//         loop {
//             print!("> ");
//             stdout().flush()?;
    
//             let mut line = String::new();
//             let bytes_read = reader.read_line(&mut line)?;
//             if bytes_read == 0 {
//                 break;
//             }
    
//             let trimmed_line = line.trim();
//             if !trimmed_line.is_empty() {
//                 Lox::run(trimmed_line.to_string());
//             }
//             self.has_error = false;
//         }
//         Ok(())
//     }

//     fn run(source: String) {
//         let mut scanner = Scanner::new(source);
//         let tokens = scanner.scan_tokens();

//         for token in tokens {
//             println!("{:?}", token);
//         }
//     }

//     // pub fn error(line: usize, message: &str) {
//     //     Lox::report(line, "", message);
//     // }

//     fn report(line: usize, position: &str, message: &str) {
//         eprintln!("[Line {}] Error {}: {}", line, position, message);
//     }

//     pub fn error(token: Token, message: &str) {
//         if token.token_type == TokenType::EOF {
//             Lox::report(token.line, " at end", message);
//         } else {
//             let position = format!("at {}", token.lexeme);
//             let position_ref: &str = &position;

//             Lox::report(token.line, position_ref, message);
//         }
//     }
// }
