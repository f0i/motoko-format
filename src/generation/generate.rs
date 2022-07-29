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
        Program => gen_program(&node, context),
        ImportList => gen_list_body(";", &node.children, context, true, 3),
        DeclarationList => gen_list_body(";", &node.children, context, true, 3),

        Import => gen_import(&node, context),
        Declaration => gen_nodes(&node.children, context),

        Id => gen_id(&node, context),

        TypeVariant => gen_list("{", ";", "}", &node.children, context, false, 1),

        PatternNullary => gen_pattern_nullary(&node, context),
        PatternPlain => gen_nodes_maybe_perenthesized(&node, context),
        Text => gen_id(&node, context),
        COMMENT => gen_nodes(&node.children, context),
        Comment => gen_comment(&node, context),
        LineComment => gen_comment_line("//", &node, context),
        DocComment => gen_comment_line("///", &node, context),
        BlockComment => gen_comment_block(&node, context),
        LineCommentContent => gen_id_trim(&node, context),
        DocCommentContent => gen_id_trim(&node, context),
        BlockCommentContent => gen_id_multiline(&node, context),
        SpacedComment => gen_spaced_comment(&node, context),
        Lit => gen_id(&node, context),
        ShouldNewline => gen_should_newline(&node, context),
        PatternField => gen_pattern_field(&node, context),
        ExpNonDec => gen_exp_non_dec(&node, context),

        Exp | ExpNonVar | ExpPlain | ExpBin | ExpNullary | ExpNest | ExpPost | DeclarationField
        | Type | TypeNoBin | TypeUn | TypePre | TypeItem | ExpBinContinue | SharedPattern
        | SharedPattern2 | ClassBody | Case | DeclarationVar => gen_nodes(&node.children, context),

        PatternUn | TypeTag => gen_nodes_no_space_between(&node.children, context),

        ExpUn => gen_exp_un(node, context),

        DeclarationNonVar => gen_declaration_non_var(&node, context),

        ExpPostContinue => {
            context.reset_expect();
            gen_nodes_maybe_perenthesized(&node, context)
        }

        ObjSort | Visibility | KeywordActor | KeywordAnd | KeywordAssert | KeywordAsync
        | KeywordAwait | KeywordBreak | KeywordCase | KeywordCatch | KeywordClass
        | KeywordContinue | KeywordDebug | KeywordDebugShow | KeywordDo | KeywordElse
        | KeywordFlexible | KeywordFalse | KeywordFor | KeywordFromCandid | KeywordFunc
        | KeywordIf | KeywordIgnore | KeywordImport | KeywordIn | KeywordModule | KeywordNot
        | KeywordNull | KeywordObject | KeywordOr | KeywordLabel | KeywordLet | KeywordLoop
        | KeywordPrivate | KeywordPublic | KeywordQuery | KeywordReturn | KeywordShared
        | KeywordStable | KeywordSwitch | KeywordSystem | KeywordThrow | KeywordToCandid
        | KeywordTrue | KeywordTry | KeywordType | KeywordVar | KeywordWhile => {
            gen_keyword(node, context)
        }

        Colon | Arrow | BinAssign | EqualSign => {
            context.expect_space();
            gen_keyword(node, context)
        }

        HashTag | KeywordUnderscore => gen_id(node, context),

        Questionmark => {
            let i = gen_id(node, context);
            context.reset_expect();
            i
        }

        BinOp | RelOp => {
            context.possible_newline();
            gen_keyword(node, context)
        }

        Block | ObjBody => {
            let force_multiline = count_lines(&node.original) > 0;
            gen_list("{", ";", "}", &node.children, context, force_multiline, 1)
        }

        FuncBody => gen_func_body(node, context),

        ExpPostFirst | PatternBin | TypeNullary => gen_nodes_maybe_perenthesized(&node, context),
        ExpList => {
            context.reset_expect();
            gen_list("(", ",", ")", &node.children, context, false, 2)
        }
        Dot | TypeBindList => gen_id_no_space(&node, context),

        TypeArgs => {
            context.reset_expect();
            let force_multiline = count_lines(&node.original) > 0;
            gen_list("<", ",", ">", &node.children, context, force_multiline, 2)
        }

        WHITESPACE | Semicolon | EOI => gen_ignore(&node, context),

        Pattern => gen_id_trim(&node, context), // TODO
        // TODO: remove to handle all cases
        //_ => gen_id(&node, context),
        _ => gen_debug(node, context),
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

