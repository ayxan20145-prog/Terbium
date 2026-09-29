use crate::{
    ast::{Expression, Program, Statement},
    token::{Token, Type, Value},
};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
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
            Token::Push => self.parse_push(),
            Token::Pop => self.parse_pop(),
            Token::Label => self.parse_label(),
            Token::Call => self.parse_call(),
            Token::While => self.parse_while(),
            _ => panic!("expected statement"),
        }
    }
    pub fn parse_program(&mut self) -> Program {
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
    fn parse_push(&mut self) -> Statement {
        self.advance();

        let value = self.parse_expression();

        match self.current() {
            Token::Semicolon => self.advance(),
            _ => panic!("expected ';'"),
        }

        Statement::Push { value }
    }
    fn parse_pop(&mut self) -> Statement {
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

        Statement::Pop { name }
    }
    fn parse_label(&mut self) -> Statement {
        self.advance();

        let name = match self.current() {
            Token::Name(name) => {
                self.advance();
                name
            }
            _ => panic!("expected name"),
        };

        match self.current() {
            Token::LBrace => self.advance(),
            _ => panic!("expected '{{'"),
        }

        let mut body = Vec::new();

        while self.current() != Token::RBrace {
            body.push(self.parse_statement());
        }

        self.advance();

        Statement::Label { name, body }
    }
    fn parse_call(&mut self) -> Statement {
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

        Statement::Call { name }
    }
    fn parse_while(&mut self) -> Statement {
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

        Statement::While { condition, body }
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
