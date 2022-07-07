use dprint_core::formatting::*;

use super::context::Context;
use super::helper::*;
use crate::configuration::Configuration;
use crate::motoko_parser::{Node, NodeType::*};

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
        //Motoko => gen_debug(&node, context),
        Motoko => gen_nodes(&node.children, context),
        Header => gen_nodes(&node.children, context),
        Program => gen_nodes(&node.children, context),
        CompleteImport => gen_nodes(&node.children, context),
        CompleteDeclaration => gen_declaration(&node, context),
        EndOfImport => gen_nodes(&node.children, context),
        EndOfDeclaration => gen_nodes(&node.children, context),
        Import => gen_import(&node, context),
        Id => gen_id(&node, context),
        EqualSign => gen_id(&node, context),
        WHITESPACE => gen_ignore(&node, context),
        PatternNullary => gen_pattern_nullary(&node, context),
        PatternPlain => gen_nodes_maybe_perenthesized(&node, context),
        Pattern => gen_id_trim(&node, context), // TODO
        Text => gen_id(&node, context),
        Semicolon => gen_ignore(&node, context),
        EOI => gen_ignore(&node, context),
        COMMENT => gen_nodes(&node.children, context),
        Comment => gen_comment(&node, context),
        LineComment => gen_comment_line("//", &node, context),
        DocComment => gen_comment_line("///", &node, context),
        BlockComment => gen_comment_block(&node, context),
        LineCommentContent => gen_id_trim(&node, context),
        DocCommentContent => gen_id_trim(&node, context),
        BlockCommentContent => gen_id_multiline(&node, context),
        SpacedComment => gen_spaced_comment(&node, context),
        Declaration => gen_nodes(&node.children, context),
        Lit => gen_id(&node, context),
        ShouldNewline => gen_should_newline(&node, context),
        PatternField => gen_pattern_field(&node, context),
        ExpNonDec => gen_exp_non_dec(&node, context),

        DeclarationNonVar | Exp | ExpNonVar | ExpPlain | ExpBin | ExpUn | ExpPostContinue
        | ExpNullary | DeclarationField | PatternUn | Type | TypeNoBin | TypeUn | TypeNullary
        | TypePre | ExpBinContinue => gen_nodes(&node.children, context),

        ObjSort | Visibility | KeywordFunc | KeywordAsync | KeywordReturn | Colon | BinOp => {
            gen_keyword(node, context)
        }

        ObjBody | Block => gen_surounded("{", "}", &node.children, context),

        FuncBody => gen_func_body(node, context),

        ExpPost | PatternBin => gen_nodes_maybe_perenthesized(&node, context),
        ExpList => gen_list("(", ",", ")", &node.children, context),
        Dot => gen_id_no_space(&node, context),

        // TODO: remove to handle all cases
        _ => gen_id(&node, context),
        //_ => gen_debug(node, context),
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
    context.expect_space();
    items.push_signal(Signal::StartIndent);
    items.push_signal(Signal::StartIndent);

    for n in node.children.iter() {
        match n.node_type {
            KeywordImport => { /* already added */ }
            PatternNullary => {
                items.extend(gen_pattern_nullary(n, context));
                context.expect_space();
            }
            EqualSign => {
                items.extend(gen_node(n, context));
                context.expect_space();
            }
            COMMENT => {
                items.extend(gen_node(n, context));
            }
            _ => items.extend(gen_node(n, context)),
        }
    }

    items.push_str(";");
    items.push_signal(Signal::FinishIndent);
    items.push_signal(Signal::FinishIndent);
    items.push_signal(Signal::FinishNewLineGroup);

    items
}

// Use the original node content as text
fn gen_id(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    items.extend(context.gen_expected_space());
    let mut first = true;
    for l in node.original.split("\n") {
        if !first {
            items.push_signal(Signal::NewLine);
        }
        first = false;
        items.push_str(l);
    }
    context.expect_space();
    items
}

