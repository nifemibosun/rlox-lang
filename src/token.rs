#[derive(Debug)]
#[derive(Clone)]
pub enum TokenType {
  // Single-character tokens.
  LPAREN, RPAREN, LBRACE, RBRACE,
  COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

  // One or two character tokens.
  BANG, BANGEQUAL,
  EQUAL, EQUALEQUAL,
  GREATER, GREATEREQUAL,
  LESS, LESSEQUAL,

  // Literals.
  IDENTIFIER, STRING, NUMBER,

  // Keywords.
  AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
  PRINT, RETURN, SUPER, THIS, TRUE, LET, WHILE,

  EOF
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Self {
        Token { 
          token_type, 
          lexeme, 
          literal, 
          line 
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
