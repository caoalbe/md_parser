use std::error::Error;
use std::fs;

use crate::Config;
use TokenType::*;

pub enum TokenType {
    Prefix,
    Suffix,
    Literal,
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub fn run_lexer(config: &Config) -> Result<Vec<Token>, Box<dyn Error>> {
    let contents: String = match fs::read_to_string(&config.md_path) {
        Ok(text) => text,
        Err(io_err) => return Err(Box::new(io_err)),
    };

    let mut output: Vec<Token> = Vec::new();
    for line in contents.lines() {
        let words: Vec<&str> = line.trim().split(" ").collect();

        if words.len() <= 1 {
            if line == "" {
                output.push(Token {
                    token_type: Suffix,
                    value: String::from("empty_line"),
                })
            } else if line.chars().all(|c| c == '=') {
                output.push(Token {
                    token_type: Suffix,
                    value: String::from("h1"),
                })
            } else if line.chars().all(|c| c == '-') {
                output.push(Token {
                    token_type: Suffix,
                    value: String::from("h2"),
                })
            } else if line.chars().all(|c| c == '-' || c == '|') {
                output.push(Token {
                    token_type: Suffix,
                    value: String::from("table"),
                })
            } else {
                output.push(Token {
                    token_type: Literal,
                    value: String::from(line),
                })
            }
        } else {
            let prefix: &str = words[0];
            if prefix.chars().all(|c| c == '#') {
                let mut tag: String = String::from("h");
                tag.push_str(prefix.len().to_string().as_str());  
                output.push(Token {
                    token_type: Prefix,
                    value: tag,
                });
                output.push(Token {
                    token_type: Literal,
                    value: String::from(&line[prefix.len()..]),
                });
            } else {
                output.push(Token {
                    token_type: Literal,
                    value: String::from(line),
                });
            }
        }
    }
    Ok(output)
}
