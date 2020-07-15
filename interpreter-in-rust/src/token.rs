#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    // Identifiers + literals
    // add, foobar, x, y, ...
    IDENT,
    // 134345
    INT,
    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NOTEQ,
    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl TokenType {
    pub fn to_literal(&self) -> String {
        match self {
            TokenType::ILLEGAL => "ILLEGAL".to_string(),
            TokenType::EOF => "".to_string(),
            TokenType::IDENT => "IDENT".to_string(),
            TokenType::INT => "INT".to_string(),
            TokenType::ASSIGN => "=".to_string(),
            TokenType::PLUS => "+".to_string(),
            TokenType::MINUS => "-".to_string(),
            TokenType::BANG => "!".to_string(),
            TokenType::ASTERISK => "*".to_string(),
            TokenType::SLASH => "/".to_string(),
            TokenType::LT => "<".to_string(),
            TokenType::GT => ">".to_string(),
            TokenType::EQ => "==".to_string(),
            TokenType::NOTEQ => "!=".to_string(),
            TokenType::COMMA => ",".to_string(),
            TokenType::SEMICOLON => ";".to_string(),
            TokenType::LPAREN => "(".to_string(),
            TokenType::RPAREN => ")".to_string(),
            TokenType::LBRACE => "{".to_string(),
            TokenType::RBRACE => "}".to_string(),
            TokenType::FUNCTION => "fn".to_string(),
            TokenType::LET => "let".to_string(),
            TokenType::TRUE => "true".to_string(),
            TokenType::FALSE => "false".to_string(),
            TokenType::IF => "if".to_string(),
            TokenType::ELSE => "else".to_string(),
            TokenType::RETURN => "return".to_string(),
        }
    }
}

//  cargo test -- --nocapture
#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::*;
    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let tests = vec![
            (TokenType::ASSIGN, "="),
            (TokenType::PLUS, "+"),
            (TokenType::LPAREN, "("),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "["),
            (TokenType::RBRACE, "]"),
            (TokenType::COMMA, ","),
            (TokenType::SEMICOLON, ";"),
            (TokenType::EOF, ""),
        ];

        let mut lex = Lexer::new(input.to_string());

        for (i, tt) in tests.iter().enumerate() {
            println!("{:?}, {:?}  it: {:?}", i, tt, lex.next_token());
        }
    }
}
