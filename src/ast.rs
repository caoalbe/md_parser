use crate::lexer::Token;
use crate::lexer::TokenType::*;
use Node::*;

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

pub fn run_ast(token_vec: Vec<Token>) -> Node {
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