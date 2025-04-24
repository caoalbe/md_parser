use std::cell::RefCell;
use std::fmt::format;
use std::rc::Rc;

use crate::lexer::Token;
use crate::lexer::TokenType::*;

pub enum Content {
    // TODO: i think we can remove the first Rc
    Children(Rc<RefCell<Vec<Rc<Node>>>>),
    Literal(String),
}

pub struct Node {
    parent: Option<Rc<Node>>,
    pub tag: String,
    pub value: Content,
}

pub struct Tree {
    pub root: Rc<Node>,
    curr: Rc<Node>,
    // size: usize,
}

impl Tree {
    // Builds a tree with a single root node, whose tag is "html"
    pub fn build() -> Tree {
        let root_node: Node = Node {
            parent: None,
            tag: "html".to_string(),
            value: Content::Children(Rc::new(RefCell::new(vec![]))),
        };

        let ptr: Rc<Node> = Rc::new(root_node);
        Tree {
            root: Rc::clone(&ptr),
            curr: Rc::clone(&ptr),
        }
    }

    // Inserts a leaf, as a child of curr node
    pub fn insert_leaf(&self, tag: &mut String, literal: &mut String) -> () {
        let to_add: Rc<Node> = Rc::new(Node {
            parent: Some(Rc::clone(&self.curr)),
            tag: std::mem::take(tag),
            value: Content::Literal(std::mem::take(literal)),
        });
        if let Content::Children(lst) = &self.curr.value {
            lst.borrow_mut().push(to_add);
        }
    }

    // Inserts a branch, as a child of curr node
    pub fn insert_branch(&mut self, tag: &mut String) -> () {
        let to_add: Rc<Node> = Rc::new(Node {
            parent: Some(Rc::clone(&self.curr)),
            tag: std::mem::take(tag),
            value: Content::Children(Rc::new(RefCell::new(vec![]))),
        });
        if let Content::Children(lst) = &self.curr.value {
            lst.borrow_mut().push(Rc::clone(&to_add));
        }

        self.curr = Rc::clone(&to_add);
    }

    // Moves curr pointer up to its parent
    fn curr_up(&mut self) -> () {
        if let Some(parent) = &self.curr.parent {
            self.curr = Rc::clone(&parent);
        }
    }

    pub fn debug_count(&self) -> usize {
        Rc::strong_count(&self.curr)
    }

    pub fn debug_curr_tag(&self) -> () {
        println!("curr tag: {}", self.curr.tag);
    }

    // Helper for the display trait.  This generates the string to print with the tab formatting
    fn display_helper(
        &self,
        builder: &mut String,
        target: &Node,
        depth: usize,
        tab_size: usize,
    ) -> () {
        builder.push_str(&" ".repeat(depth * tab_size));
        match &target.value {
            Content::Children(vec_node) => {
                builder.push_str(&format!("<{}>\n", target.tag));
                for node in vec_node.borrow().iter() {
                    self.display_helper(builder, node, depth + 1, tab_size);
                }
                builder.push_str(&" ".repeat(depth * tab_size));
                builder.push_str(&format!("<{}>\n", target.tag));
            }
            Content::Literal(text) => {
                builder.push_str(&format!("<{}>{}</{}>\n", target.tag, text, target.tag));
            }
        }
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: String = String::new();
        self.display_helper(&mut output, &self.root, 0, 4);
        write!(f, "{}", output)
    }
}

pub fn run_ast(mut token_vec: Vec<Token>) -> Tree {
    let output: Tree = Tree::build();

    if token_vec.is_empty() {
        return output;
    }

    let mut open_tag: String = String::new();
    let mut open_text: String = String::new();

    // prime first node
    match token_vec[0].token_type {
        Prefix => {
            open_tag = std::mem::take(&mut token_vec[0].value); // modify node
        }
        Suffix => {
            // throw an error
        }
        Literal => {
            open_text = std::mem::take(&mut token_vec[0].value); // modify node
        }
    }

    let mut i = 1;
    while i < token_vec.len() {
        match token_vec[i].token_type {
            Prefix => {
                // Submit node, then modify upcoming node
                output.insert_leaf(&mut open_tag, &mut open_text);
                open_tag = std::mem::take(&mut token_vec[i].value);
            }
            Suffix => {
                // breakdown possible suffixes
                match token_vec[i].value.as_str() {
                    "empty_line" => match token_vec[i - 1].token_type {
                        Prefix => {}
                        Suffix => {}
                        Literal => {
                            output.insert_leaf(&mut open_tag, &mut open_text);
                        }
                    },
                    "h1" => {
                        // Modify node, then submit
                        open_tag = std::mem::take(&mut token_vec[i].value);
                        output.insert_leaf(&mut open_tag, &mut open_text);
                    }
                    "table" => {
                        // Change parent node
                    }
                    _ => {}
                }
            }
            Literal => {
                if open_tag == "" {
                    open_tag = "p".to_string();
                }
                match token_vec[i - 1].token_type {
                    Prefix => {
                        // Modify node, then submit
                        open_text = std::mem::take(&mut token_vec[i].value);
                        output.insert_leaf(&mut open_tag, &mut open_text);
                    }
                    Suffix => {
                        // Submit, then modify node
                        open_text = std::mem::take(&mut token_vec[i].value);
                    }
                    Literal => {
                        // Submit, then modify node
                        output.insert_leaf(&mut open_tag, &mut open_text);
                        open_text = std::mem::take(&mut token_vec[i].value);
                    }
                }
            }
        }

        i = i + 1;
    }

    output
}
