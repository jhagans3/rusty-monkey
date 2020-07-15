// use crate::ast::{Node, Program};
// use crate::lexer::Lexer;
// use crate::token::Token;

// pub struct Parser {
//     lex: Lexer,
//     curToken: Token,
//     peekToken: Token,
// }

// impl Parser {
//     pub fn new(mut l: Lexer) -> Self {
//         // book's func (p *Parser) nextToken()
//         let c = l.next_token();
//         let p = l.next_token();

//         Parser {
//             lex: l,
//             curToken: c,
//             peekToken: p,
//         }
//     }

//     pub fn parse_program(&self) -> Program {
//         Program::new()
//     }
// }
