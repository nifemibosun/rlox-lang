







































// use std::collections::btree_map::ValuesMut;

// use crate::lox::Lox;
// use crate::tokens::{ Token, TokenType };
// use crate::ast::Expr;

// struct Parser {
//     tokens: Vec<Token>,
//     current: i64,
// }

// impl Parser {
//     fn new(tokens: Vec<Token>) -> Self {
//         Parser { 
//             tokens,
//             current: 0
//         }
//     }

//     fn expression(&self) -> Expr {
//         return self.equality();
//     }

//     fn equality(&self) -> Expr {
//         let mut expr: Expr = self.comparison();
    
//         while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
//             let operator = self.previous();
//             let right = self.comparison();
//             expr = Expr::Binary {
//                 left: Box::new(expr),
//                 operator,
//                 right: Box::new(right),
//             };
//         }
    
//         return expr;
//     }

//     fn comparison(&self) -> Expr {
//         let mut expr: Expr = self.term();

//         while self.match_token(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
//             let operator: Token = self.previous();
//             let right: Expr = self.term();
//             expr = Expr::Binary {
//                 left: Box::new(expr),
//                 operator,
//                 right: Box::new(right),
//             };
//         }

//         return expr;
//     }

//     fn term(&self) -> Expr {
//         let mut expr: Expr = self.factor();
    
//         while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
//             let operator = self.previous();
//             let right = self.factor();
//             expr = Expr::Binary {
//                 left: Box::new(expr),
//                 operator,
//                 right: Box::new(right),
//             };
//         }
    
//         return expr;
//     }

//     fn factor(&self) -> Expr {
//         let mut expr: Expr = self.unary();
    
//         while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
//             let operator = self.previous();
//             let right = self.unary();
//             expr = Expr::Binary {
//                 left: Box::new(expr),
//                 operator,
//                 right: Box::new(right),
//             };
//         }
    
//         return expr;
//     }

//     fn unary(&self) -> Expr {
//         if self.match_token(vec![BANG, MINUS]) {
//             let operator = self.previous();
//             let right = self.unary();
//             let expr = Expr::Unary {
//                 operator,
//                 right: Box::new(right),
//             };
//             return expr;
//         }
      
//         return self.primary();
//     }

//     fn primary(&self) -> Expr {
//         if self.match_token(vec![TokenType::False]) {
//             return Expr::Literal(false)
//         }
//         if self.match_token(vec![TokenType::True]) {
//             return Expr::Literal(true)
//         }
//         if self.match_token(vec![TokenType::Nil]) {
//             return Expr::Literal(None)
//         }
//         if self.match_token(vec![TokenType::Number, TokenType::String]) {
//             return Expr::Literal(self.previous().literal)
//         }

//         if self.match_token(vec![TokenType::LParen]) {
//             let expr: Expr = self.expression();
//             consume(TokenType::RParen, "Expect ')' after expression.");
//             return Expr.Grouping { 
//                 expr
//             };
//         }
//     }

//     fn match_token(&self, types: Vec<TokenType>) -> bool {
//         for token_type in types {
//             if self.check(token_type) {
//                 self.advance();
//                 return true;
//             }
//         }
//         false
//     }

//     fn consume(expected: TokenType, message: &str) -> Token {
//         if check(expected) {
//             return advance();
//         }
    
//         panic!("{}", error(peek(), message));
//     }

//     fn check(&self, token_type: TokenType) -> bool {
//         if self.is_at_end() {
//             return false;
//         }
//         self.peek().token_type == token_type
//     }

//     fn advance(&self) -> Token {
//         if !self.is_at_end() {
//             self.current + 1;
//         }
//         return self.previous()
//     }

//     fn is_at_end(&self) -> bool {
//         return self.peek().token_type == TokenType::EOF;
//     }
    
//     fn peek(&self) -> Token {
//     return self.tokens.get(self.current);
//     }
    
//     fn previous(&self) -> Token {
//         return self.tokens.get(self.current - 1);
//     }

//     fn error(token: &Token, message: &str) -> ParseError {
//         Lox::error(token, message);
//         ParseError::new()
//     }
    
// }