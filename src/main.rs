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
}

fn main() {
    let lexer = Lexer::new("1 + 2;");
}
