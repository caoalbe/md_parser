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
    is_leaf: bool,
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

    pub fn get_literal(&self) -> Option<String> {
        match &self.value {
            Inline(text) => Some(text.clone()),
            _ => None,
        }
    }
}

impl Tree {
    // Builds a tree with a single root node, whose tag is "html"
    pub fn build() -> Tree {
        let root_node: Node = Node {
            parent: None,
            tag: "html".to_string(),
            value: Children(vec![]),
            is_leaf: false,
        };

        let ptr: Rc<RefCell<Node>> = Rc::new(RefCell::new(root_node));
        Tree {
            root: Rc::clone(&ptr),
            curr: Rc::clone(&ptr),
        }
    }

    // Inserts a leaf, as a child of curr node
    pub fn insert_leaf(&self, literal: &mut String) -> () {
        let to_add: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {
            parent: Some(Rc::clone(&self.curr)),
            tag: "".to_string(),
            value: Inline(std::mem::take(literal)),
            is_leaf: true,
        }));
        if let Children(lst) = &mut self.curr.borrow_mut().value {
            lst.push(to_add);
        }
    }

    // Inserts a branch, as a child of curr node
    pub fn insert_branch(&mut self, tag: &mut String) -> () {
        let to_add: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {
            parent: Some(Rc::clone(&self.curr)),
            tag: std::mem::take(tag),
            value: Children(vec![]),
            is_leaf: false,
        }));
        if let Children(lst) = &mut self.curr.borrow_mut().value {
            lst.push(Rc::clone(&to_add));
        }

        self.curr = Rc::clone(&to_add);
    }

    pub fn insert_node(&mut self, node: Rc<RefCell<Node>>) -> () {
        node.borrow_mut().parent = Some(Rc::clone(&self.curr));

        if let Children(lst) = &mut self.curr.borrow_mut().value {
            lst.push(Rc::clone(&node));
        }

        if !self.curr.borrow().is_leaf {
            self.curr = Rc::clone(&node);
        }
    }

    // Moves curr pointer up to its parent
    fn curr_up(&mut self) -> () {
        let maybe_parent = self.curr.borrow().parent.clone();
        if let Some(parent) = maybe_parent {
            self.curr = parent
        }
    }

    // removes curr pointer; assuming that its the last one
    fn remove_curr(&mut self) -> Option<Rc<RefCell<Node>>> {
        let mut borrowed = self.curr.borrow_mut();
        match &mut borrowed.value {
            Children(vec_node) => vec_node.pop(),
            Inline(_) => None,
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
            Children(vec_node) => {
                if vec_node.len() == 1 && vec_node[0].borrow().is_leaf {
                    // Single, leaf child.  Print all in one line
                    if let Inline(child_text) = &vec_node[0].borrow().value {
                        builder.push_str(&format!(
                            "<{}>{}</{}>\n",
                            target.tag, child_text, target.tag
                        ));
                    }
                } else {
                    // Multiple children
                    builder.push_str(&format!("<{}>\n", target.tag));
                    for node in vec_node {
                        self.display_helper(builder, &node.borrow(), depth + 1, tab_size);
                    }
                    builder.push_str(&" ".repeat(depth * tab_size));
                    builder.push_str(&format!("</{}>\n", target.tag));
                }
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

pub fn run_ast(token_vec: Vec<Token>) -> Tree {
    let mut output: Tree = Tree::build();
    if token_vec.is_empty() {
        return output;
    }

    let mut open_tag: String;
    let mut open_text: String;
    for mut token in token_vec {
        match token.token_type {
            Prefix => {
                // Create branch node with given tag
                open_tag = std::mem::take(&mut token.value);
                output.insert_branch(&mut open_tag);
            }
            Suffix => {
                match token.value.as_str() {
                    "empty_line" => {
                        if output.get_curr_tag() == "table" {
                            output.curr_up();
                        }
                    }
                    "h1" | "h2" | "h3" |"h4" | "h5" |"h6" => {
                        // Modify node, then submit
                        let prev: Rc<RefCell<Node>> = output.remove_curr().expect("");
                        open_tag = std::mem::take(&mut token.value);
                        output.insert_branch(&mut open_tag);
                        output.insert_node(prev);
                        output.curr_up();
                        output.curr_up();
                    }
                    "table" => {
                        // Change parent node
                        let prev: Rc<RefCell<Node>> = output.remove_curr().expect("");
                        let token_headers: Option<String> = prev.borrow().get_literal();

                        open_tag = std::mem::take(&mut token.value);
                        output.insert_branch(&mut open_tag);
                        output.insert_branch(&mut "tr".to_string());

                        if let Some(text) = token_headers {
                            for col in text.split('|').filter(|s| !s.is_empty()) {
                                output.insert_branch(&mut "th".to_string());
                                output.insert_leaf(&mut col.to_string());
                                output.curr_up();
                            }
                        }
                        output.curr_up();
                    }
                    _ => {}
                }
            }
            Literal => {
                if output.get_curr_tag() == "table" {
                    open_text = std::mem::take(&mut token.value);
                    output.insert_branch(&mut "tr".to_string());
                    for col in open_text.split('|').filter(|s| !s.is_empty()) {
                        output.insert_branch(&mut "td".to_string());
                        output.insert_leaf(&mut col.to_string());
                        output.curr_up();
                    }
                    output.curr_up();
                } else {
                    // Create leaf node with given text
                    open_text = std::mem::take(&mut token.value);
                    output.insert_leaf(&mut open_text);
                    output.curr_up();
                }
            }
        }
    }

    output
}
