use dprint_core::formatting::*;
use std::rc::Rc;

use super::context::Context;
use super::helper::*;
use crate::configuration::Configuration;
use crate::motoko_parser::{Node, NodeType};

#[cfg(debug_assertions)]
pub fn generate(nodes: &Vec<Node>, text: &str, config: &Configuration) -> PrintItems {
    let mut context = Context::new(text, config);
    let mut items = PrintItems::new();

    items.extend(gen_nodes(nodes, &mut context));
    println!("{}", "#".repeat(40));
    println!("{}", items.get_as_text());
    println!("{}", "#".repeat(40));
    items
}

#[cfg(not(debug_assertions))]
pub fn generate(nodes: &Vec<Node>, text: &str, config: &Configuration) -> PrintItems {
    let mut context = Context::new(text, config);
    let mut items = PrintItems::new();

    items.extend(gen_nodes(nodes, &mut context));

    items
}

fn gen_node<'a>(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();

    items.extend(match &node.node_type {
        NodeType::Motoko => gen_nodes(&node.children, context),
        NodeType::Header => gen_nodes(&node.children, context),
        NodeType::Program => gen_nodes(&node.children, context),
        NodeType::CompleteImport => gen_nodes(&node.children, context),
        NodeType::CompleteDeclaration => gen_declaration(&node, context),
        NodeType::EndOfImport => gen_nodes(&node.children, context),
        NodeType::EndOfDeclaration => gen_nodes(&node.children, context),
        NodeType::Import => gen_import(&node, context),
        NodeType::Id => gen_id(&node, context),
        NodeType::EqualSign => gen_id(&node, context),
        NodeType::WHITESPACE => gen_ignore(&node, context),
        NodeType::PatternNullary => gen_pattern_nullary(&node, context),
        NodeType::PatternPlain => gen_nodes(&node.children, context),
        NodeType::Pattern => gen_id_trim(&node, context), // TODO
        NodeType::Text => gen_id(&node, context),
        NodeType::Semicolon => gen_ignore(&node, context),
        NodeType::EOI => gen_ignore(&node, context),
        NodeType::COMMENT => gen_nodes(&node.children, context),
        NodeType::Comment => gen_comment(&node, context),
        NodeType::LineComment => gen_comment_line("//", &node, context),
        NodeType::DocComment => gen_comment_line("///", &node, context),
        NodeType::BlockComment => gen_comment_block(&node, context),
        NodeType::LineCommentContent => gen_id_trim(&node, context),
        NodeType::DocCommentContent => gen_id_trim(&node, context),
        NodeType::BlockCommentContent => gen_id_multiline(&node, context),
        NodeType::SpacedComment => gen_spaced_comment(&node, context),
        NodeType::Declaration => gen_nodes(&node.children, context),
        NodeType::Lit => gen_id(&node, context),
        NodeType::ShouldNewline => gen_should_newline(&node, context),
        NodeType::PatternField => gen_pattern_field(&node, context),
        NodeType::DeclarationNonVar => gen_id_trim(&node, context), // TODO
        NodeType::ExpNonDec => gen_id_trim(&node, context),         // TODO

        // TODO: remove to handle all cases
        _ => gen_id(&node, context),
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
    items.push_signal(Signal::StartIndent);
    items.push_signal(Signal::StartIndent);

    for n in node.children.iter() {
        match n.node_type {
            NodeType::KeywordImport => {}
            NodeType::PatternNullary => {
                items.extend(gen_pattern_nullary(n, context));
                items.push_signal(Signal::SpaceOrNewLine);
                context.expect_space = true;
                println!("þ set expect_space");
            }
            NodeType::EqualSign => {
                items.extend(gen_node(n, context));
                items.push_signal(Signal::SpaceIfNotTrailing);
                context.expect_space = true;
                println!("þ set expect_space");
            }
            _ => items.extend(gen_node(n, context)),
        }
    }

    items.push_str(";");
    context.expect_space = false;
    println!("þ reset expect_space");
    items.push_signal(Signal::FinishIndent);
    items.push_signal(Signal::FinishIndent);
    items.push_signal(Signal::FinishNewLineGroup);

    items
}

// Use the original node content as text
fn gen_id(node: &Node, _context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    // TODO? check & handle line breaks
    let mut first = true;
    for l in node.original.split("\n") {
        if !first {
            items.push_signal(Signal::NewLine);
        }
        first = false;
        items.push_str(l);
    }
    items
}

fn gen_id_trim(node: &Node, _context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    let mut first = true;
    for l in node.original.trim().split("\n") {
        if !first {
            items.push_signal(Signal::NewLine);
        }
        first = false;
        items.push_str(l);
    }
    items
}

fn gen_id_multiline(node: &Node, _context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    let lines = node.original.clone();
    // optional '\r' at line end is removed by trim_end
    let len = lines.split("\n").count();
    if len == 1 {
        items.push_str(lines.trim())
    } else {
        for (i, l) in lines.split("\n").enumerate() {
            if l.trim() == "" {
                // ignore
            } else if i == 0 && l.starts_with(" ") {
                items.push_str(l.trim_end())
            } else {
                items.push_str(l.trim_end());
            }
            if i < (len - 1) {
                items.push_signal(Signal::NewLine);
            }
        }
    }
    items
}

fn gen_ignore(_node: &Node, _context: &mut Context) -> PrintItems {
    PrintItems::new()
}

fn gen_comment_line(pre: &str, node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    let spaces = if context.expect_space {
        gen_spaces(1)
    } else {
        gen_spaces(2)
    };
    items.push_condition(conditions::if_true(
        "endLineText",
        Rc::new(|context| Some(context.writer_info.column_number > 0)),
        spaces,
    ));
    items.push_str(pre);
    items.push_signal(Signal::SpaceIfNotTrailing);
    // TODO: wrap / reflow text?
    items.extend(gen_nodes(&node.children, context));
    items.push_signal(Signal::ExpectNewLine);
    items
}

fn gen_comment(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();

    for n in node.children.iter() {
        match n.node_type {
            _ => items.extend(gen_node(n, context)),
        }
    }

    items
}

fn gen_comment_block(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    items.push_condition(conditions::if_true(
        "endLineText",
        Rc::new(|context| Some(context.writer_info.column_number > 0)),
        {
            // inline block comments only with one space
            let mut items = PrintItems::new();
            if !context.expect_space {
                items.push_signal(Signal::SpaceIfNotTrailing);
            }
            context.expect_space = false;
            items
        },
    ));
    items.push_signal(Signal::StartIgnoringIndent);

    items.push_str("/*");
    items.push_signal(Signal::SpaceIfNotTrailing);
    // TODO: wrap / reflow text?

    let mut add_linebreak = false;
    for n in node.children.iter() {
        match n.node_type {
            NodeType::WHITESPACE => {
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

fn gen_should_newline(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();

    let mut has_linebreak = false;
    for (_i, n) in node.children.iter().enumerate() {
        match n.node_type {
            NodeType::WHITESPACE => {
                let lines = count_lines(&n.original).clamp(0, 3);
                if lines > 0 {
                    items.extend(gen_newlines(lines));
                    has_linebreak = true;
                }
            }
            _ => {
                items.extend(gen_node(n, context));
            }
        }
    }
    if !has_linebreak {
        items.push_signal(Signal::NewLine);
    }

    items
}

fn gen_declaration(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    items.push_signal(Signal::StartNewLineGroup);

    let mut semicolon = 0;
    for (i, n) in node.children.iter().enumerate() {
        semicolon = i;
        match n.node_type {
            NodeType::EndOfDeclaration => break,
            _ => {
                items.extend(gen_node(n, context));
            }
        }
    }

    items.push_str(";");
    items.push_signal(Signal::FinishNewLineGroup);

    for n in node.children.iter().skip(semicolon) {
        match n.node_type {
            _ => items.extend(gen_node(n, context)),
        }
    }

    items
}

fn gen_pattern_nullary(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();

    if has_child(&node, NodeType::PatternField) {
        items.extend(gen_list("{", ";", "}", &node.children, context));
    } else {
        items.extend(gen_nodes(&node.children, context))
    }

    items
}

fn gen_list(
    start: &str,
    sep: &str,
    end: &str,
    nodes: &Vec<Node>,
    context: &mut Context,
) -> PrintItems {
    let mut items = PrintItems::new();

    items.push_signal(Signal::StartNewLineGroup);
    items.push_str(start);
    items.push_signal(Signal::SpaceOrNewLine);

    let mut first = true;
    for n in nodes {
        if is_whitespace_or_comment(n) || is_ignored(n) {
            items.extend(gen_node(n, context));
        } else {
            if !first {
                items.push_str(sep);
                items.push_signal(Signal::SpaceOrNewLine);
            }
            first = false;
            items.extend(gen_node(n, context));
        }
    }
    items.push_signal(Signal::SpaceIfNotTrailing);
    items.push_str(end);
    items.push_signal(Signal::FinishNewLineGroup);

    items
}

fn gen_pattern_field(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    for n in node.children.iter() {
        match n.node_type {
            NodeType::Type => {
                items.push_str(":");
                items.push_signal(Signal::SpaceIfNotTrailing);
                items.extend(gen_node(n, context));
            }
            NodeType::EqualSign => {}
            NodeType::Pattern => {
                items.push_signal(Signal::SpaceIfNotTrailing);
                items.push_str("=");
                items.push_signal(Signal::SpaceIfNotTrailing);
                context.expect_space = true;
                println!("þ set expect_space");
                items.extend(gen_node(n, context));
                println!("þ reset expect_space");
                context.expect_space = false;
            }

            _ => {
                items.extend(gen_node(n, context));
            }
        }
    }

    items
}

fn gen_spaced_comment(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();

    for (_i, n) in node.children.iter().enumerate() {
        match n.node_type {
            NodeType::WHITESPACE => {
                let lines = count_lines(&n.original).clamp(0, 3);
                if lines > 0 {
                    items.extend(gen_newlines(lines));
                }
            }
            _ => {
                items.extend(gen_node(n, context));
            }
        }
    }

    items
}

/* template

fn gen_(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();

    for (i, n) in node.children.iter().enumerate() {
        match n.node_type {
            _ => {
                items.extend(gen_node(n, context));
            }
        }
    }

    items
}

*/
