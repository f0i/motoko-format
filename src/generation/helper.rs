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

pub fn count_not_ignored(nodes: &Vec<Node>) -> usize {
    let mut count = 0;
    for node in nodes {
        if !is_ignored(node) {
            count += 1;
        }
    }
    count
}

pub fn gen_spaces(n: usize) -> PrintItems {
    let mut items = PrintItems::new();
    for _ in 0..n {
        items.push_signal(Signal::SpaceIfNotTrailing);
    }
    items
}

pub fn if_not_start_of_line(then: PrintItems) -> PrintItems {
    let mut items = PrintItems::new();
    items.push_condition(conditions::if_false(
        "endLineText",
        Rc::new(|context| Some(context.writer_info.is_start_of_line())),
        then,
    ));
    items
}

/// Group of optional linebreaks that break all or none
pub struct MultiLineGroup {
    pub resolver: ConditionResolver,
    pub start_ln: LineNumber,
    pub end_ln: LineNumber,
    started: bool,
    ended: bool,
}

impl MultiLineGroup {
    pub fn new(force_multiline: bool) -> Self {
        let start_ln = LineNumber::new("start");
        let end_ln = LineNumber::new("end");
        let resolver = Rc::new(move |condition_context: &mut ConditionResolverContext| {
            if force_multiline {
                return Some(true);
            }
            // check if it spans multiple lines, and if it does then make it multi-line
            condition_helpers::is_multiple_lines(condition_context, start_ln, end_ln)
        });

        Self {
            resolver,
            start_ln,
            end_ln,
            started: false,
            ended: false,
        }
    }

    pub fn space_or_newline(&self) -> PrintItems {
        let mut items = PrintItems::new();
        let newline = Signal::NewLine.into();
        let space = Signal::SpaceOrNewLine.into();
        items.push_condition(conditions::if_true_or(
            "multi_line_group",
            self.resolver.clone(),
            newline,
            space,
        ));
        items
    }
}