fn gen_program(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();

    let mut after_import = false;
    let mut after_declarations = false;
    let mut lines = 0;

    for n in node.children.iter() {
        match n.node_type {
            ImportList => {
                items.extend(gen_node(n, context));
                after_import = count_not_ignored_or_comment(&n.children) > 0;
                lines = 0;
            }
            DeclarationList => {
                if count_not_ignored_or_comment(&n.children) > 0 {
                    if after_import {
                        items.extend(gen_newlines(2));
                        after_import = false;
                    }
                    lines = 0;
                    after_declarations = true;
                }
                items.extend(gen_node(n, context));
            }
            WHITESPACE => lines = count_lines(&n.original),

            _ => {
                assert!(is_comment(n));
                if after_import && lines > 0 {
                    items.extend(gen_newlines(2));
                    after_import = false;
                    lines = 0;
                }
                items.extend(gen_node(n, context));
            }
        }
    }
    if after_import || after_declarations {
        items.extend(gen_newlines(1));
    }
    items
}

fn gen_import(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
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

    items.push_signal(Signal::FinishIndent);
    items.push_signal(Signal::FinishIndent);

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
    debug("gen_id", items)
}

fn gen_id_no_space(node: &Node, context: &mut Context) -> PrintItems {
    context.reset_expect();
    let items = gen_id(node, context);
    context.reset_expect();
    items
}

fn gen_id_trim(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    items.extend(context.gen_expected_space());

    let mut first = true;
    for l in node.original.trim().split("\n") {
        if !first {
            items.push_signal(Signal::NewLine);
        }
        first = false;
        items.push_str(l);
    }
    context.expect_space();
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
    //context.expect_space();
    items.extend(context.gen_expected_space());

    items.extend(gen_id_trim(node, context));
    context.expect_space();
    context.force_space();
    debug("gen_keyword", items)
}

