/// Parser to generate AST Nodes
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "motoko_parser/motoko.pest"]
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
    Raw(String),
    Text(String),

    Program {
        nodes: Vec<Node>,
    },
    Import {
        what: Box<Node>,
        from: Box<Node>,
        with_equal: bool,
    },
    PatternNullary {
        content: String,
    },
    None(String),
}

#[derive(Debug, Clone)]
pub struct Declaration {}

#[derive(Debug, Clone)]
pub enum CommentType {
    Doc,
    Line,
    Block,
}

pub fn parse(content: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
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
    Node::from_nth_inner_pair(pairs.peek().unwrap(), 0)
    //Node::from_pair(pairs.peek().unwrap().into_inner().next().unwrap())
}

impl Node {
    fn from_pair(pair: Pair<Rule>) -> Node {
        let raw: String = pair.as_str().into();
        //println!("pair: {:?}", pair);
        match pair.as_rule() {
            Rule::COMMENT => parse_comment(raw.as_str()),
            Rule::WHITESPACE => Node::WHITESPACE(raw),

            Rule::Program => Node::Program {
                nodes: Self::from_inner_pairs(pair),
            },
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

            Rule::DocComment => Self::from_nth_inner_pair(pair, 0),
            Rule::LineComment => Self::from_nth_inner_pair(pair, 0),
            Rule::BlockComment => Self::from_nth_inner_pair(pair, 0),
            Rule::Import => {
                let mut sub = Self::from_inner_pairs(pair)
                    .into_iter()
                    .filter(|p| matches!(p, Node::PatternNullary { .. } | Node::Text(_)));

                let what = sub.next().unwrap();
                let from = sub.next().unwrap();

                Node::Import {
                    // TODO
                    what: Box::new(what),
                    from: Box::new(from),
                    with_equal: false,
                }
            }
            Rule::PatternNullary => Node::PatternNullary { content: raw },
            Rule::Text => Node::Text(raw),

            // Others
            Rule::InvalidPart => Node::Raw(raw),
            Rule::Semicolon | Rule::EOL | Rule::EOI => Node::None(format!("{:?}", pair.as_rule())),
            Rule::Declaration => Node::Raw(raw), // TODO

            rule => panic!("Unhandled parser rule {:?}", rule),
        }
    }

    fn from_inner_pairs(pair: Pair<Rule>) -> Vec<Node> {
        Self::from_pairs(pair.into_inner().collect())
    }

    fn from_pairs(pairs: Vec<Pair<Rule>>) -> Vec<Node> {
        pairs
            .into_iter()
            .map(|pair| Self::from_pair(pair))
            .collect()
    }

    fn from_nth_inner_pair(pair: Pair<Rule>, n: usize) -> Node {
        let rule_string = format!("{:?}", pair.as_rule());

        if let Some(p) = pair.into_inner().into_iter().nth(n) {
            Self::from_pair(p)
        } else {
            Node::None(rule_string)
        }
    }
}

pub fn print(nodes: Vec<Node>) -> String {
    let options = PrintOptions::default();
    nodes.iter().map(|n| n.print()).collect::<String>()
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
    pub fn print(&self) -> String {
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
            Node::Raw(s) => s.clone(),
            Node::Program { nodes } => {
                let node_strings: Vec<String> =
                    nodes.into_iter().map(|node| node.print()).collect();
                format!("{}", node_strings.join("\n"))
            }
            Node::Import {
                what,
                from,
                with_equal,
            } => {
                if *with_equal {
                    format!("{} = {};", what.print(), from.print())
                } else {
                    format!("{} {};", what.print(), from.print())
                }
            }
            Node::PatternNullary { content } => content.into(),
            Node::Text(t) => t.into(),
            Node::None(_) => "".into(),
        };
        formatted
    }

    pub fn debug(&self) -> String {
        let formatted = match self {
            Node::Program { nodes } => {
                let strings: Vec<String> = nodes.into_iter().map(|node| node.debug()).collect();
                strings.join("\n")
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
