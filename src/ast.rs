use std::cell::RefCell;
use std::rc::Rc;

use crate::lexer::Token;
// use crate::lexer::TokenType::*;

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
    pub fn insert_leaf(&self, tag: String, literal: String) -> () {
        let to_add: Rc<Node> = Rc::new(Node {
            parent: Some(Rc::clone(&self.curr)),
            tag: tag,
            value: Content::Literal(literal),
        });
        if let Content::Children(lst) = &self.curr.value {
            lst.borrow_mut().push(to_add);
        }
    }

    // Inserts a branch, as a child of curr node
    pub fn insert_branch(&mut self, tag: String) -> () {
        let to_add: Rc<Node> = Rc::new(Node {
            parent: Some(Rc::clone(&self.curr)),
            tag: tag,
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
}

pub fn run_ast(token_vec: Vec<Token>) -> Tree {
    let mut output: Tree = Tree::build();

    if token_vec.len() == 0 {
        return output;
    }

    // output.insert_leaf("h1".to_string(), "lorem".to_string());
    output.insert_branch("table".to_string());
    output.debug_curr_tag();
    output.curr_up();
    output.debug_curr_tag();

    // let mut open_tag: String = String::new();
    // let mut open_content: String = String::new();

    // prime first node
    // match token_vec[0].token_type {
    //     Prefix => {
    //         open_tag = token_vec[0].value.clone(); // modify node
    //     }
    //     Suffix => {
    //         // throw an error
    //     }
    //     Literal => {
    //         open_content = token_vec[0].value.clone(); // modify node
    //     }
    // }

    // // TODO: fix performance with .clone overusage
    // let mut i = 1;
    // while i < token_vec.len() {
    //     match token_vec[i].token_type {
    //         Prefix => {
    //             // Submit node, then modify upcoming node
    //             submit_node(&mut open_tag, &mut open_content);
    //             open_tag = token_vec[i].value.clone();
    //         }
    //         Suffix => {
    //             // breakdown possible suffixes
    //             match token_vec[i].value.as_str() {
    //                 "empty_line" => match token_vec[i - 1].token_type {
    //                     Prefix => {}
    //                     Suffix => {}
    //                     Literal => {
    //                         submit_node(&mut open_tag, &mut open_content);
    //                     }
    //                 },
    //                 "h1" => {
    //                     // Modify node, then submit
    //                     open_tag = token_vec[i].value.clone();
    //                     submit_node(&mut open_tag, &mut open_content);
    //                 }
    //                 "table" => {
    //                     // Change parent node
    //                 }
    //                 _ => {}
    //             }
    //         }
    //         Literal => {
    //             if open_tag == "" {
    //                 open_tag = "p".to_string();
    //             }
    //             match token_vec[i - 1].token_type {
    //                 Prefix => {
    //                     // Modify node, then submit
    //                     open_content = token_vec[i].value.clone();
    //                     submit_node(&mut open_tag, &mut open_content);
    //                 }
    //                 Suffix => {
    //                     // Submit, then modify node
    //                     open_content = token_vec[i].value.clone();
    //                 }
    //                 Literal => {
    //                     // Submit, then modify node
    //                     submit_node(&mut open_tag, &mut open_content);
    //                     open_content = token_vec[i].value.clone();
    //                 }
    //             }
    //         }
    //     }

    //     i = i + 1;
    // }

    output
}
