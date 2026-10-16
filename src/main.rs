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
}

fn main() {
    let mut lexer = Lexer::new("1 + 2;");
    println!("{:?}", lexer.current());
    lexer.advance();
    println!("{:?}", lexer.current());
}
