// use crate:: means its local to this source tree
use crate::token;

pub struct Program {
    pub statements: Vec<Statement>,
}

pub enum Statement {
    Expression,
    Return,
    Let,
}

pub enum Expression {
    IntegerLiteral(usize),
    InfixExpression { left: Box<Expression>, operator: token::Token, right: Box<Expression> },
}