mod ast;
mod cli;
mod compiler;
mod lexer;
mod parser;
mod token;

use clap::Parser as ClapParser;
use std::fs;

use crate::{cli::Cli, compiler::compile, lexer::Lexer, parser::Parser};

fn main() {
    let args = Cli::parse();

    let content = fs::read_to_string(&args.input).expect("Failed to read program");

    let mut lexer = Lexer::new(&content);

    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);

    let program = parser.parse_program();

    let bytecode = compile(&program);

    fs::write(&args.output, bytecode).expect("Failed to write bytecode");
}
