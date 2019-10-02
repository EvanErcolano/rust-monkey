// use crate:: means its local to this source tree
use crate::token;

pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Expression,
    Return,
    Let {name: token::Token, value: Box<Expression>},
}

#[derive(Debug)]
pub enum Expression {
    IntegerLiteral(usize),
    InfixExpression { left: Box<Expression>, operator: token::Token, right: Box<Expression> },
}