use std::{env, fs};

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Let,
    Name(String),
    Equals,
    Value(i64),

    Print,
    LParen,
    RParen,

    Semicolon,
    Eof,
}

#[derive(Debug)]
enum Statement {
    Let { name: String, value: i64 },
    Print { value: i64 },
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

                loop {
                    match self.current() {
                        Some(c) => {
                            if c.is_ascii_digit() {
                                number.push(c);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        None => break,
                    }
                }

                Token::Value(number.parse().unwrap())
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

                if name == "let" {
                    Token::Let
                } else if name == "print" {
                    Token::Print
                } else {
                    Token::Name(name)
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
            Token::Let => self.parse_let(),
            Token::Print => self.parse_print(),
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
    fn parse_let(&mut self) -> Statement {
        match self.current() {
            Token::Let => self.advance(),
            _ => panic!("expected 'let'"),
        }

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

        match self.current() {
            Token::Semicolon => self.advance(),
            _ => panic!("expected ';'"),
        }

        Statement::Let { name, value }
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
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: terbc <PATH>");
        return;
    }

    let content = fs::read_to_string(&args[1]).expect("Failed to read program");
    let mut lexer = Lexer::new(&content);

    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);

    let program = parser.parse_program();

    let bytecode = compile(&program);

    fs::write("program.tbc", bytecode).expect("Failed to write bytecode");
}

fn compile(program: &Program) -> String {
    let mut bytecode = String::new();

    for statement in &program.statements {
        match statement {
            Statement::Let { name, value } => {
                bytecode.push_str(&format!("push {}\n", value));
                bytecode.push_str(&format!("store {}\n", name));
            }
            Statement::Print { value } => {
                bytecode.push_str(&format!("push {}\n", value));
                bytecode.push_str(&format!("print\n"));
            }
        }
    }

    bytecode
}
