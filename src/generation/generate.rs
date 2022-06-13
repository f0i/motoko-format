use dprint_core::formatting::*;

use super::context::Context;
use super::helpers::*;
use crate::configuration::Configuration;
use crate::motoko_parser::Node;

pub fn generate(nodes: &Vec<Node>, text: &str, config: &Configuration) -> PrintItems {
  let mut context = Context::new(text, config);
  let mut items = PrintItems::new();
  
  items.extend(gen_nodes(nodes, &mut context));
  items.extend(Signal::NewLine.into());

  items
}

fn gen_node<'a>(node: &Node, context: &mut Context) -> PrintItems {
  let mut items = PrintItems::new();

  context.set_current_node(node.clone());
  items.extend(match node {
    Node::Program {nodes} => gen_nodes(nodes, context),
    Node::Import {what, from, with_equal} => {
      gen_import(what, from, context)
    },
    _ => {
      let mut i = PrintItems::new();
      println!("TODO: generate dprint IR from {:?}", node);
      for s in node.print().split("\n") {
        i.push_str(s);
      }
      i
    },
  });
  context.pop_current_node();
  items
}


fn gen_nodes<'a>(nodes: &Vec<Node>, context: &mut Context) -> PrintItems {
  let mut items = PrintItems::new();

  for node in nodes {
    items.extend(gen_node(node, context));
  }
  items
}

fn gen_import(what: &Node, from: &Node, context: &mut Context) -> PrintItems {
  let mut items = PrintItems::new();

  items.push_str("import");
  items.push_signal(Signal::SpaceIfNotTrailing);
  items.extend(gen_node(what, context));
  items.push_signal(Signal::SpaceIfNotTrailing);
  items.extend(gen_node(from, context));
  items.extend(";".into());

  //if let Some(value) = &node.value {
  //  items.push_str("=");
  //  items.extend(gen_node(value.into(), context));
  //}

  items
}