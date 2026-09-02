#[derive(Debug, PartialEq, Clone)]
enum Token {
    Let,
    Name(String),
    Equals,
    Value(i64),
    Semicolon,
    Eof,
}

#[derive(Debug)]
enum Statement {
    Let { name: String, value: i64 },
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
    fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        while self.current() != Token::Eof {
            statements.push(self.parse_statement());
        }

        Program { statements }
    }
}

fn main() {
    let mut lexer = Lexer::new("let x = 5; let y = 10;");

    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);

    let program = parser.parse_program();

    println!("{:#?}", program);
}
