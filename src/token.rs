use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Type(Type),
    Name(String),
    Equals,
    Value(Value),

    LParen,
    RParen,

    LBrace,
    RBrace,

    LBracket,
    RBracket,

    Plus,
    Minus,
    Star,
    Slash,

    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    EqualEqual,
    NotEqual,

    IO,
    Output,
    Input,

    Import,

    If,
    Else,

    Push,
    Pop,

    Label,
    Call,

    While,

    Semicolon,
    Comma,
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::String(value) => write!(f, "{}", value),
            Value::Bool(value) => write!(f, "{}", value),
        }
    }
}
