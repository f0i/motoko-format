#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use pest::Parser;
use std::io::BufRead;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Calculator;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left),
            Operator::new(power, Right),
        ])
    };
}

fn eval(expression: Pairs<Rule>) -> f64 {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::num => pair.as_str().parse::<f64>().unwrap(),
            Rule::expr => eval(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: f64, op: Pair<Rule>, rhs: f64| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::subtract => lhs - rhs,
            Rule::multiply => lhs * rhs,
            Rule::divide => lhs / rhs,
            Rule::power => lhs.powf(rhs),
            _ => unreachable!(),
        },
    )
}

fn main() {
    let stdin = std::io::stdin();

    for line in vec![Some("1+2*3")] {
        // stdin.lock().lines() {
        let line = line.unwrap();
        let parse_result = Calculator::parse(Rule::calculation, &line);

        match parse_result {
            Ok(calc) => println!(" = {}", eval(calc)),
            Err(_) => println!(" Syntax error"),
        }
    }
}

/*







use std::fs;
mod ast;

fn main() {
    let input: String = fs::read_to_string("tests/comment.mo")
        .expect("cannot read file")
        .into();

    println!("-----------------");
    println!("{}", input);
    println!("-----------------");

    //let a = ast::parse("// test".into());
    let a = parse(input);
    println!("{:?}", a);
    println!("-----------------");
    println!("{}", print(a.unwrap()));
    println!("-----------------");
}

#[grammar = "motoko.pest"]
pub struct MotokoParser;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Rule {
    plus,
    minus,
    times,
    divide,
    power,
}
static CLIMBER: PrecClimber<Rule> = prec_climber![
    L   plus | minus,
    L   times | divide,
    R   power,
];

#[derive(Debug)]
pub enum Node {
    Comment(Comment),
    Line(usize),
    EOI,
}

#[derive(Debug)]
pub enum Comment {
    Doc(String),
    Line(String),
    Block(String),
}

pub fn parse(content: String) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let mut pairs = MotokoParser::parse(Rule::Motoko, &content)?;
    for pair in pairs.next().unwrap().into_inner() {
        println!("{:?}", build_ast_node(pair.clone()));
        ast.extend(build_ast_node(pair));
    }
    Ok(ast)
}

fn build_ast_node(pair: pest::iterators::Pair<Rule>) -> Vec<Node> {
    match (pair.as_rule()) {
        Rule::Comment => pair.into_inner().map(|p| build_ast_comment(p)).collect(),
        Rule::Lines => vec![Node::Line(pair.as_str().matches("\n").count())],
        // others
        Rule::EOI => vec![Node::EOI],
        rule => panic!("Unhandled parser rule {:?}", rule),
    }
}

fn build_ast_comment(pair: pest::iterators::Pair<Rule>) -> Node {
    let c = match (pair.as_rule()) {
        Rule::DocComment => Comment::Doc(pair.as_str().into()),
        Rule::LineComment => Comment::Line(pair.as_str().into()),
        Rule::BlockComment => Comment::Line(pair.as_str().into()),
        rule => panic!("Unhandled parser rule {:?}", rule),
    };
    Node::Comment(c)
}

pub fn print(nodes: Vec<Node>) -> String {
    nodes.iter().map(|n| print_node(&n)).collect::<String>()
}

pub fn print_node(node: &Node) -> String {
    match node {
        Node::Comment(Comment::Doc(s)) => s.clone(),
        Node::Comment(Comment::Line(s)) => s.clone(),
        Node::Comment(Comment::Block(s)) => s.clone(),
        Node::Line(n) => "\n".repeat(*n).into(),
        Node::EOI => "".into(),
        _ => panic!("No printer for Node {:?}", node),
    }
}
*/
