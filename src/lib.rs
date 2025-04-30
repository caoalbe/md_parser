use std::error::Error;

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
    let ast = run_ast(tokens);

    // WRITE
    println!("{ast}");

    Ok(())
}
