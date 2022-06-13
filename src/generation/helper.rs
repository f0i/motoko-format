use crate::motoko_parser::Node;
use dprint_core::formatting::*;

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
    v.iter().count() == n + 1
}
