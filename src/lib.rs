use std::error::Error;

pub mod ast;
pub mod config;
pub mod lexer;

// use ast::Node;
// use ast::Node::*;
use ast::run_ast;
use config::Config;
use lexer::Token;
use lexer::TokenType::*;
use lexer::run_lexer;

use ast::Content;

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
    let ast = run_ast(tokens);
    println!("COMPLETED - run_ast");

    println!("<{}>", ast.root.tag);

    // match &ast.root.value {
    //     Content::Children(vec_of_nodes) => {
    //         println!("length: {}", vec_of_nodes.borrow().len());
    //         for node in vec_of_nodes.borrow().iter() {
    //             println!("wf");
    //         }
    //     },
    //     _ => {}
    // }

    println!(
        "    <{}>...<{}>",
        match &ast.root.value {
            Content::Children(vec_of_nodes) => {
                if vec_of_nodes.borrow().len() > 0 {
                    vec_of_nodes.borrow()[0].tag.clone()
                } else {
                    "lmao".to_string()
                }
            }
            _ => "lmao".to_string(),
        },
        match &ast.root.value {
            Content::Children(vec_of_nodes) => {
                if vec_of_nodes.borrow().len() > 0 {
                    vec_of_nodes.borrow()[0].tag.clone()
                } else {
                    "lmao".to_string()
                }
            }
            _ => "lmao".to_string(),
        },
    );
    println!("</{}>", ast.root.tag);

    // println!("{:#?}", _ast);
    // match ast {
    //     Branch { children, .. } => {
    //         for node in children.iter() {
    //             match node {
    //                 Branch { tag, .. } => {
    //                     println!("<{}>...</{}>", tag, tag);
    //                 }
    //                 Leaf { tag, literal, .. } => {
    //                     println!("<{}>{}</{}>", tag, literal, tag);
    //                 }
    //             }
    //         }
    //     }
    //     Leaf { .. } => {
    //         println!("you shouldnt see this");
    //     }
    // }

    // WRITE

    Ok(())
}
