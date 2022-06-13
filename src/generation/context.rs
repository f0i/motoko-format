use std::collections::HashSet;

use crate::motoko_parser::Node;

use crate::configuration::Configuration;

pub struct Context<'a> {
    pub config: &'a Configuration,
    pub text: &'a str,
    pub handled_comments: HashSet<usize>,
    current_node: Option<Node>,
    parent_stack: Vec<Node>,
    pub gen_string_content: bool,
}

impl<'a> Context<'a> {
    pub fn new(text: &'a str, config: &'a Configuration) -> Self {
        Self {
            config,
            text,
            handled_comments: HashSet::new(),
            current_node: None,
            parent_stack: Vec::new(),
            gen_string_content: false,
        }
    }

    pub fn set_current_node(&mut self, node: Node) {
        if let Some(parent) = self.current_node.take() {
            self.parent_stack.push(parent);
        }
        self.current_node = Some(node);
    }

    pub fn pop_current_node(&mut self) {
        self.current_node = self.parent_stack.pop();
    }

    pub fn parent(&self) -> Option<&Node> {
        self.parent_stack.last()
    }
}
