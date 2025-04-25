use std::cell::RefCell;
use std::rc::Rc;

use crate::lexer::Token;
use crate::lexer::TokenType::*;
use Content::*;

pub enum Content {
    Children(Vec<Rc<RefCell<Node>>>),
    Inline(String),
}

pub struct Node {
    parent: Option<Rc<RefCell<Node>>>,
    pub tag: String,
    pub value: Content,
}

pub struct Tree {
    pub root: Rc<RefCell<Node>>,
    curr: Rc<RefCell<Node>>,
}

impl Node {
    pub fn get_tag(&self) -> &str {
        return self.tag.as_str();
    }
    pub fn set_tag(&mut self, new_tag: &mut String) -> () {
        self.tag = std::mem::take(new_tag);
    }
}

impl Tree {
    // Builds a tree with a single root node, whose tag is "html"
    pub fn build() -> Tree {
        let root_node: Node = Node {
            parent: None,
            tag: "html".to_string(),
            value: Children(vec![]),
        };

        let ptr: Rc<RefCell<Node>> = Rc::new(RefCell::new(root_node));
        Tree {
            root: Rc::clone(&ptr),
            curr: Rc::clone(&ptr),
        }
    }

    // Inserts a leaf, as a child of curr node
    pub fn insert_leaf(&self, literal: &mut String) -> () {
        if literal == "" {
            return;
        }
        let to_add: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {
            parent: Some(Rc::clone(&self.curr)),
            tag: "".to_string(),
            value: Inline(std::mem::take(literal)),
        }));
        if let Children(lst) = &mut self.curr.borrow_mut().value {
            lst.push(to_add);
        }
    }

    // Inserts a branch, as a child of curr node
    pub fn insert_branch(&mut self, tag: &mut String) -> () {
        if tag == "" {
            return;
        }
        let to_add: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {
            parent: Some(Rc::clone(&self.curr)),
            tag: std::mem::take(tag),
            value: Children(vec![]),
        }));
        if let Children(lst) = &mut self.curr.borrow_mut().value {
            lst.push(Rc::clone(&to_add));
        }

        self.curr = Rc::clone(&to_add);
    }

    // Moves curr pointer up to its parent
    fn curr_up(&mut self) -> () {
        let maybe_parent = self.curr.borrow().parent.clone();
        if let Some(parent) = maybe_parent {
            self.curr = parent
        }
    }

    pub fn get_curr_tag(&self) -> String {
        self.curr.borrow().get_tag().to_string()
    }

    pub fn set_curr_tag(&mut self, new_tag: &mut String) -> () {
        self.curr.borrow_mut().set_tag(new_tag);
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
            // TODO: If vec_node has literal as only child; then print succinctly in one line,
            // i.e. <h1>header1</h1>
            Children(vec_node) => {
                builder.push_str(&format!("<{}>\n", target.tag));
                for node in vec_node {
                    self.display_helper(builder, &node.borrow(), depth + 1, tab_size);
                }
                builder.push_str(&" ".repeat(depth * tab_size));
                builder.push_str(&format!("</{}>\n", target.tag));
            }
            Inline(text) => {
                builder.push_str(&format!("{}\n", text));
            }
        }
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: String = String::new();
        self.display_helper(&mut output, &self.root.borrow(), 0, 4);
        write!(f, "{}", output)
    }
}

pub fn run_ast(mut token_vec: Vec<Token>) -> Tree {
    let mut output: Tree = Tree::build();

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

    // i think we can do everything just in time;
    // since the ast is split between tags and literals
    let mut i = 1;
    while i < token_vec.len() {
        match token_vec[i].token_type {
            Prefix => {
                // Submit node, then modify upcoming node
                output.insert_branch(&mut open_tag);
                output.insert_leaf(&mut open_text);
                output.curr_up();
                open_tag = std::mem::take(&mut token_vec[i].value);
            }
            Suffix => {
                // breakdown possible suffixes
                match token_vec[i].value.as_str() {
                    "empty_line" => {
                        // if curr node points to table, then exit it
                        if output.get_curr_tag() == "table" {
                            output.curr_up();
                        } else {
                            match token_vec[i - 1].token_type {
                                Prefix => {}
                                Suffix => {}
                                Literal => {
                                    // if open_text != "" {
                                    output.insert_leaf(&mut open_text);

                                    // }
                                }
                            }
                        }
                    }
                    "h1" => {
                        // Modify node, then submit
                        open_tag = std::mem::take(&mut token_vec[i].value);
                        output.insert_branch(&mut open_tag);
                        output.insert_leaf(&mut open_text);
                        output.curr_up();
                    }
                    "table" => {
                        // Change parent node
                        open_tag = std::mem::take(&mut token_vec[i].value);
                        output.insert_branch(&mut open_tag);
                        output.insert_branch(&mut "tr".to_string());

                        for col in open_text.split('|').filter(|s| !s.is_empty()) {
                            output.insert_branch(&mut "th".to_string());
                            output.insert_leaf(&mut col.to_string());
                            output.curr_up();
                        }

                        output.curr_up();
                        // output.adopt_sibling();
                    }
                    _ => {}
                }
            }
            Literal => {
                if output.get_curr_tag() == "table" {
                    open_text = std::mem::take(&mut token_vec[i].value);
                    output.insert_branch(&mut "tr".to_string());
                    for col in open_text.split('|').filter(|s| !s.is_empty()) {
                        output.insert_branch(&mut "td".to_string());
                        output.insert_leaf(&mut col.to_string());
                        output.curr_up();
                    }
                    output.curr_up();
                } else {
                    match token_vec[i - 1].token_type {
                        Prefix => {
                            // Modify node, then submit
                            open_text = std::mem::take(&mut token_vec[i].value);
                            output.insert_branch(&mut open_tag);
                            output.insert_leaf(&mut open_text);
                            output.curr_up();
                        }
                        Suffix => {
                            // Modify node
                            open_text = std::mem::take(&mut token_vec[i].value);
                        }
                        Literal => {
                            // Submit, then modify node
                            // output.insert_branch(&mut open_tag);
                            output.insert_leaf(&mut open_text);
                            // output.curr_up();
                            open_text = std::mem::take(&mut token_vec[i].value);
                        }
                    }
                }
            }
        }

        i = i + 1;
    }

    output
}
