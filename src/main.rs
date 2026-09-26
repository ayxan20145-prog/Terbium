use clap::Parser as ClapParser;
use std::{fmt, fs};

const STD_MATH: &str = include_str!("../std/math.tbc");

#[derive(ClapParser, Debug)]
#[command(name = "terbc", version, about = "Terbium bytecode compiler")]
struct Cli {
    input: String,

    #[arg(short = 'o', long, default_value = "program.tbc")]
    output: String,
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Type(Type),
    Name(String),
    Equals,
    Value(Value),

    LParen,
    RParen,

    LBrace,
    RBrace,

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

    Semicolon,
    Eof,
}

#[derive(Debug)]
enum Statement {
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
}

#[derive(Debug, PartialEq, Clone)]
enum Type {
    Int,
    Float,
    String,
    Bool,
}

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug)]
enum Expression {
    Value(Value),
    Operation(Box<Expression>, Token, Box<Expression>),
    Variable(String),
}

struct Lexer {
    source: Vec<char>,
    position: usize,
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

#[derive(Debug)]
struct Program {
    statements: Vec<Statement>,
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
impl Lexer {
    fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
        }
    }
    fn current(&self) -> Option<char> {
        self.source.get(self.position).copied()
    }
    fn advance(&mut self) {
        self.position += 1;
    }
    fn next_token(&mut self) -> Token {
        loop {
            match self.current() {
                Some(c) => {
                    if c.is_whitespace() {
                        self.advance();
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }

        match self.current() {
            None => Token::Eof,

            Some('=') => {
                self.advance();

                match self.current() {
                    Some('=') => {
                        self.advance();
                        Token::EqualEqual
                    }
                    _ => Token::Equals,
                }
            }

            Some(';') => {
                self.advance();
                Token::Semicolon
            }

            Some('(') => {
                self.advance();
                Token::LParen
            }
            Some(')') => {
                self.advance();
                Token::RParen
            }

            Some('{') => {
                self.advance();
                Token::LBrace
            }
            Some('}') => {
                self.advance();
                Token::RBrace
            }

            Some(c) if c.is_ascii_digit() => {
                let mut number = String::new();
                let mut is_float = false;

                loop {
                    match self.current() {
                        Some(c) if c.is_ascii_digit() => {
                            number.push(c);
                            self.advance();
                        }

                        Some('.') if !is_float => {
                            is_float = true;
                            number.push('.');
                            self.advance();
                        }

                        _ => break,
                    }
                }

                if is_float {
                    Token::Value(Value::Float(number.parse().unwrap()))
                } else {
                    Token::Value(Value::Int(number.parse().unwrap()))
                }
            }

            Some('"') => {
                self.advance();

                let mut string = String::new();

                loop {
                    match self.current() {
                        Some('"') => {
                            self.advance();
                            break;
                        }

                        Some(c) => {
                            string.push(c);
                            self.advance();
                        }

                        None => panic!("unclosed string"),
                    }
                }

                Token::Value(Value::String(string))
            }

            Some(c) if c.is_alphabetic() => {
                let mut name = String::new();

                loop {
                    match self.current() {
                        Some(c) => {
                            if c.is_alphanumeric() {
                                name.push(c);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        None => break,
                    }
                }

                if name == "int" {
                    Token::Type(Type::Int)
                } else if name == "float" {
                    Token::Type(Type::Float)
                } else if name == "string" {
                    Token::Type(Type::String)
                } else if name == "bool" {
                    Token::Type(Type::Bool)
                } else if name == "true" || name == "false" {
                    Token::Value(Value::Bool(name.parse().unwrap()))
                } else if name == "IO" {
                    Token::IO
                } else if name == "import" {
                    Token::Import
                } else if name == "if" {
                    Token::If
                } else if name == "else" {
                    Token::Else
                } else {
                    Token::Name(name)
                }
            }

            Some('#') => {
                loop {
                    match self.current() {
                        Some(c) => {
                            if c == '\n' {
                                break;
                            }

                            self.advance();
                        }
                        None => break,
                    }
                }

                self.next_token()
            }

            Some('+') => {
                self.advance();
                Token::Plus
            }
            Some('-') => {
                self.advance();
                Token::Minus
            }
            Some('*') => {
                self.advance();
                Token::Star
            }
            Some('/') => {
                self.advance();
                Token::Slash
            }

            Some('>') => {
                self.advance();

                match self.current() {
                    Some('=') => {
                        self.advance();
                        Token::GreaterEqual
                    }
                    Some('>') => {
                        self.advance();
                        Token::Output
                    }
                    _ => Token::Greater,
                }
            }
            Some('<') => {
                self.advance();

                match self.current() {
                    Some('=') => {
                        self.advance();
                        Token::LessEqual
                    }
                    Some('<') => {
                        self.advance();
                        Token::Input
                    }
                    _ => Token::Less,
                }
            }

            Some('!') => {
                self.advance();

                match self.current() {
                    Some('=') => {
                        self.advance();
                        Token::NotEqual
                    }
                    _ => panic!("expected '=' after '!'"),
                }
            }

            Some(c) => {
                panic!("unexpected char: {}", c);
            }
        }
    }
    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();

            if token == Token::Eof {
                tokens.push(Token::Eof);
                break;
            }

            tokens.push(token);
        }

        tokens
    }
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }
    fn current(&self) -> Token {
        self.tokens[self.position].clone()
    }
    fn advance(&mut self) {
        self.position += 1;
    }
    fn parse_statement(&mut self) -> Statement {
        match self.current() {
            Token::Type(Type::Int) => self.parse_decleration(),
            Token::Type(Type::Float) => self.parse_decleration(),
            Token::Type(Type::String) => self.parse_decleration(),
            Token::Type(Type::Bool) => self.parse_decleration(),
            Token::IO => self.parse_io(),
            Token::Import => self.parse_import(),
            Token::If => self.parse_if(),
            _ => panic!("expected statement"),
        }
    }
    fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        while self.current() != Token::Eof {
            statements.push(self.parse_statement());
        }

        Program { statements }
    }
    fn parse_decleration(&mut self) -> Statement {
        // rust doesnt let me use type as a variable name :(
        let typee = match self.current() {
            Token::Type(typee) => {
                self.advance();
                typee
            }
            _ => panic!("expected type"),
        };

        let name = match self.current() {
            Token::Name(name) => {
                self.advance();
                name
            }
            _ => panic!("expected name"),
        };

        match self.current() {
            Token::Equals => self.advance(),
            _ => panic!("expected '='"),
        }

        let value = self.parse_expression();

        match (&typee, &value) {
            (Type::Int, Expression::Value(Value::Int(_))) => {}
            (Type::Float, Expression::Value(Value::Float(_))) => {}
            (Type::String, Expression::Value(Value::String(_))) => {}
            (Type::Bool, Expression::Value(Value::Bool(_))) => {}

            (Type::Int, Expression::Operation(_, _, _)) => {}
            (Type::Float, Expression::Operation(_, _, _)) => {}

            _ => panic!("type mismatch"),
        }

        match self.current() {
            Token::Semicolon => self.advance(),
            _ => panic!("expected ';'"),
        }

        Statement::Decleration { name, value }
    }
    fn parse_io(&mut self) -> Statement {
        match self.current() {
            Token::IO => self.advance(),
            _ => panic!("expected 'IO'"),
        }

        match self.current() {
            Token::Output => {
                self.advance();

                let mut values = Vec::new();
                values.push(self.parse_expression());

                while self.current() == Token::Output {
                    self.advance();
                    values.push(self.parse_expression());
                }

                match self.current() {
                    Token::Semicolon => self.advance(),
                    _ => panic!("expected ';'"),
                }

                Statement::Output { values }
            }
            Token::Input => {
                self.advance();

                let typee = match self.current() {
                    Token::Type(typee) => {
                        self.advance();
                        typee
                    }
                    _ => panic!("expected type"),
                };

                let name = match self.current() {
                    Token::Name(name) => {
                        self.advance();
                        name
                    }
                    _ => panic!("expected variable name"),
                };

                match self.current() {
                    Token::Semicolon => self.advance(),
                    _ => panic!("expected ';'"),
                }

                Statement::Input { name, typee }
            }
            _ => panic!("expected '>>' or '<<'"),
        }
    }
    fn parse_import(&mut self) -> Statement {
        self.advance();

        let name = match self.current() {
            Token::Name(name) => {
                self.advance();
                name
            }
            _ => panic!("expected name"),
        };

        match self.current() {
            Token::Semicolon => self.advance(),
            _ => panic!("expected ';'"),
        }

        Statement::Import { name }
    }
    fn parse_if(&mut self) -> Statement {
        self.advance();

        let condition = self.parse_expression();

        match self.current() {
            Token::LBrace => self.advance(),
            _ => panic!("expected '{{'"),
        }

        let mut body = Vec::new();

        while self.current() != Token::RBrace {
            body.push(self.parse_statement());
        }

        self.advance();

        let else_body = if self.current() == Token::Else {
            self.advance();

            match self.current() {
                Token::LBrace => self.advance(),
                _ => panic!("expected '{{'"),
            }

            let mut body = Vec::new();

            while self.current() != Token::RBrace {
                body.push(self.parse_statement());
            }

            self.advance();

            Some(body)
        } else {
            None
        };

        Statement::If {
            condition,
            body,
            else_body,
        }
    }
    fn parse_expression(&mut self) -> Expression {
        let mut left = match self.current() {
            Token::Value(value) => {
                self.advance();
                Expression::Value(value)
            }

            Token::Name(name) => {
                self.advance();
                Expression::Variable(name)
            }
            _ => panic!("expected value"),
        };

        loop {
            let op = match self.current() {
                Token::Plus
                | Token::Minus
                | Token::Star
                | Token::Slash
                | Token::Less
                | Token::LessEqual
                | Token::Greater
                | Token::GreaterEqual
                | Token::EqualEqual
                | Token::NotEqual => {
                    let op = self.current();
                    self.advance();
                    op
                }

                _ => break,
            };

            let right = match self.current() {
                Token::Value(value) => {
                    self.advance();
                    Expression::Value(value)
                }

                Token::Name(name) => {
                    self.advance();
                    Expression::Variable(name)
                }
                _ => panic!("expected value"),
            };

            left = Expression::Operation(Box::new(left), op, Box::new(right));
        }

        left
    }
}

fn main() {
    let args = Cli::parse();

    let content = fs::read_to_string(&args.input).expect("Failed to read program");

    let mut lexer = Lexer::new(&content);

    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);

