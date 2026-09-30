use crate::token::{Token, Type, Value};

pub struct Lexer {
    source: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
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

            Some(',') => {
                self.advance();
                Token::Comma
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

            Some('[') => {
                self.advance();
                Token::LBracket
            }
            Some(']') => {
                self.advance();
                Token::RBracket
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
                } else if name == "push" {
                    Token::Push
                } else if name == "label" {
                    Token::Label
                } else if name == "call" {
                    Token::Call
                } else if name == "while" {
                    Token::While
                } else if name == "pop" {
                    Token::Pop
                } else {
                    Token::Name(name)
                }
            }

            Some(c) => {
                panic!("unexpected char: {}", c);
            }
        }
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
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
