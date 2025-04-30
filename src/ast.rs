use std::cell::RefCell;
use std::rc::Rc;

use crate::lexer::Token;
use crate::lexer::TokenType::*;
use Content::*;

enum TreeState {
    Start,
    Prefix,
    Literal,
}

pub enum Content {
    Children(Vec<Rc<RefCell<Node>>>),
    Inline(String),
}

pub struct Node {
    parent: Option<Rc<RefCell<Node>>>,
    tag: String,
    value: Content,
    is_leaf: bool,
}

pub struct Tree {
    root: Rc<RefCell<Node>>,
    curr: Rc<RefCell<Node>>,
}

impl Node {
    pub fn build_branch() -> Node {
        Node {
            parent: None,
            tag: "".to_string(),
            value: Children(vec![]),
            is_leaf: false,
        }
    }

    pub fn build_leaf() -> Node {
        Node {
            parent: None,
            tag: "".to_string(),
            value: Inline("".to_string()),
            is_leaf: true,
        }
    }

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

    pub fn set_value(&mut self, new_content: Content) -> () {
        match new_content {
            Children(_) => {
                self.is_leaf = false;
            }
            Inline(_) => {
                self.is_leaf = true;
            }
        }
        self.value = new_content;
    }

    pub fn append_literal(&mut self, to_add: &mut String) -> () {
        if let Inline(curr) = &self.value {
            let mut temp: String = curr.clone();
            temp.push_str(&to_add);
            self.set_value(Inline(temp));
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
    pub fn insert_leaf(&mut self, tag: &mut String, literal: &mut String) -> () {
        let to_add: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {
            parent: Some(Rc::clone(&self.curr)),
            tag: std::mem::take(tag),
            value: Inline(std::mem::take(literal)),
            is_leaf: true,
        }));
        if let Children(lst) = &mut self.curr.borrow_mut().value {
            lst.push(Rc::clone(&to_add));
        }

        self.curr = Rc::clone(&to_add);
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

    // Inserts a node, as a child of curr node
    pub fn insert_node(&mut self, node: Rc<RefCell<Node>>) -> () {
        node.borrow_mut().parent = Some(Rc::clone(&self.curr));

        if let Children(lst) = &mut self.curr.borrow_mut().value {
            lst.push(Rc::clone(&node));
        }

        self.curr = Rc::clone(&node);
    }

    // Moves curr pointer up to its parent
    fn curr_up(&mut self) -> () {
        let maybe_parent = self.curr.borrow().parent.clone();
        if let Some(parent) = maybe_parent {
            self.curr = parent
        }
    }

    // Removes last child of curr; only if curr has children
    fn remove_curr_youngest(&mut self) -> Option<Rc<RefCell<Node>>> {
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

    pub fn append_curr_literal(&mut self, to_add: &mut String) -> () {
        self.curr.borrow_mut().append_literal(to_add);
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
                // Multiple children
                builder.push_str(&format!("<{}>\n", target.tag));
                for node in vec_node {
                    self.display_helper(builder, &node.borrow(), depth + 1, tab_size);
                }
                builder.push_str(&" ".repeat(depth * tab_size));
                builder.push_str(&format!("</{}>\n", target.tag));
            }
            Inline(text) => {
                builder.push_str(&format!(
                    "<{}>{}</{}>\n",
                    target.get_tag(),
                    text,
                    target.get_tag()
                ));
            }
        }
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: String = String::new();
        self.display_helper(&mut output, &self.root.borrow(), 0, 4);
        write!(f, "{}", output.trim())
    }
}

pub fn run_ast(token_vec: Vec<Token>) -> Tree {
    let mut output: Tree = Tree::build();
    if token_vec.is_empty() {
        return output;
    }

    let mut tree_state: TreeState = TreeState::Start;
    let mut open_tag: String;
    let mut open_text: String;

    for mut token in token_vec {
        match token.token_type {
            Prefix => {
                // Create branch node with given tag
                open_tag = std::mem::take(&mut token.value);
                output.insert_leaf(&mut open_tag, &mut "".to_string());
                output.curr_up();

                tree_state = TreeState::Prefix;
            }
            Suffix => {
                // Assumes <curr> points to the node to edit
                match token.value.as_str() {
                    "empty_line" => match output.get_curr_tag().as_str() {
                        "table" => {
                            output.curr_up();
                        }
                        _ => {}
                    },
                    "code_block" => {
                        if output.get_curr_tag() == "code" {
                            // exit block
                            output.curr_up();
                            output.curr_up();
                        } else {
                            // start block
                            output.insert_branch(&mut "pre".to_string());
                            output.insert_leaf(&mut "code".to_string(), &mut "".to_string());
                        }
                    }
                    "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                        open_tag = std::mem::take(&mut token.value);
                        let target: Rc<RefCell<Node>> = output.remove_curr_youngest().expect("");
                        target.borrow_mut().set_tag(&mut open_tag);
                        output.insert_node(target);
                        output.curr_up();
                    }
                    "table" => {
                        // Change parent node
                        output.curr_up();
                        let prev: Rc<RefCell<Node>> = output.remove_curr_youngest().expect("");
                        let token_headers: Option<String> = prev.borrow().get_literal();

                        open_tag = std::mem::take(&mut token.value);
                        output.insert_branch(&mut open_tag);
                        output.insert_branch(&mut "tr".to_string());

                        if let Some(text) = token_headers {
                            for col in text.split('|').filter(|s| !s.is_empty()) {
                                output.insert_leaf(&mut "th".to_string(), &mut col.to_string());
                                output.curr_up();
                            }
                        }
                        output.curr_up();
                    }
                    _ => {}
                }

                tree_state = TreeState::Start;
            }
            Literal => {
                match output.get_curr_tag().as_str() {
                    "table" => {
                        open_text = std::mem::take(&mut token.value);
                        output.insert_branch(&mut "tr".to_string());
                        for col in open_text.split('|').filter(|s| !s.is_empty()) {
                            output.insert_leaf(&mut "td".to_string(), &mut col.to_string());
                            output.curr_up();
                        }
                        output.curr_up();
                    }
                    "code" => {
                        open_text = std::mem::take(&mut token.value);
                        output.append_curr_literal(&mut open_text);
                    }
                    _ => match tree_state {
                        TreeState::Start => {
                            open_text = std::mem::take(&mut token.value);
                            output.insert_leaf(&mut "p".to_string(), &mut open_text);
                            output.curr_up();
                        }
                        TreeState::Prefix => {
                            open_text = std::mem::take(&mut token.value);
                            let target: Rc<RefCell<Node>> =
                                output.remove_curr_youngest().expect("");
                            target.borrow_mut().set_value(Inline(open_text));
                            output.insert_node(target);
                            output.curr_up();
                        }
                        TreeState::Literal => {
                            open_text = std::mem::take(&mut token.value);
                            output.insert_leaf(&mut "p".to_string(), &mut open_text);
                            output.curr_up();
                        }
                    },
                }

                tree_state = TreeState::Literal;
            }
        }
    }

    output
}