fn gen_id_no_space(node: &Node, context: &mut Context) -> PrintItems {
    context.reset_expect();
    let items = gen_id(node, context);
    context.reset_expect();
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

fn gen_keyword(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    context.expect_space();
    items.extend(context.gen_expected_space());

    items.extend(gen_id(node, context));
    context.expect_space();
    items
}

fn gen_comment_line(pre: &str, node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    let spaces = gen_spaces(2);
    context.expect_space();
    items.extend(if_not_start_of_line(spaces));

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
    //items.extend(gen_expected_space(context));

    // inline block comments only with one space
    items.extend(if_not_start_of_line(gen_spaces(1)));
    items.push_signal(Signal::StartIgnoringIndent);

    items.push_str("/*");
    items.push_signal(Signal::SpaceIfNotTrailing);
    // TODO: wrap / reflow text?

    let mut add_linebreak = false;
    for n in node.children.iter() {
        match n.node_type {
            WHITESPACE => {
                add_linebreak = true;
            }
            _ => items.extend(gen_node(n, context)),
        }
    }
    items.push_signal(Signal::SpaceIfNotTrailing);
    items.push_str("*/");
    context.expect_space();
    items.push_signal(Signal::FinishIgnoringIndent);
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
            WHITESPACE => {
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
            EndOfDeclaration => break,
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

    items.extend(context.gen_expected_space());

    if node.get_one_child(&PatternField).is_some() {
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

    context.reset_expect();
    let count = count_not_ignored(nodes);

    items.push_str(start);

    let mut multiline = MultiLineGroup::new(false, 1);
    multiline.possible_newline();

    if count > 1 {
        context.expect_space_or_newline();
    }

    multiline.extend(gen_list_body(sep, nodes, context));

    multiline.if_multiline(sep.to_string().into());

    if count > 1 {
        multiline.push_signal(Signal::SpaceIfNotTrailing);
    }

    multiline.possible_newline();
    items.extend(multiline.take());
    items.push_str(end);
    context.expect_space_or_newline();

    items
}

/// generate list content
/// return number of printed elements and items
fn gen_list_body(sep: &str, nodes: &Vec<Node>, context: &mut Context) -> PrintItems {
    let mut items = MultiLineGroup::new(false, 0);

    let mut first = true;

    for n in nodes {
        if is_ignored(n) {
        } else if is_whitespace_or_comment(n) {
            items.extend(gen_node(n, context));
        } else {
            if !first {
                items.push_str(sep);
                items.possible_newline();
                context.expect_space();
            }
            items.extend(gen_node(n, context));
            first = false;
        }
    }

    items.take()
}

fn gen_surounded(start: &str, end: &str, nodes: &Vec<Node>, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    let mut multiline = MultiLineGroup::new(false, 1);

    items.extend(context.gen_expected_space());
    items.push_str(start);

    multiline.possible_newline();
    context.expect_space_or_newline();

    let mut any = false;
    for n in nodes {
        if is_ignored(n) {
        } else if is_whitespace_or_comment(n) {
            multiline.extend(gen_node(n, context));
            any = true;
        } else {
            multiline.extend(gen_node(n, context));
            any = true;
        }
    }
    if any {
        multiline.push_signal(Signal::SpaceIfNotTrailing);
    } else {
        // empty list
    }
    items.extend(multiline.take());
    items.push_str(end);

    items
}

fn gen_pattern_field(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    for n in node.children.iter() {
        match n.node_type {
            Type => {
                items.push_str(":");
                items.push_signal(Signal::SpaceIfNotTrailing);
                items.extend(gen_node(n, context));
            }
            EqualSign => {}
            Pattern => {
                items.push_signal(Signal::SpaceIfNotTrailing);
                items.push_str("=");
                items.push_signal(Signal::SpaceIfNotTrailing);
                items.extend(gen_node(n, context));
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
            WHITESPACE => {
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

fn gen_exp_non_dec(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();

    let mut indents = 0;
    for (i, n) in node.children.iter().enumerate() {
        match n.node_type {
            ColonEqual => {
                items.push_str(":=");
                items.push_signal(Signal::SpaceOrNewLine);
                items.push_signal(Signal::StartIndent);
                indents += 1;
            }
            _ => {
                items.extend(gen_node(n, context));
            }
        }
    }

    for _ in 0..indents {
        items.push_signal(Signal::FinishIndent);
    }

    items
}

fn gen_nodes_maybe_perenthesized(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();

    if node.is_surrounded_by(&BracketOpen, &BracketClose) {
        // pharentesized
        items.extend(gen_list(
            "(",
            ",",
            ")",
            &node.children_without_outer(),
            context,
        ));
        context.expect_space_or_newline();
    } else {
        items.extend(gen_nodes(&node.children, context));
    }

    items
}

fn gen_func_body(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();

    if node.has_child(&Exp) {
        items.push_str("=");
    }
    items.extend(gen_nodes(&node.children, context));

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

fn gen_debug(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    items.extend(context.gen_expected_space());
    println!("TODO: generate dprint IR from {:?}", node);
    for s in node.print("".into()).split("\n") {
        items.push_str(s);
        items.push_signal(Signal::ExpectNewLine);
    }
    items
}
