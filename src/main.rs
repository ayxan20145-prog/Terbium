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

    Print,
    Println,
    LParen,
    RParen,

    Semicolon,
    Eof,
}

#[derive(Debug)]
enum Statement {
    Decleration { name: String, value: Value },
    Print { value: Value },
    Println { value: Value },
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
                } else if name == "print" {
                    Token::Print
                } else if name == "println" {
                    Token::Println
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
            Token::Print => self.parse_print(),
            Token::Println => self.parse_println(),
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

        let value = match self.current() {
            Token::Value(value) => {
                self.advance();
                value
            }
            _ => panic!("expected value"),
        };

        match (&typee, &value) {
            (Type::Int, Value::Int(_)) => {}
            (Type::Float, Value::Float(_)) => {}
            (Type::String, Value::String(_)) => {}
            _ => panic!("type mismatch"),
        }

        match self.current() {
            Token::Semicolon => self.advance(),
            _ => panic!("expected ';'"),
        }

        Statement::Decleration { name, value }
    }
    fn parse_print(&mut self) -> Statement {
        match self.current() {
            Token::Print => self.advance(),
            _ => panic!("expected 'print'"),
        }

        match self.current() {
            Token::LParen => self.advance(),
            _ => panic!("expected '('"),
        }

        let value = match self.current() {
            Token::Value(value) => {
                self.advance();
                value
            }
            _ => panic!("expected value"),
        };

        match self.current() {
            Token::RParen => self.advance(),
            _ => panic!("expected ')'"),
        }

        match self.current() {
            Token::Semicolon => self.advance(),
            _ => panic!("expected ';'"),
        }

        Statement::Print { value }
    }
    fn parse_println(&mut self) -> Statement {
        match self.current() {
            Token::Println => self.advance(),
            _ => panic!("expected 'println'"),
        }

        match self.current() {
            Token::LParen => self.advance(),
            _ => panic!("expected '('"),
        }

        let value = match self.current() {
            Token::Value(value) => {
                self.advance();
                value
            }
            _ => panic!("expected value"),
        };

        match self.current() {
            Token::RParen => self.advance(),
            _ => panic!("expected ')'"),
        }

        match self.current() {
            Token::Semicolon => self.advance(),
            _ => panic!("expected ';'"),
        }

        Statement::Println { value }
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
                match value {
                    Value::String(value) => {
                        bytecode.push_str(&format!("pushstr {}\n", value));
                    }
                    _ => {
                        bytecode.push_str(&format!("push {}\n", value));
                    }
                }

                bytecode.push_str(&format!("store {}\n", name));
            }
            Statement::Print { value } => {
                match value {
                    Value::String(value) => {
                        bytecode.push_str(&format!("pushstr {}\n", value));
                    }
                    _ => {
                        bytecode.push_str(&format!("push {}\n", value));
                    }
                }

                bytecode.push_str(&format!("print\n"));
            }
            Statement::Println { value } => {
                match value {
                    Value::String(value) => {
                        bytecode.push_str(&format!("pushstr {}\n", value));
                    }
                    _ => {
                        bytecode.push_str(&format!("push {}\n", value));
                    }
                }

                bytecode.push_str(&format!("print\n"));
                bytecode.push_str(&format!("println\n"));
            }
        }
    }

    bytecode
}
