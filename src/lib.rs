use std::error::Error;
use std::fs::File;
use std::io::Write;

pub mod ast;
pub mod config;
pub mod lexer;

use ast::run_ast;
use config::Config;
use lexer::Token;
use lexer::run_lexer;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // LEXER
    let tokens: Vec<Token> = run_lexer(&config)?;

    // AST
    let ast: ast::Tree = run_ast(tokens);

    // WRITE
    let mut file: File = File::create(config.output_path)?;
    write!(file, "{}", ast)?;

    Ok(())
}
