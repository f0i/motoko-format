extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::iterators::Pair;
use pest::Parser;

use std::fs;

fn main() {
    //let file = "tests/comment.mo";
    let files = vec![
        //"tests/comment.mo",
        "tests/imports.mo",
        //"tests/main.mo",
        //"tests/module.mo",
        //"tests/lit.mo",
    ];

    for file in files {
        println!("{:?}", file);
        let input: String = fs::read_to_string(file)
            .expect(&format!("cannot read {}", file))
            .into();

        println!("-----------------");
        println!("{}", input);
        println!("-----------------");

        //let a = ast::parse("// test".into());
        let a = parse(input);
        match a {
            Ok(x) => {
                println!("{:?}", x);
                println!("-----------------");
                println!("{}", print(x.clone()));
                println!("-----------------");
                println!("{}", debug(x));
                println!("-----------------");
            }
            Err(ref e) => println!("{}\n{:?}", e, e),
        }
    }
}

#[derive(Parser)]
#[grammar = "motoko2.pest"]
pub struct MotokoParser;

#[derive(Debug, Clone)]
pub struct PrintOptions {
    indent: usize,
    line_width: usize,
}

#[derive(Debug, Clone)]
pub enum Node {
    Comment {
        kind: CommentType,
        content: String,
    },
    WHITESPACE(String),
    EOI,
    Unhandled(String, String),

    Program {
        imports: Vec<Node>,
        declarations: Vec<Node>,
    },
    Import {
        what: String,
        from: String,
        with_equal: bool,
    },
    PatternNullary {
        content: String,
    },
}

#[derive(Debug, Clone)]
pub struct Declaration {}

#[derive(Debug, Clone)]
pub enum CommentType {
    Doc,
    Line,
    Block,
}

pub fn parse(content: String) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let mut pairs = MotokoParser::parse(Rule::Motoko, &content)?;
    for pair in pairs.next().unwrap().into_inner() {
        //println!("{:?}", pair.clone());
        //ast.push(build_ast_node(pair));
        ast.push(Node::from_pair(pair))
    }
    Ok(ast)
}

// comments
fn parse_comment(content: &str) -> Node {
    let mut pairs = MotokoParser::parse(Rule::Comment, &content).unwrap();
    Node::from_pair(pairs.peek().unwrap().into_inner().next().unwrap())
}

impl Node {
    fn from_pair(pair: Pair<Rule>) -> Node {
        let raw: String = pair.as_str().into();
        match pair.as_rule() {
            Rule::COMMENT => parse_comment(raw.as_str()),
            Rule::WHITESPACE => Node::WHITESPACE(raw),

            Rule::Program => {
                let subnodes = Self::from_inner_pairs(pair);
                // TODO: use Vec::drain_filter when it's stable
                let (imports, declarations) = subnodes
                    .into_iter()
                    .filter(|s| !matches!(s, Node::Unhandled(_, _) | Node::WHITESPACE(_)))
                    .partition(|s| matches!(s, Node::Import { .. }));
                Node::Program {
                    imports,
                    declarations,
                }
            }
            Rule::DocCommentContent => Node::Comment {
                kind: CommentType::Doc,
                content: pair.as_str().into(),
            },
            Rule::LineCommentContent => Node::Comment {
                kind: CommentType::Line,
                content: pair.as_str().into(),
            },
            Rule::BlockCommentContent => Node::Comment {
                kind: CommentType::Block,
                content: pair.as_str().into(),
            },

            Rule::DocComment => Self::from_inner_pairs(pair).into_iter().nth(1).unwrap(),
            Rule::LineComment => Self::from_inner_pairs(pair).into_iter().nth(1).unwrap(),
            Rule::BlockComment => Self::from_inner_pairs(pair).into_iter().nth(1).unwrap(),
            Rule::Import => {
                let pattern = Self::from_nth_inner_pair(pair, 0);
                Node::Import {
                    // TODO
                    what: raw.into(),
                    from: "".into(),
                    with_equal: false,
                }
            }
            // Others
            Rule::Semicolon | Rule::EOL | Rule::EOI => {
                Node::Unhandled(raw, format!("{:?}", pair.as_rule()))
            }
            rule => panic!("Unhandled parser rule {:?}", rule),
        }
    }

    fn from_inner_pairs(pair: Pair<Rule>) -> Vec<Node> {
        Self::from_pairs(pair.into_inner().collect())
    }

    fn from_pairs(pairs: Vec<Pair<Rule>>) -> Vec<Node> {
        // println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
        // println!("VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV");
        // println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
        // println!("{:?}", pairs);
        pairs
            .into_iter()
            .map(|pair| Self::from_pair(pair))
            .collect()
    }

    fn from_nth_inner_pair(pair: Pair<Rule>, n: usize) -> Node {
        if let Some(p) = pair.into_inner().into_iter().nth(n) {
            Self::from_pair(p)
        } else {
            Node::Unhandled("".into(), format!("No pairs"))
        }
    }
}

pub fn print(nodes: Vec<Node>) -> String {
    let options = PrintOptions::default();
    nodes.iter().map(|n| n.print(&options)).collect::<String>()
}
pub fn debug(nodes: Vec<Node>) -> String {
    nodes.iter().map(|n| n.debug()).collect::<String>()
}

impl PrintOptions {
    fn default() -> Self {
        Self {
            indent: 4,
            line_width: 100,
        }
    }
}

impl Node {
    fn print(&self, options: &PrintOptions) -> String {
        let formatted = match self {
            Node::Comment {
                kind: CommentType::Doc,
                content,
            } => format!("/// {}", content.trim()),
            Node::Comment {
                kind: CommentType::Line,
                content,
            } => format!("// {}", content.trim()),
            Node::Comment {
                kind: CommentType::Block,
                content,
            } => format!("/// {}", content.trim()),
            Node::WHITESPACE(s) => print_whitespace(&s),
            Node::Unhandled(s, _) => s.clone(),
            Node::EOI => "".into(),
            Node::Program {
                imports,
                declarations,
            } => {
                let imp: Vec<String> = imports
                    .into_iter()
                    .map(|node| node.print(options))
                    .collect();
                let dec: Vec<String> = declarations
                    .into_iter()
                    .map(|node| node.print(options))
                    .collect();
                format!("{}\n\n{}", imp.join("\n"), dec.join("\n"))
            }
            Node::Import {
                what,
                from,
                with_equal,
            } => {
                if *with_equal {
                    format!("{} = {};", what.trim(), from.trim())
                } else {
                    format!("{} {};", what.trim(), from.trim())
                }
            }
            Node::PatternNullary { content } => content.into(),
        };
        formatted
    }

    fn debug(&self) -> String {
        let formatted = match self {
            Node::EOI => "".into(),
            Node::Program {
                imports,
                declarations,
            } => {
                let imp: Vec<String> = imports.into_iter().map(|node| node.debug()).collect();
                let dec: Vec<String> = declarations.into_iter().map(|node| node.debug()).collect();
                format!("{}\n\n{}", imp.join("\n"), dec.join("\n"))
            }
            _ => format!("{:?}\n", self),
        };
        formatted
    }
}

fn print_whitespace(s: &String) -> String {
    match s.matches("\n").count() {
        0 => " ",
        1 => "\n",
        _ => "\n\n",
    }
    .into()
}