    let program = parser.parse_program();

    let bytecode = compile(&program);

    fs::write(&args.output, bytecode).expect("Failed to write bytecode");
}

fn compile(program: &Program) -> String {
    let mut bytecode = String::new();
    let mut label_id = 0;

    for statement in &program.statements {
        match statement {
            Statement::Decleration { name, value } => {
                bytecode.push_str(&compile_expression(value));
                bytecode.push_str(&format!("store {}\n", name));
            }
            Statement::Output { values } => {
                for value in values {
                    bytecode.push_str(&compile_expression(value));
                    bytecode.push_str("print\n");
                }
            }
            Statement::Input { name, typee } => {
                bytecode.push_str("read\n");
                match typee {
                    Type::Int => {
                        bytecode.push_str("stoi\n");
                    }
                    Type::Float => {
                        bytecode.push_str("stof\n");
                    }
                    Type::String => {}
                    Type::Bool => {
                        bytecode.push_str("stob\n");
                    }
                }
                bytecode.push_str(&format!("store {}\n", name));
            }
            Statement::Import { name } => match name.as_str() {
                "math" => {
                    bytecode.push_str(STD_MATH);
                }
                _ => panic!("unknown import: {}", name),
            },
            Statement::If {
                condition,
                body,
                else_body,
            } => {
                let if_label = format!("if_{}", label_id);
                let else_label = format!("else_{}", label_id);
                let end_label = format!("end_{}", label_id);
                label_id += 1;

                bytecode.push_str(&compile_expression(condition));

                bytecode.push_str(&format!("jumpif true {}\n", if_label));
                bytecode.push_str(&format!("jumpif false {}\n", else_label));

                bytecode.push_str(&format!("label {}\n", if_label));

                for statement in body {
                    match statement {
                        Statement::Output { values } => {
                            for value in values {
                                bytecode.push_str(&compile_expression(value));
                                bytecode.push_str("print\n");
                            }
                        }

                        _ => panic!("statement not supported in if yet :("),
                    }
                }

                bytecode.push_str(&format!("jump {}\n", end_label));

                bytecode.push_str(&format!("label {}\n", else_label));

                if let Some(else_body) = else_body {
                    for statement in else_body {
                        match statement {
                            Statement::Output { values } => {
                                for value in values {
                                    bytecode.push_str(&compile_expression(value));
                                    bytecode.push_str("print\n");
                                }
                            }

                            _ => panic!("statement not supported in else yet :("),
                        }
                    }
                    bytecode.push_str(&format!("jump {}\n", end_label));
                }

                bytecode.push_str(&format!("label {}\n", end_label));
            }
        }
    }

    bytecode
}
fn compile_value(value: &Value) -> String {
    match value {
        Value::String(value) => format!("pushstr {}\n", value),
        Value::Bool(value) => format!("pushbool {}\n", value),
        _ => format!("push {}\n", value),
    }
}
fn compile_expression(expression: &Expression) -> String {
    match expression {
        Expression::Value(value) => compile_value(value),
        Expression::Variable(name) => format!("load {}\n", name),
        Expression::Operation(left, op, right) => {
            let mut bytecode = String::new();

            bytecode.push_str(&compile_expression(left));
            bytecode.push_str(&compile_expression(right));

            match op {
                Token::Plus => bytecode.push_str("add\n"),
                Token::Minus => bytecode.push_str("sub\n"),
                Token::Star => bytecode.push_str("mul\n"),
                Token::Slash => bytecode.push_str("div\n"),

                Token::Less => bytecode.push_str("lt\n"),
                Token::LessEqual => bytecode.push_str("le\n"),
                Token::Greater => bytecode.push_str("gt\n"),
                Token::GreaterEqual => bytecode.push_str("ge\n"),
                Token::EqualEqual => bytecode.push_str("eq\n"),
                Token::NotEqual => bytecode.push_str("ne\n"),

                _ => panic!("invalid operator"),
            }

            bytecode
        }
    }
}
