use crate::motoko_parser::{Node, NodeType};
use dprint_core::formatting::*;
use std::rc::Rc;

pub fn count_lines(s: &String) -> usize {
    s.matches("\n").count()
}

pub fn gen_newlines(n: usize) -> PrintItems {
    let mut items = PrintItems::new();

    for _ in 0..n {
        items.push_signal(Signal::NewLine);
    }

    items
}

pub fn is_last(v: &Vec<Node>, n: usize) -> bool {
    v.len() == n + 1
}

pub fn has_child(node: &Node, t: NodeType) -> bool {
    for n in node.children.iter() {
        if n.node_type == t {
            return true;
        }
    }
    false
}

pub fn is_whitespace_or_comment(node: &Node) -> bool {
    match node.node_type {
        NodeType::WHITESPACE => true,
        NodeType::Comment => true,
        _ => false,
    }
}

pub fn is_ignored(node: &Node) -> bool {
    match node.node_type {
        NodeType::WHITESPACE => true,
        NodeType::Semicolon => true,
        NodeType::EOI => true,
        _ => false,
    }
}

pub fn gen_spaces(n: usize) -> PrintItems {
    let mut items = PrintItems::new();
    for _ in 0..n {
        items.push_signal(Signal::SpaceIfNotTrailing);
    }
    items
}