fn gen_comment_line(pre: &str, node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    let spaces = gen_spaces(2);
    context.expect_space();
    items.extend(if_not_start_of_line(spaces));

    items.push_signal(Signal::StartForceNoNewLines);
    items.push_str(pre);
    context.expect_space();
    // TODO: wrap / reflow text?
    items.extend(gen_nodes(&node.children, context));
    items.push_signal(Signal::FinishForceNoNewLines);
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

fn gen_declaration_non_var(node: &Node, context: &mut Context) -> PrintItems {
    if node.has_child(&KeywordLet) {
        return gen_let_statement(node, context);
    }
    let mut items = PrintItems::new();
    //TODO: this is ugly. abstract it
    items.push_signal(Signal::StartNewLineGroup);
    items.push_signal(Signal::StartNewLineGroup);
    items.push_signal(Signal::StartNewLineGroup);
    let mut groups = 3;

    items.push_signal(Signal::QueueStartIndent);
    let mut indent = true;

    for n in node.children.iter() {
        match n.node_type {
            ObjBody | FuncBody | ClassBody => {
                if indent {
                    items.push_signal(Signal::FinishIndent);
                    indent = false;
                }
                items.extend(gen_node(n, context));
            }
            PatternPlain => {
                // Function parameter
                context.reset_expect();
                items.extend(gen_node(n, context));
                if groups > 0 {
                    groups -= 1;
                    items.push_signal(Signal::FinishNewLineGroup);
                }
            }
            Type => {
                items.extend(gen_node(n, context));
                if groups > 0 {
                    groups -= 1;
                    items.push_signal(Signal::FinishNewLineGroup);
                }
            }
            EqualSign => {
                items.extend(gen_node(n, context));
                if groups > 0 {
                    groups -= 1;
                    items.push_signal(Signal::FinishNewLineGroup);
                    items.push_signal(Signal::PossibleNewLine);
                }
            }
            _ => {
                items.extend(gen_node(n, context));
            }
        }
    }

    if indent {
        items.push_signal(Signal::FinishIndent);
    }

    while groups > 0 {
        groups -= 1;
        items.push_signal(Signal::FinishNewLineGroup);
    }

    items
}

fn gen_let_statement(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = MultiLineGroup::new(false, 1, true, "let_statement");

    for n in node.children.iter() {
        match n.node_type {
            EqualSign => {
                items.extend(gen_node(&n, context));
                items.possible_newline();
            }
            _ => items.extend(gen_node(&n, context)),
        }
    }

    items.take()
}

fn gen_pattern_nullary(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();

    items.extend(context.gen_expected_space());

    if node.has_child(&PatternField) {
        assert!(node.is_surrounded_by(&CurlyBracketOpen, &CurlyBracketClose));
        items.extend(gen_list(
            "{",
            ";",
            "}",
            &node.children_without_outer(),
            context,
            false,
            1,
        ));
    } else {
        items.extend(gen_nodes(&node.children, context))
    }

    items
}

fn gen_nodes_no_space_between(nodes: &Vec<Node>, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    let len = nodes.len();
    for (i, n) in nodes.iter().enumerate() {
        items.extend(gen_node(&n, context));
        if i < (len - 1) {
            context.reset_expect();
        }
    }
    items
}

fn gen_list(
    start: &str,
    sep: &str,
    end: &str,
    nodes: &Vec<Node>,
    context: &mut Context,
    force_multiline: bool,
    // spaces or newlines will be added after `start` and brefor `end`
    // if at least `space` not_ignored nodes are in `nodes`
    space: usize,
) -> PrintItems {
    let mut items = PrintItems::new();

    let count = count_not_ignored_or_comment(nodes);

    items.extend(context.gen_expected_space());

    items.push_str(start);

    let mut multiline = MultiLineGroup::new(force_multiline, 1, false, "gen_list");
    multiline.possible_newline();

    if count >= space {
        context.expect_space_or_newline();
    }

    multiline.extend(gen_list_body(sep, nodes, context, force_multiline, 2));

    if count >= space {
        multiline.push_signal(Signal::SpaceIfNotTrailing);
    }

    multiline.possible_newline();
    items.extend(multiline.take());
    items.push_str(end);
    context.expect_space_or_newline();

    items
}

fn gen_list_body(
    sep: &str,
    nodes: &Vec<Node>,
    context: &mut Context,
    force_multiline: bool,
    keep_newlines: usize,
) -> PrintItems {
    let mut items = MultiLineGroup::new(force_multiline, 0, false, "gen_list_body");

    let count = count_not_ignored_or_comment(nodes);
    let mut need_separator = false;
    let mut index = 0; // current generated node index
    let mut counter = 0; // count not_ignored nodes that have been printed
    let mut lines = 0;
    let mut allow_newlines = false;

    for (i, n) in nodes.iter().enumerate() {
        index = i;
        if is_whitespace(n) {
            lines = count_lines(&n.original).clamp(0, keep_newlines);
        } else if is_ignored(n) {
        } else if is_comment(n) {
            if need_separator {
                items.push_str(sep);
                context.expect_space_or_newline();
                need_separator = false;
            }
            if allow_newlines {
                items.extend(gen_newlines(lines));
            }
            allow_newlines = true;
            lines = 0;
            items.extend(gen_node(n, context));
        } else {
            if need_separator {
                items.push_str(sep);
                context.expect_space_or_newline();
            }
            if lines > 0 && allow_newlines {
                items.extend(gen_newlines(lines));
                context.reset_expect();
            }
            allow_newlines = true;
            lines = 0;
            items.extend(gen_node(n, context));
            need_separator = true;

            counter += 1;
            if counter >= count {
                // last not_ignored node was just generated
                break;
            }
        }
    }

    if count > 0 {
        items.if_multiline(sep.to_string().into());
    }

    let mut lines = 0;
    for n in nodes.iter().skip(index).skip(1) {
        match n.node_type {
            WHITESPACE => {
                lines = count_lines(&n.original).clamp(0, keep_newlines);
            }
            _ if is_ignored(n) => {}
            _ => {
                items.extend(gen_newlines(lines));
                lines = 0;
                items.extend(gen_node(n, context));
            }
        }
    }

    items.take()
}

fn gen_pattern_field(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    for n in node.children.iter() {
        match n.node_type {
            Type => {
                items.push_str(":");
                context.expect_space();
                items.extend(gen_node(n, context));
            }
            EqualSign => {}
            Pattern => {
                items.extend(context.gen_expected_space());
                items.push_str("=");
                context.expect_space();
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

fn gen_exp_un(node: &Node, context: &mut Context) -> PrintItems {
    let items = if node.has_child(&HashTag) {
        gen_nodes_no_space_between(&node.children, context)
    } else {
        gen_nodes(&node.children, context)
    };
    context.expect_space();

    items
}

fn gen_exp_non_dec(node: &Node, context: &mut Context) -> PrintItems {
    if node.has_child(&KeywordSwitch) {
        return gen_switch(node, context);
    }
    if node.has_child(&KeywordIf) {
        return gen_if_statement(node, context);
    }
    if node.has_child(&KeywordLabel) {
        return gen_label(node, context);
    }
    let mut items = PrintItems::new();
    let is_for_loop = node.has_child(&KeywordFor);
    items.push_signal(Signal::QueueStartIndent);
    let mut indent = true;
    for n in node.children.iter() {
        match n.node_type {
            ColonEqual => {
                // TODO: generalize or refactor
                items.push_signal(Signal::SpaceOrNewLine);
                items.push_str(":=");
            }
            Exp => {
                if indent {
                    items.push_signal(Signal::FinishIndent);
                    indent = false;
                }
                items.extend(gen_node(n, context));
            }
            ExpNest => {
                if is_for_loop {
                    items.push_str(")");
                }
                if indent {
                    items.push_signal(Signal::FinishIndent);
                    indent = false;
                }
                items.extend(gen_node(n, context));
            }
            KeywordFor => {
                items.extend(gen_node(n, context));
                items.extend(context.gen_expected_space());
                items.push_str("(");
            }
            _ => {
                items.extend(gen_node(n, context));
            }
        }
    }

    if indent {
        items.push_signal(Signal::FinishIndent);
    }

    items
}

fn gen_switch(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    let pre = node
        .children
        .iter()
        .take_while(|n| n.node_type != CurlyBracketOpen);
    let post: Vec<Node> = node
        .children_without_outer()
        .iter()
        .skip_while(|n| n.node_type != CurlyBracketOpen)
        .skip(1)
        .map(|n| n.clone())
        .collect();

    for n in pre {
        match n.node_type {
            ExpNullary => {
                context.force_space();
                items.extend(gen_node(&n, context));
            }
            _ => items.extend(gen_node(&n, context)),
        }
    }

    items.extend(gen_list("{", ";", "}", &post, context, true, 1));

    items
}

fn gen_if_statement(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = MultiLineGroup::new(false, 0, false, "if_statement");
    for n in node.children.iter() {
        items.extend(gen_node(&n, context));
    }
    items.take()
}

fn gen_label(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = MultiLineGroup::new(false, 0, false, "label");
    for n in node.children.iter() {
        match n.node_type {
            ExpNest => {
                items.possible_newline();
                items.extend(gen_node(&n, context));
            }
            _ => items.extend(gen_node(&n, context)),
        }
    }
    items.take()
}

fn gen_nodes_maybe_perenthesized(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();
    items.extend(context.gen_expected_space());

    if node.is_surrounded_by(&RoundBracketOpen, &RoundBracketClose) {
        // pharentesized
        items.extend(gen_list(
            "(",
            ",",
            ")",
            &node.children_without_outer(),
            context,
            false,
            2,
        ));
        context.expect_space_or_newline();
    } else if node.is_surrounded_by(&SquareBracketOpen, &SquareBracketClose) {
        // pharentesized
        items.extend(gen_list(
            "[",
            ",",
            "]",
            &node.children_without_outer(),
            context,
            false,
            2,
        ));
        context.expect_space_or_newline();
    } else {
        items.extend(gen_nodes(&node.children, context));
    }

    items
}

fn gen_func_body(node: &Node, context: &mut Context) -> PrintItems {
    let mut items = PrintItems::new();

    items.push_signal(Signal::StartNewLineGroup);
    let mut indent = false;
    // child is either Exp or Block
    if node.has_child(&Exp) {
        // single expression
        context.expect_space();
        //items.push_signal(Signal::QueueStartIndent);
        //indent = true;
        items.extend(context.gen_expected_space());
        items.push_str("=");
        context.expect_space_or_newline();
    } else {
        // block as function body
        assert!(node.has_child(&Block));
    }
    items.extend(gen_nodes(&node.children, context));
    if indent {
        items.push_signal(Signal::FinishIndent);
    }
    items.push_signal(Signal::FinishNewLineGroup);

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

fn debug(_name: &str, ir: PrintItems) -> PrintItems {
    let mut items = PrintItems::new();
    //items.push_str("·");
    //items.push_str(name);
    //items.push_str("…");
    items.extend(ir);
    //items.push_str("·");
    items
}
