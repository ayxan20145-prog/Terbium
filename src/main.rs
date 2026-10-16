#[derive(Debug, PartialEq, Clone)]
enum Token {
    Value(i64),

    Plus,
    Minus,
    Star,
    Slash,

    Semicolon,
    Eof,
}

#[derive(Debug)]
enum Statement {
    Expression(Expression),
}

#[derive(Debug)]
enum Expression {
    Value(i64),
    Operation(Box<Expression>, Token, Box<Expression>),
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
        let expr = self.parse_expression();

        match self.current() {
            Token::Semicolon => self.advance(),
            _ => panic!("expected ';'"),
        }

        Statement::Expression(expr)
    }
    fn parse_expression(&mut self) -> Expression {
        let mut left = match self.current() {
            Token::Value(value) => {
                self.advance();
                Expression::Value(value)
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

fn main() {
    let mut lexer = Lexer::new("1 + 2 - 1 * 2 / 1;");

    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);

    let program = parser.parse_program();

    println!("{:#?}", program);
}
