use crate::ast::{Expression, Program, Statement};
use crate::token;
use crate::Lexer;

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    curToken: token::Token,
    peekToken: token::Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut p = Parser {
            lexer,
            curToken: token::Token::Illegal,
            peekToken: token::Token::Illegal,
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
        let mut p = Program {
            statements: Vec::new(),
        };

        p.statements.push(self.parse_let_statement().unwrap());
        p
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.curToken {
            token::Token::Let => Option::None,
            _ => Option::None,
        }
    }

    // fn expect_peek(&mut self, token: token::Token) -> bool {
    //     if self.peekToken == token {
    //         self.lexer.next();
    //         return true
    //     }
    //     false
    // }

    // TODO: want to return specificially a let statement though...
    fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.peekToken.is_identifier() {
            return Option::None;
        }
        let name = self.peekToken.clone();

        self.next_token();

        if !self.peekToken.is_assign() {
            return Option::None;
        }

        self.next_token();

        //TODO: skipping expres until we encounter semicolon
        while !self.curToken.is_semicolon() {
            self.next_token();
        }

        Option::Some(Statement::Let {
            name,
            value: Box::new(Expression::IntegerLiteral(42)),
        })
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_let_statement() {
        let lexer = Lexer::new(&String::from("let foo = 5;")).unwrap();
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        println!("{:?}", program.statements[0]);
    }

}