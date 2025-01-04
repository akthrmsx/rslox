use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal {
        value: Literal,
    },
    Variable {
        name: Token,
    },
    Assign {
        name: Token,
        value: Box<Expression>,
    },
    Call {
        callee: Token,
        paren: Token,
        arguments: Vec<Expression>,
    },
    Grouping {
        expression: Box<Expression>,
    },
    Unary {
        operator: Token,
        expression: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Logical {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Get {
        object: Box<Expression>,
        name: Token,
    },
    Set {
        object: Box<Expression>,
        name: Token,
        value: Box<Expression>,
    },
    This {
        keyword: Token,
    },
    Super {
        keyword: Token,
        method: Token,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}
