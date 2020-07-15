use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer { input: input }
    }

    pub fn next_token(&mut self) -> Token {
        match self.next() {
            Some(token) => token,
            None => Token {
                token_type: TokenType::EOF,
                literal: TokenType::EOF.to_literal(),
            },
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    // book's func (l *Lexer) readChar()
    fn next(&mut self) -> Option<Token> {
        let mut input_chars = self.input.chars().peekable();
        let mut current_char: Option<char> = input_chars.next();
        let mut optional_tokenization: Option<String> = None;

        // check for terminating condition
        if current_char.is_none() {
            optional_tokenization = Some("".to_string());
            to_token(optional_tokenization)
        } else {
            // book's func (l *Lexer) skipWhitespace()
            let mut white_space = current_char;

            // while there are Some chars in white space
            // if char is white space call next() and loop
            // else set previous loop's next to current char and ws to None
            // if white space is None break
            while let Some(c) = white_space {
                if c.is_whitespace() {
                    white_space = input_chars.next();
                } else {
                    current_char = white_space;
                    white_space = None
                }
            }

            // book's func (l *Lexer) readIdentifier() string
            let mut letters = String::from("");
            let mut numbers = String::from("");

            // while there is Some chars in current char loop
            // if current char is None break
            while let Some(c) = current_char {
                //book's func isLetter(ch byte) bool
                if c.is_alphabetic() || c == '_' {
                    letters.push(c);
                    current_char = input_chars.next();
                } else if c.is_numeric() {
                    numbers.push(c);
                    current_char = input_chars.next();
                } else {
                    let mut current_string = "".to_string();
                    let mut next_string = "".to_string();

                    if let Some(c) = current_char {
                        current_string = c.to_string();
                    };
                    let next_char = input_chars.peek().map(|c| c.to_string());
                    if let Some(c) = next_char {
                        next_string = c;
                    };
                    match (current_string.as_ref(), next_string.as_ref()) {
                        ("=", "=") => {
                            optional_tokenization = Some(current_string.clone() + &next_string);
                            input_chars.next();
                        }
                        ("!", "=") => {
                            optional_tokenization = Some(current_string.clone() + &next_string);
                            input_chars.next();
                        }
                        _ => {
                            optional_tokenization =
                                current_char.map(|c| c.to_string().trim().to_string())
                        }
                    }
                    current_char = None;
                }
            }

            // step towards the terminating condition
            self.input = input_chars.collect();

            if letters.len() > 0
                && (letters.contains("fn")
                    || letters.contains("let")
                    || letters.contains("true")
                    || letters.contains("false")
                    || letters.contains("if")
                    || letters.contains("else")
                    || letters.contains("return"))
            {
                if let Some(s) = optional_tokenization {
                    self.input = s.clone() + &self.input
                }
                to_token(Some(letters))
            } else if letters.len() > 0 {
                if let Some(s) = optional_tokenization {
                    self.input = s.clone() + &self.input
                }
                to_token(Some("ident".to_string() + &letters))
            } else if numbers.len() > 0 {
                if let Some(s) = optional_tokenization {
                    self.input = s.clone() + &self.input
                }
                to_token(Some("int".to_string() + &numbers))
            } else {
                to_token(optional_tokenization)
            }
        }
    }
}

// book's func (l *Lexer) NextToken() token.Token
fn to_token(string: Option<String>) -> Option<Token> {
    match string.as_ref().map(String::as_str) {
        Some(";") => Some(Token {
            token_type: TokenType::SEMICOLON,
            literal: TokenType::SEMICOLON.to_literal(),
        }),
        Some("(") => Some(Token {
            token_type: TokenType::LPAREN,
            literal: TokenType::LPAREN.to_literal(),
        }),
        Some(")") => Some(Token {
            token_type: TokenType::RPAREN,
            literal: TokenType::RPAREN.to_literal(),
        }),
        Some(",") => Some(Token {
            token_type: TokenType::COMMA,
            literal: TokenType::COMMA.to_literal(),
        }),
        Some("+") => Some(Token {
            token_type: TokenType::PLUS,
            literal: TokenType::PLUS.to_literal(),
        }),
        Some("-") => Some(Token {
            token_type: TokenType::MINUS,
            literal: TokenType::MINUS.to_literal(),
        }),
        Some("=") => Some(Token {
            token_type: TokenType::ASSIGN,
            literal: TokenType::ASSIGN.to_literal(),
        }),
        Some("!") => Some(Token {
            token_type: TokenType::BANG,
            literal: TokenType::BANG.to_literal(),
        }),
        Some("*") => Some(Token {
            token_type: TokenType::ASTERISK,
            literal: TokenType::ASTERISK.to_literal(),
        }),
        Some("/") => Some(Token {
            token_type: TokenType::SLASH,
            literal: TokenType::SLASH.to_literal(),
        }),
        Some("<") => Some(Token {
            token_type: TokenType::LT,
            literal: TokenType::LT.to_literal(),
        }),
        Some(">") => Some(Token {
            token_type: TokenType::GT,
            literal: TokenType::GT.to_literal(),
        }),
        Some("==") => Some(Token {
            token_type: TokenType::EQ,
            literal: TokenType::EQ.to_literal(),
        }),
        Some("!=") => Some(Token {
            token_type: TokenType::NOTEQ,
            literal: TokenType::NOTEQ.to_literal(),
        }),
        Some("{") => Some(Token {
            token_type: TokenType::LBRACE,
            literal: TokenType::LBRACE.to_literal(),
        }),
        Some("}") => Some(Token {
            token_type: TokenType::RBRACE,
            literal: TokenType::RBRACE.to_literal(),
        }),
        Some("let") => Some(Token {
            token_type: TokenType::LET,
            literal: TokenType::LET.to_literal(),
        }),
        Some("fn") => Some(Token {
            token_type: TokenType::FUNCTION,
            literal: TokenType::FUNCTION.to_literal(),
        }),
        Some("true") => Some(Token {
            token_type: TokenType::TRUE,
            literal: TokenType::TRUE.to_literal(),
        }),
        Some("false") => Some(Token {
            token_type: TokenType::FALSE,
            literal: TokenType::FALSE.to_literal(),
        }),
        Some("if") => Some(Token {
            token_type: TokenType::IF,
            literal: TokenType::IF.to_literal(),
        }),
        Some("else") => Some(Token {
            token_type: TokenType::ELSE,
            literal: TokenType::ELSE.to_literal(),
        }),
        Some("return") => Some(Token {
            token_type: TokenType::RETURN,
            literal: TokenType::RETURN.to_literal(),
        }),
        Some("") => Some(Token {
            token_type: TokenType::EOF,
            literal: TokenType::EOF.to_literal(),
        }),
        Some(s) if s.starts_with("int") => {
            let lit: Vec<&str> = s.split("int").collect();
            Some(Token {
                token_type: TokenType::INT,
                literal: lit[1].to_string(),
            })
        }
        Some(s) if s.starts_with("ident") => {
            let lit: Vec<&str> = s.split("ident").collect();
            Some(Token {
                token_type: TokenType::IDENT,
                literal: lit[1].to_string(),
            })
        }
        Some(c) => Some(Token {
            token_type: TokenType::ILLEGAL,
            literal: c.to_string(),
        }),
        None => None,
    }
}

#[test]
fn test_next_token() {
    assert_eq!(2 + 2, 4);

    let input = "=+(){},;";
    let tests = vec![
        (TokenType::ASSIGN, "="),
        (TokenType::PLUS, "+"),
        (TokenType::LPAREN, "("),
        (TokenType::RPAREN, ")"),
        (TokenType::LBRACE, "{"),
        (TokenType::RBRACE, "}"),
        (TokenType::COMMA, ","),
        (TokenType::SEMICOLON, ";"),
        (TokenType::EOF, ""),
    ];

    let mut lex = Lexer::new(input.to_string());

    for tt in tests.iter() {
        let tok = lex.next_token();
        let (expected_type, expected_literal) = tt;

        assert_eq!(*expected_type, tok.token_type);
        assert_eq!(expected_literal.to_string(), tok.literal);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_source() {
        let input = String::from(
            "let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
 };

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
return true;
} else {
return false;
}

10 == 10;
10 != 9;
",
        );

        let tests = vec![
            (TokenType::LET, "let"),
            (TokenType::IDENT, "five"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "ten"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "10"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "add"),
            (TokenType::ASSIGN, "="),
            (TokenType::FUNCTION, "fn"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "x"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "y"),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::IDENT, "x"),
            (TokenType::PLUS, "+"),
            (TokenType::IDENT, "y"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "result"),
            (TokenType::ASSIGN, "="),
            (TokenType::IDENT, "add"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "five"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "ten"),
            (TokenType::RPAREN, ")"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::BANG, "!"),
            (TokenType::MINUS, "-"),
            (TokenType::SLASH, "/"),
            (TokenType::ASTERISK, "*"),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::INT, "5"),
            (TokenType::LT, "<"),
            (TokenType::INT, "10"),
            (TokenType::GT, ">"),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::IF, "if"),
            (TokenType::LPAREN, "("),
            (TokenType::INT, "5"),
            (TokenType::LT, "<"),
            (TokenType::INT, "10"),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::RETURN, "return"),
            (TokenType::TRUE, "true"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::ELSE, "else"),
            (TokenType::LBRACE, "{"),
            (TokenType::RETURN, "return"),
            (TokenType::FALSE, "false"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::INT, "10"),
            (TokenType::EQ, "=="),
            (TokenType::INT, "10"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::INT, "10"),
            (TokenType::NOTEQ, "!="),
            (TokenType::INT, "9"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::EOF, ""),
        ];

        let mut lex = Lexer::new(input);

        for tt in tests.iter() {
            let tok = lex.next_token();
            let (expected_type, expected_literal) = tt;

            println!("{:?} <~> {:?}", tt, tok);

            assert_eq!(*expected_type, tok.token_type);
            assert_eq!(expected_literal.to_string(), tok.literal);
        }
    }
}
