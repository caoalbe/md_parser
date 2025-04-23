use std::error::Error;

pub mod ast;
pub mod config;
pub mod lexer;

use ast::Node;
use ast::Node::*;
use ast::run_ast;
use config::Config;
use lexer::Token;
use lexer::TokenType::*;
use lexer::run_lexer;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // LEXER
    let tokens: Vec<Token> = run_lexer(&config)?;

    // just for debugging
    println!("   type - value");
    println!("---------------------");
    for token in tokens.iter() {
        match token.token_type {
            Prefix => {
                println!(" prefix - {}", token.value);
            }
            Suffix => {
                println!(" suffix - {}", token.value);
            }
            Literal => {
                println!("literal - {}", token.value);
            }
        }
    }
    println!("-=-=-=--=-=-=--=-=-=--=-=-=--=-=-=--=-=-=--=-=-=--=-=-=--=-=-=-");

    // AST
    let ast: Node = run_ast(tokens);
    match ast {
        Branch { children, .. } => {
            for node in children.iter() {
                match node {
                    Branch { tag, .. } => {
                        println!("<{}>...</{}>", tag, tag);
                    }
                    Leaf { tag, literal, .. } => {
                        println!("<{}>{}</{}>", tag, literal, tag);
                    }
                }
            }
        }
        Leaf { .. } => {
            println!("you shouldnt see this");
        }
    }

    // WRITE

    Ok(())
}
