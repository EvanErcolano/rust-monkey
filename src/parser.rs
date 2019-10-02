use crate::Lexer;
use crate::token;
use crate::ast::{Expression, Program, Statement};

#[derive(Debug)]
pub struct Parser {
    lexer:  Lexer,
    curToken : token::Token,
    peekToken: token::Token,
}

impl Parser {

    pub fn new(lexer : Lexer) -> Parser{
        let mut p = Parser {
            lexer,
            curToken : token::Token::Illegal,
            peekToken : token::Token::Illegal,
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn next_token(&mut self) {
        self.curToken = self.peekToken.clone();
        self.peekToken = self.lexer.next().unwrap();
    }

    pub fn parse_program(&mut self) -> Program {
        Program {
            statements: Vec::new(),
        }
    }
}