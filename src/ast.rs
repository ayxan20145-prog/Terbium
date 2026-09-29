use crate::token::{Token, Type, Value};

#[derive(Debug)]
pub enum Statement {
    Decleration {
        name: String,
        value: Expression,
    },
    Output {
        values: Vec<Expression>,
    },
    Input {
        name: String,
        typee: Type,
    },
    Import {
        name: String,
    },
    If {
        condition: Expression,
        body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    Push {
        value: Expression,
    },
    Pop {
        name: String,
    },
    Label {
        name: String,
        body: Vec<Statement>,
    },
    Call {
        name: String,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
}

#[derive(Debug)]
pub enum Expression {
    Value(Value),
    Operation(Box<Expression>, Token, Box<Expression>),
    Variable(String),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
