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
    ($($rule:ident),* $(,)?) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum NodeType {
            $(
                $rule,
            )+
            Unknown(String)
        }

        impl NodeType {
            fn from_pair(pair: &Pair<Rule>) -> NodeType {
                match pair.as_rule() {
                $(
                    Rule::$rule => NodeType::$rule,
                )+
                    rule => NodeType::Unknown(format!("{:?}", rule))
                }
            }
        }
    };
}

make_node_types! {
    Program,
    CompleteImport,
    CompleteDeclaration,
    Import,
    LineComment,
    DocComment,
    BlockComment,
    PatternPlain,
    PatternNullary,
    EOI,
    WHITESPACE,
    Semicolon,
    Id,
    COMMENT,
    Comment,
    Text,
    EqualSign,
    LineCommentContent,
    DocCommentContent,
    BlockCommentContent,
    Declaration,
    Lit,
    ShouldNewline,
    C,
}

#[derive(Debug, Clone)]
pub struct Declaration {}

pub fn parse(content: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let mut pairs = MotokoParser::parse(Rule::Motoko, &content)?;
    for pair in pairs.next().unwrap().into_inner() {
        ast.push(Node::from_pair(pair))
    }
    Ok(ast)
}

impl Node {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let node_type = NodeType::from_pair(&pair);
        // TODO?: replace "\t" in comments and strings with "\\t"
        let original = pair.as_str().replace("\t", "  ").into();
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
        let strings: Vec<String> = self
            .children
            .clone()
            .into_iter()
            .map(|node| node.print(format!("{}|  ", indent)).into())
            .collect();
        let children = strings.join("\n");
        if self.children.is_empty() {
            format!("{}{:?}(  {:?}  )", indent, self.node_type, self.original)
        } else {
            format!(
                "{}{:?}(  {:?}\n{}\n{})",
                indent, self.node_type, self.original, children, indent
            )
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(format!("{}\n", self.print("".into())).as_str())
        //f.write_str(&format!("{:?}({:?})", self.node_type, self.children))
    }
}
