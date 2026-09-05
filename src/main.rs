use clap::Parser as ClapParser;
use std::{fmt, fs};

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

    Plus,
    Minus,
    Star,
    Slash,

    IO,
    Output,
    Input,

    Semicolon,
    Eof,
}

#[derive(Debug)]
enum Statement {
    Decleration { name: String, value: Expression },
    Output { values: Vec<Expression> },
    Input { name: String, typee: Type },
}

#[derive(Debug, PartialEq, Clone)]
enum Type {
    Int,
    Float,
    String,
}

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Int(i32),
    Float(f64),
    String(String),
}

#[derive(Debug)]
enum Expression {
    Value(Value),
    Operation(Value, Token, Value),
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
                Token::Equals
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
                } else if name == "IO" {
                    Token::IO
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
                    Some('>') => {
                        self.advance();
                        Token::Output
                    }
                    _ => panic!("expected '>'"),
                }
            }
            Some('<') => {
                self.advance();

                match self.current() {
                    Some('<') => {
                        self.advance();
                        Token::Input
                    }
                    _ => panic!("expected '<'"),
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
            Token::IO => self.parse_io(),
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

            (Type::Int, Expression::Operation(Value::Int(_), _, Value::Int(_))) => {}
            (Type::Float, Expression::Operation(Value::Float(_), _, Value::Float(_))) => {}

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
    fn parse_expression(&mut self) -> Expression {
        let left = match self.current() {
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

        match self.current() {
            Token::Plus | Token::Minus | Token::Star | Token::Slash => {
                let op = self.current();
                self.advance();

                let right = match self.current() {
                    Token::Value(value) => {
                        self.advance();
                        value
                    }
                    _ => panic!("expected value"),
                };

                match left {
                    Expression::Value(value) => Expression::Operation(value, op, right),
                    Expression::Variable(_) => {
                        panic!("operations with variables not supported yet :(")
                    }
                    _ => panic!("error message"),
                }
            }
            _ => left,
        }
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
                }
                bytecode.push_str(&format!("store {}\n", name));
            }
        }
    }

    bytecode
}
fn compile_value(value: &Value) -> String {
    match value {
        Value::String(value) => format!("pushstr {}\n", value),
        _ => format!("push {}\n", value),
    }
}
fn compile_expression(expression: &Expression) -> String {
    match expression {
        Expression::Value(value) => compile_value(value),
        Expression::Variable(name) => format!("load {}\n", name),
        Expression::Operation(left, op, right) => {
            let mut bytecode = String::new();

            bytecode.push_str(&compile_value(left));
            bytecode.push_str(&compile_value(right));

            match op {
                Token::Plus => bytecode.push_str("add\n"),
                Token::Minus => bytecode.push_str("sub\n"),
                Token::Star => bytecode.push_str("mul\n"),
                Token::Slash => bytecode.push_str("div\n"),
                _ => panic!("invalid operator"),
            }

            bytecode
        }
    }
}
