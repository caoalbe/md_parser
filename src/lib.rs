use std::error::Error;
use std::fs;

use TokenType::*;
use Node::*;

pub struct Config {
    pub md_path: String,
    pub output_path: String
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip path of executable

        // Read program arguments
        let md_path = match args.next() {
            Some(path) => path,
            None => return Err("No markdown file specified"),
        };

        let output_path = match args.next() {
            Some(path) => path,
            None => "output.html".to_string()
        };

        // TODO: Error check arguments <md_path>, <output_path>
        // ...

        Ok(Config { md_path, output_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // LEXER
    let tokens: Vec<Token> = lexer(&config)?;

    // just for debugging
    println!("   type - value");
    println!("---------------------");
    for token in tokens.iter() {
        match token.token_type {
            Prefix => { println!(" prefix - {}", token.value); }
            Suffix => { println!(" suffix - {}", token.value); }
            Literal => { println!("literal - {}", token.value); }
        }
    }
    println!("-=-=-=--=-=-=--=-=-=--=-=-=--=-=-=--=-=-=--=-=-=--=-=-=--=-=-=-");

    // AST
    let ast: Node = ast(tokens);
    match ast {
        Branch{tag: _, children} => {
            for node in children.iter() {
                match node {
                    Branch{tag, children: _} => { 
                        println!("<{}>...</{}>", tag, tag);
                    },
                    Leaf{tag, literal} => { 
                        println!("<{}>{}</{}>", tag, literal, tag);
                    }
                }
            }
        },
        Leaf{tag: _, literal: _} => { println!("you shouldnt see this"); }
    }

    // WRITE

    Ok(())
}

pub enum TokenType {
    Prefix, Suffix, Literal
}

// TODO: create an enum for each token
// smth like Prefix(h1) for token type

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

fn lexer(config: &Config) -> Result<Vec<Token>, Box<dyn Error>> {
    let contents: String = match fs::read_to_string(&config.md_path) {
        Ok(text) => text,
        Err(io_err) => { return Err(Box::new(io_err)) }
    };

    let mut output: Vec<Token> = Vec::new();
    for line in contents.lines() {
        let words: Vec<&str> = line.split(" ").collect();
        
        if words.len() < 2 {
            if line == "" {
                output.push(Token{ token_type: Suffix, value: String::from("empty_line") })
            } else if line.chars().all(|c| c == '=') {
                output.push(Token{ token_type: Suffix, value: String::from("h1") })  // modify
            } else if line.chars().all(|c| c == '-') {
                output.push(Token{ token_type: Suffix, value: String::from("h2") })
            } else if line.chars().all(|c| c == '-' || c == '|') {
                output.push(Token{ token_type: Suffix, value: String::from("table") })
            } else {
                output.push(Token{ token_type: Literal, value: String::from(line) })
            }
        } else {
            let prefix: &str = words[0];
            if prefix.chars().all(|c| c == '#') {
                output.push(Token{ token_type: Prefix, value:String::from("h1") }); // TODO: should not always be h1
                output.push(Token{ token_type: Literal, value: String::from(&line[prefix.len()..]) });
            } else {
                output.push(Token{ token_type: Literal, value: String::from(line) });
            }
        }

    }
    Ok(output)
}

// todo: add &parent field
pub enum Node {
    Branch {
        tag: String,
        children: Box<Vec<Node>>
    },
    Leaf {
        tag: String,
        literal: String
    }
}

fn ast(token_vec: Vec<Token>) -> Node {
    let mut output: Node = Branch {
        tag: "doc".to_string(),
        children: Box::new(Vec::new())
    };

    if token_vec.len() == 0 { return output; }

    let parent: &mut Node = &mut output;
    let mut open_tag: String = String::new();
    let mut open_content: String = String::new();

    // this closure adds a node to the ast
    let mut submit_node = |open_tag: &mut String, open_content: &mut String| {
        if let Branch { children, ..} = parent {
            children.push(Leaf{
                tag: open_tag.clone(),
                literal: open_content.clone()
            });

            open_tag.clear();
            open_content.clear();
        }
    };

    
    // prime first node
    match token_vec[0].token_type {
        Prefix => {
            // modify node
            open_tag = token_vec[0].value.clone();
        },
        Suffix => {
            // throw an error
        },
        Literal => {
            // modify node
            open_content = token_vec[0].value.clone();
        }
    }

    // TODO: fix performance with .clone overusage
    let mut i = 1;
    while i < token_vec.len() {

        match token_vec[i].token_type {
            Prefix => {
                // Submit node, then modify upcoming node
                submit_node(&mut open_tag, &mut open_content);
                open_tag = token_vec[i].value.clone();
            },
            Suffix => {
                // breakdown possible suffixes
                match token_vec[i].value.as_str() {
                    "empty_line" => {
                        match token_vec[i-1].token_type {
                            Prefix => {},
                            Suffix => {},
                            Literal => {
                                submit_node(&mut open_tag, &mut open_content);
                            }
                        }
                    },
                    "h1" => {
                        // Modify node, then submit
                        open_tag = token_vec[i].value.clone();
                        submit_node(&mut open_tag, &mut open_content);   
                    },
                    "table" => {
                        // Change parent node
                        
                    },
                    _ => {}
                }
            },
            Literal => {
                if open_tag == "" { open_tag = "p".to_string(); }
                match token_vec[i-1].token_type {
                    Prefix => {
                        // Modify node, then submit
                        open_content = token_vec[i].value.clone();
                        submit_node(&mut open_tag, &mut open_content);

                    },
                    Suffix => {
                        // Submit, then modify node
                        open_content = token_vec[i].value.clone();
                    },
                    Literal => {
                        // Submit, then modify node
                        submit_node(&mut open_tag, &mut open_content);
                        open_content = token_vec[i].value.clone();
                    }
                }
            }
        }

        i = i + 1;
    }

    output
}
