use crate::lox::Lox;
use crate::token::{ Token, TokenType };

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner { 
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let new_token = Token::new(TokenType::EOF, "".to_string(), None, self.line);
        
        self.tokens.push(new_token);
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LPAREN),
            ')' => self.add_token(TokenType::RPAREN),
            '{' => self.add_token(TokenType::LBRACE),
            '}' => self.add_token(TokenType::RBRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                if self.match_next('=') {
                    self.add_token(TokenType::BANGEQUAL);
                } else {
                    self.add_token(TokenType::BANG);
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_token(TokenType::EQUALEQUAL);
                } else {
                    self.add_token(TokenType::EQUAL);
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenType::LESSEQUAL);
                } else {
                    self.add_token(TokenType::LESS);
                }
              }
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenType::GREATEREQUAL);
                } else {
                    self.add_token(TokenType::GREATER);
                }
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1
            }
            _ => Lox::error(self.line, "Unexpected character")
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let current_char = self.source[self.current..].chars().next().unwrap();
        self.current += current_char.len_utf8();

        current_char
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        let new_token = Token::new(token_type, text, literal, self.line);
        self.tokens.push(new_token);
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source[self.current..].chars().next().unwrap();
    }
}
