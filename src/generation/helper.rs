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
    resolver: ConditionResolver,
    start_ln: LineNumber,
    end_ln: LineNumber,
    indent: u32,
    items: PrintItems,
}

impl MultiLineGroup {
    pub fn new(force_multi_line: bool, indent: u32) -> Self {
        let start_ln = LineNumber::new("multi_line_start");
        let end_ln = LineNumber::new("multi_line_end");
        let _ = LineNumber::new("unused");
        let resolver = Rc::new(move |condition_context: &mut ConditionResolverContext| {
            if force_multi_line {
                return Some(true);
            }
            // check if it spans multiple lines, and if it does then make it multi-line
            condition_helpers::is_multiple_lines(condition_context, start_ln, end_ln)
        });

        let mut items = PrintItems::new();
        items.push_info(start_ln);
        items.push_anchor(LineNumberAnchor::new(start_ln));
        items.push_signal(Signal::StartNewLineGroup);

        Self {
            resolver,
            start_ln,
            end_ln,
            indent,
            items,
        }
    }

    pub fn extend(&mut self, items: PrintItems) {
        self.items.extend(items);
    }

    pub fn push_str(&mut self, s: &str) {
        self.items.push_str(s);
    }

    pub fn push_signal(&mut self, s: Signal) {
        self.items.push_signal(s);
    }

    pub fn take(mut self) -> PrintItems {
        self.items.push_info(self.end_ln);
        self.items.push_signal(Signal::FinishNewLineGroup);

        ir_helpers::with_indent_times(self.items, self.indent)
    }

    pub fn space_or_newline(&mut self) {
        let newline = Signal::NewLine.into();
        let space = Signal::SpaceOrNewLine.into();
        self.if_multiline_or(newline, space);
    }

    pub fn possible_newline(&mut self) {
        let newline = Signal::NewLine.into();
        let possible = Signal::PossibleNewLine.into();
        self.if_multiline_or(newline, possible);
    }

    pub fn if_multiline_or(&mut self, multi: PrintItems, single: PrintItems) {
        let cond =
            conditions::if_true_or("multi_line_group", self.resolver.clone(), multi, single).into();
        self.items.extend(cond);
    }

    pub fn if_multiline(&mut self, multi: PrintItems) {
        self.if_multiline_or(multi, PrintItems::new());
    }
}
