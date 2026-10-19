use clap::Parser as ClapParser;
use std::{collections::HashMap, fmt, fs, process::Command};

#[derive(ClapParser, Debug)]
#[command(name = "terbc", version, about = "Terbium compiler")]
struct Cli {
    input: String,

    #[arg(short = 'o', long, default_value = "output.o")]
    output: String,
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Type(Type),
    Name(String),
    Equals,
    Value(Value),

    Plus,
    Minus,
    Star,
    Slash,

    Semicolon,
    Eof,
}

#[derive(Debug)]
enum Statement {
    Decleration {
        typee: Type,
        name: String,
        value: Expression,
    },
}

#[derive(Debug)]
enum Expression {
    Value(Value),
    Operation(Box<Expression>, Token, Box<Expression>),
    Variable(String),
}

#[derive(Debug, PartialEq, Clone)]
enum Type {
    Int,
    Char,
}

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Int(i32),
    Char(u8),
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

            Some(';') => {
                self.advance();
                Token::Semicolon
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

            Some('=') => {
                self.advance();
                Token::Equals
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

                Token::Value(Value::Int(number.parse().unwrap()))
            }

            Some('\'') => {
                self.advance();

                let byte = match self.current() {
                    None => panic!("unterminated char"),
                    Some('\\') => {
                        self.advance();
                        let esc = self.current().expect("unterminated escape");
                        self.advance();
                        match esc {
                            'n' => b'\n',
                            't' => b'\t',
                            'r' => b'\r',
                            '0' => 0,
                            '\\' => b'\\',
                            '\'' => b'\'',
                            other => panic!("unknown escape: \\{}", other),
                        }
                    }
                    Some(c) => {
                        self.advance();
                        let cp = c as u32;
                        if cp > 255 {
                            panic!("error message");
                        }
                        cp as u8
                    }
                };

                match self.current() {
                    Some('\'') => {
                        self.advance();
                        Token::Value(Value::Char(byte))
                    }
                    _ => panic!("expected closing quote"),
                }
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
                } else if name == "char" {
                    Token::Type(Type::Char)
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

        match self.current() {
            Token::Semicolon => self.advance(),
            _ => panic!("expected ';'"),
        }

        Statement::Decleration { typee, name, value }
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
                Token::Plus | Token::Minus | Token::Star | Token::Slash => {
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
    fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        while self.current() != Token::Eof {
            statements.push(self.parse_statement());
        }

        Program { statements }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{}", value),
            Value::Char(value) => write!(f, "{}", value),
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

    let assembly = compile(&program);

    fs::write("output.asm", assembly).unwrap();

    Command::new("nasm")
        .args(["-f", "elf64", "output.asm", "-o", &args.output])
        .status()
        .unwrap();

    fs::remove_file("output.asm").unwrap();
}

fn compile(program: &Program) -> String {
    let mut assembly = String::new();
    let mut variables: HashMap<String, (i32, Type)> = HashMap::new();
    let mut offset = 0;

    for statement in &program.statements {
        if let Statement::Decleration { typee, name, .. } = statement {
            offset += 8;
            variables.insert(name.clone(), (offset, typee.clone()));
        }
    }

    let stack_size = offset;

    assembly.push_str("global _start\n");
    assembly.push_str("section .text\n");
    assembly.push_str("_start:\n");
    assembly.push_str("    push rbp\n");
    assembly.push_str("    mov rbp, rsp\n");

    assembly.push_str(&format!("    sub rsp, {}\n", stack_size));

    for statement in &program.statements {
        match statement {
            Statement::Decleration { typee, name, value } => {
                compile_expression(&mut assembly, value, &variables);
                let (off, _) = &variables[name];
                match typee {
                    Type::Int => assembly.push_str(&format!("    mov [rbp - {}], rax\n", off)),
                    Type::Char => assembly.push_str(&format!("    mov [rbp - {}], al\n", off)),
                }
            }
        }
    }

    assembly.push_str("    mov rsp, rbp\n");
    assembly.push_str("    pop rbp\n");
    assembly.push_str("    mov rax, 60\n");
    assembly.push_str("    xor rdi, rdi\n");
    assembly.push_str("    syscall\n");

    assembly
}

fn compile_expression(
    assembly: &mut String,
    expr: &Expression,
    variables: &HashMap<String, (i32, Type)>,
) {
    match expr {
        Expression::Value(v) => {
            assembly.push_str(&format!("    mov rax, {}\n", v));
        }

        Expression::Variable(name) => {
            let (offset, typee) = variables
                .get(name)
                .unwrap_or_else(|| panic!("undefined variable: {}", name));
            match typee {
                Type::Int => assembly.push_str(&format!("    mov rax, [rbp - {}]\n", offset)),
                Type::Char => {
                    assembly.push_str(&format!("    movzx rax, byte [rbp - {}]\n", offset))
                }
            }
        }

        Expression::Operation(left, op, right) => {
            compile_expression(assembly, left, variables);
            assembly.push_str("    push rax\n");
            compile_expression(assembly, right, variables);
            assembly.push_str("    mov rcx, rax\n");
            assembly.push_str("    pop rax\n");

            match op {
                Token::Plus => assembly.push_str("    add rax, rcx\n"),
                Token::Minus => assembly.push_str("    sub rax, rcx\n"),
                Token::Star => assembly.push_str("    imul rax, rcx\n"),
                Token::Slash => {
                    assembly.push_str("    cqo\n");
                    assembly.push_str("    idiv rcx\n");
                }

                _ => unreachable!("non-operator token"),
            }
        }
    }
}
