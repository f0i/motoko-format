/// Parser to generate AST Nodes
use core::fmt;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "motoko_parser/motoko.pest"]
pub struct MotokoParser;

#[derive(Clone)]
pub struct Node {
    pub original: String,
    pub start: usize,
    pub end: usize,
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

macro_rules! make_node_types {
    ($($rule:ident => $type:ident),* $(,)?) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum NodeType {
            $(
                $type,
            )+
            Unknown(String)
        }

        impl NodeType {
            fn from_pair(pair: &Pair<Rule>) -> NodeType {
                match pair.as_rule() {
                $(
                    Rule::$rule => NodeType::$type,
                )+
                    rule => NodeType::Unknown(format!("{:?}", rule))
                }
            }
        }
    };
}

make_node_types! {
    Program => Program,
    Import => Import,
    LineComment => InlineComment,
    DocComment => DocComment,
    BlockComment => BlockComment,
    PatternPlain => PatternPlain,
    PatternNullary => PatternNullary,
    EOI => Eoi,
    WHITESPACE => Whitespace,
    Semicolon => Semicolon,
    Id => Id,
    COMMENT => Comment,
    Text => Text,
    EqualSign => EqualSign,
    LineCommentContent => LineCommentContent,
    DocCommentContent =>  DocCommentContent,
    BlockCommentContent => BlockCommentContent,
    CheckWhitespace => CheckWhitespace,
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
    Node::new(pairs.peek().unwrap())
}

impl Node {
    fn new(pair: Pair<Rule>) -> Self {
        Node::from_pair(pair)
    }

    fn get_child(&self, node_type: &NodeType) -> Option<Node> {
        for node in self.children.iter() {
            if node.node_type == *node_type {
                return Some(node.clone());
            }
        }
        None
    }

    fn from_pair(pair: Pair<Rule>) -> Self {
        let node_type = NodeType::from_pair(&pair);
        let original = pair.as_str().into();
        let start = pair.as_span().start();
        let end = pair.as_span().end();
        let children = Self::from_inner_pairs(pair);

        Node {
            node_type,
            children,
            start,
            end,
            original,
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
}

// TODO: remove all of the below:

impl Node {
    pub fn print(&self, indent: String) -> String {
        let formatted = match self {
            Node {
                node_type: NodeType::Program,
                children,
                ..
            } => {
                let strings: Vec<String> = children
                    .into_iter()
                    .map(|node| node.print(format!("{}  ", indent)))
                    .collect();
                strings.join("\n")
            }
            _ => format!("{}{:?}\n", indent, self),
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

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{:?}({:?})", self.node_type, self.children))
    }
}
