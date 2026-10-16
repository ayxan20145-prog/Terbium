#[derive(Debug, PartialEq)]
enum Token {
    Value(i64),

    Plus,
    Minus,
    Star,
    Slash,

    Semicolon,
    Eof,
}

struct Lexer {
    source: Vec<char>,
    position: usize,
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
}

fn main() {
    let mut lexer = Lexer::new("1 + 2;");

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);

        if token == Token::Eof {
            break;
        }
    }
}
