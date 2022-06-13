use dprint_core::formatting::*;

use super::context::Context;
use super::helpers::*;
use crate::configuration::Configuration;
use crate::motoko_parser::{Node, NodeType};

pub fn generate(nodes: &Vec<Node>, text: &str, config: &Configuration) -> PrintItems {
    let mut context = Context::new(text, config);
    let mut items = PrintItems::new();

    items.extend(gen_nodes(nodes, &mut context));
    items
}

fn gen_node<'a>(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();

    context.set_current_node(node.clone());
    items.extend(match &node.node_type {
        NodeType::Program => gen_nodes(&node.children, context),
        NodeType::Import => gen_import(&node, context),
        NodeType::Id => gen_id(&node, context),
        NodeType::EqualSign => gen_id(&node, context),
        NodeType::Whitespace => gen_ignore(&node, context),
        NodeType::PatternNullary => gen_nodes(&node.children, context),
        NodeType::PatternPlain => gen_nodes(&node.children, context),
        NodeType::Text => gen_id(&node, context),
        NodeType::Semicolon => gen_ignore(&node, context),
        NodeType::Eoi => gen_ignore(&node, context),
        NodeType::Comment => gen_nodes(&node.children, context),
        NodeType::InlineComment => gen_comment_line("//", &node, context),
        NodeType::DocComment => gen_comment_line("///", &node, context),
        NodeType::BlockComment => gen_comment_block(&node, context),
        NodeType::LineCommentContent => gen_id_trim(&node, context),
        NodeType::DocCommentContent => gen_id_trim(&node, context),
        NodeType::BlockCommentContent => gen_id_multiline(&node, context),

        _ => {
            let mut i = PrintItems::new();
            println!("TODO: generate dprint IR from {:?}", node);
            for s in node.print("".into()).split("\n") {
                i.push_str(s);
                i.push_signal(Signal::ExpectNewLine);
            }
            i
        }
    });
    context.pop_current_node();
    println!("aaaaaaaaaaaaaaaaaaaaaa\n{}", items.get_as_text());
    items
}

fn gen_nodes<'a>(nodes: &Vec<Node>, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();

    for node in nodes {
        items.extend(gen_node(node, context));
    }
    items
}

fn gen_import(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    items.push_signal(Signal::StartNewLineGroup);
    items.push_str("import");
    items.push_signal(Signal::SpaceIfNotTrailing);

    for n in node.children.iter() {
        match n.node_type {
            NodeType::PatternNullary => {
                items.extend(gen_node(n, context));
                items.push_signal(Signal::SpaceOrNewLine);
            }
            NodeType::EqualSign => {
                items.extend(gen_node(n, context));
                items.push_signal(Signal::SpaceIfNotTrailing);
            }
            _ => items.extend(gen_node(n, context)),
        }
    }

    items.push_str(";");
    items.push_signal(Signal::FinishNewLineGroup);
    items.extend(Signal::NewLine.into());

    //if let Some(value) = &node.value {
    //  items.push_str("=");
    //  items.extend(gen_node(value.into(), context));
    //}

    items
}

// Use the original node content as text
fn gen_id(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    // TODO? check & handle line breaks
    items.push_string(node.original.clone());
    items
}

fn gen_id_trim(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    // TODO? check & handle line breaks
    items.push_str(node.original.trim());
    items
}

fn gen_id_multiline(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    let lines = node.original.clone();
    // optional '\r' at line end is removed by trim_end
    let len = lines.split("\n").count();
    for (i, l) in lines.split("\n").enumerate() {
        if i == 0 {
            items.push_str(l.trim())
        } else {
            items.push_str(l.trim_end());
        }
        if i < (len - 1) {
            items.push_signal(Signal::NewLine);
        }
    }
    items
}

fn gen_ignore(node: &Node, context: &mut Context) -> PrintItems {
    PrintItems::new()
}

fn gen_comment_line(pre: &str, node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    items.push_signal(Signal::StartForceNoNewLines);
    items.push_str(pre);
    items.push_signal(Signal::SpaceIfNotTrailing);
    // TODO: wrap / reflow text?
    items.extend(gen_nodes(&node.children, context));
    items.push_signal(Signal::FinishForceNoNewLines);
    items.push_signal(Signal::NewLine);
    items
}

fn gen_comment_block(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    items.push_signal(Signal::StartIgnoringIndent);
    items.push_str("/*");
    items.push_signal(Signal::SpaceIfNotTrailing);
    // TODO: wrap / reflow text?

    let mut add_linebreak = false;
    for n in node.children.iter() {
        match n.node_type {
            NodeType::CheckWhitespace => {
                add_linebreak = true;
            }
            _ => items.extend(gen_node(n, context)),
        }
    }
    items.push_signal(Signal::SpaceIfNotTrailing);
    items.push_str("*/");
    items.push_signal(Signal::FinishIgnoringIndent);
    items.push_signal(Signal::SpaceIfNotTrailing);
    if add_linebreak {
        items.push_signal(Signal::NewLine);
    }

    items
}
