enum Token {
    Let,
    Name(String),
    Equals,
    Value(i64),
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
    let lexer = Lexer::new("let x = 5;");
}
