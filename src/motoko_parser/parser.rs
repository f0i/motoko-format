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
    Motoko,
    Header,
    Program,
    CompleteImport,
    CompleteDeclaration,
    Import,
    LineComment,
    DocComment,
    BlockComment,
    SpacedComment,
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
    EndOfImport,
    EndOfDeclaration,
    PatternField,
    Pattern,
    Type,
    DeclarationNonVar,
    ExpNonDec,
    ClassBody,
    DeclarationField,
    Exp,
    FuncBody,
    BinOp,
    RelOp,
    TypeBindList,
    // Keywords
    KeywordActor,
    KeywordAnd,
    KeywordAssert,
    KeywordAsync,
    KeywordAwait,
    KeywordBreak,
    KeywordCase,
    KeywordCatch,
    KeywordClass,
    KeywordContinue,
    KeywordDebug,
    KeywordDebugShow,
    KeywordDo,
    KeywordElse,
    KeywordFlexible,
    KeywordFalse,
    KeywordFor,
    KeywordFromCandid,
    KeywordFunc,
    KeywordIf,
    KeywordIgnore,
    KeywordImport,
    KeywordIn,
    KeywordModule,
    KeywordNot,
    KeywordNull,
    KeywordObject,
    KeywordOr,
    KeywordLabel,
    KeywordLet,
    KeywordLoop,
    KeywordPrivate,
    KeywordPublic,
    KeywordQuery,
    KeywordReturn,
    KeywordShared,
    KeywordStable,
    KeywordSwitch,
    KeywordSystem,
    KeywordThrow,
    KeywordToCandid,
    KeywordTrue,
    KeywordTry,
    KeywordType,
    KeywordVar,
    KeywordWhile,
    //
}

#[derive(Debug, Clone)]
pub struct Declaration {}

pub fn parse(content: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let mut pairs = MotokoParser::parse(Rule::Motoko, &content)?;
    let pair = pairs.next().unwrap();
    ast.push(Node::from_pair(pair));

    Ok(ast)
}

#[cfg(test)]
fn parse_with(
    content: &str,
    rule: Rule,
) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let mut pairs = MotokoParser::parse(rule, &content)?;
    let pair = pairs.next().unwrap();
    ast.push(Node::from_pair(pair));

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

    #[cfg(test)]
    fn get_one_descendant(&self, node_type: &NodeType) -> Option<Node> {
        if self.node_type == *node_type {
            return Some(self.clone());
        }
        for child in self.children.iter() {
            if let Some(n) = child.get_one_descendant(node_type) {
                return Some(n);
            }
        }
        None
    }
}

// TODO: remove all of the below:

impl Node {
    pub fn print(&self, indent: String) -> String {
        let strings: Vec<String> = self
            .children
            .clone()
            .into_iter()
            .map(|node| node.print(format!("{}│ ", indent)).into())
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

#[cfg(test)]
mod test_parsers {
    use super::*;

    #[macro_export]
    macro_rules! expect_parse {
        ($content:expr, $rule:expr, $expected:expr $(,)?) => {
            let nodes = match parse_with($content, $rule) {
                Ok(nodes) => nodes,
                Err(err) => {
                    println!("Parsing error:\n{}", err);
                    panic!("Couldn't parse");
                }
            };
            let node = nodes.first().expect("schould contain node, but was empty");
            println!("----- success ------\n{:?}\n----- end -----", node);
            assert_eq!(nodes.len(), 1, "Each Rule should return exactly one Pair");
            if let Some(_sub) = node.get_one_descendant(&$expected) {
                assert_eq!(
                    node.original, $content,
                    "Rule::{:?} did not consume the whole input",
                    $rule
                );
            } else {
                println!("Expected Node to contain a {:?}\n", $expected);
                println!("Node was: \n{:?}", node);
                panic!("does not contain {:?}", $expected);
            }
        };
    }

    #[test]
    fn test_function() {
        expect_parse!(
            "func asdf() { return 1; }",
            Rule::DeclarationNonVar,
            NodeType::KeywordReturn
        );

        expect_parse!(
            "public func idQuick() : async Principal { return; }",
            Rule::DeclarationField,
            NodeType::Declaration
        );

        expect_parse!("(this)", Rule::ExpPlain, NodeType::Exp);
        expect_parse!("(this)", Rule::ExpNullary, NodeType::Exp);
        expect_parse!("(this)", Rule::ExpPost, NodeType::Exp);
        expect_parse!("Principal.fromActor(this)", Rule::Exp, NodeType::Exp);

        expect_parse!(
            "public func idQuick() : async Principal { return Principal.fromActor(this); }",
            Rule::DeclarationField,
            NodeType::Declaration
        );
    }

    #[test]
    fn test_function_async() {
        parse_with(
            "func asdf() : async Principle { return 1; }",
            Rule::DeclarationNonVar,
        )
        .unwrap();

        parse_with(
            "func argument() : async Principal { return someone; };",
            Rule::DeclarationNonVar,
        )
        .unwrap();

        parse_with(
            "query func argument() : async Principal { return someone; };",
            Rule::DeclarationNonVar,
        )
        .unwrap();
    }

    #[test]
    fn test_class() {
        expect_parse!(
            "shared class MyClass(a: B) = {}",
            Rule::DeclarationNonVar,
            NodeType::ClassBody,
        );

        expect_parse!(
            "shared (install) actor class WhoAmI(someone : Principal) = this { }",
            Rule::DeclarationNonVar,
            NodeType::ClassBody,
        );

        expect_parse!(
            "shared (install) actor class WhoAmI(someone : Principal) = this { var i = 1; }",
            Rule::DeclarationNonVar,
            NodeType::ClassBody,
        );
    }

    #[test]
    fn test_class_body() {
        expect_parse!(
            "{ public query func installer() : async Principal { return; }; }",
            Rule::ClassBody,
            NodeType::ExpNonDec,
        );

        expect_parse!(
            "{ public query func installer() : async Principal { return; }; }",
            Rule::ClassBody,
            NodeType::Declaration,
        );
    }

    #[test]
    fn test_return_statement() {
        expect_parse!(
            "return Principal.fromActor(this)",
            Rule::Declaration,
            NodeType::KeywordReturn,
        );
    }

    #[test]
    fn test_expression() {
        expect_parse!(
            "func x { x + 1 }", //
            Rule::Exp,
            NodeType::FuncBody
        );

        expect_parse!(
            "let a2 = Array.map<Nat, Nat>(func x { x + 1 }, a)",
            Rule::Exp,
            NodeType::FuncBody,
        );

        expect_parse!(
            "type Op = Nat -> Nat", //
            Rule::Exp,
            NodeType::Type,
        );
    }

    #[test]
    fn test_styleguide_layout_spacing() {
        // sample code from https://internetcomputer.org/docs/current/developer-docs/build/languages/motoko/style/#spacing

        expect_parse!(
            "let z = - 2*x + 3*y + 4*(x*x + y*y)", //
            Rule::Declaration,
            NodeType::Lit,
        );

        expect_parse!(
            concat!(
                "4 + 5 <= 5 + 4;",
                "not (a or b and not c);",
                "v := 0;",
                "v += 1;",
            ),
            Rule::Motoko,
            NodeType::Lit,
        );

        expect_parse!(
            concat!(
                "var v = 0;", //
                "let r = { a = 1; b = 2 };",
            ),
            Rule::Motoko,
            NodeType::Lit
        );

        expect_parse!("(x:Nat)", Rule::PatternPlain, NodeType::Id);
        expect_parse!("((x, y) : (Nat, Nat))", Rule::PatternPlain, NodeType::Id);

        expect_parse!(
            concat!(
                "var v : Nat = 0;",
                "func foo(x : Nat, y : Nat) : Nat { x + y };",
                "func bar((x, y) : (Nat, Nat)) : Nat { x + y };",
                "let w = 1 ^ 0xff : Nat16;",
            ),
            Rule::Motoko,
            NodeType::Lit
        );
    }

    #[test]
    fn test_issues_in_dfinity_examples() {
        expect_parse!("(size * size) / 64 + 1", Rule::Exp, NodeType::Lit);

        expect_parse!(
            "let words = (size * size) / 64 + 1;",
            Rule::Motoko,
            NodeType::Lit
        );

        expect_parse!(
            "let j : Nat64 = 56 -% 8 *% Nat64.fromIntWrap(i);",
            Rule::Motoko,
            NodeType::BinOp
        );

        expect_parse!(
            "accountIdentifier.size() != 32",
            Rule::Declaration,
            NodeType::RelOp
        );

        expect_parse!(
            "func (i, j) { false; }",
            Rule::Declaration,
            NodeType::KeywordFalse,
        );

        expect_parse!("Nat", Rule::Type, NodeType::Id,);
        expect_parse!("(Nat,Nat)", Rule::Type, NodeType::Id,);
        expect_parse!("<(Nat,Nat)>", Rule::TypeArgs, NodeType::Id,);
        expect_parse!("List.nil<(Nat,Nat)>", Rule::TypeNullary, NodeType::Id,);
        expect_parse!("var cs = List.nil<(Nat,Nat)>()", Rule::Exp, NodeType::Id,);

        expect_parse!("\"P\" # Nat32", Rule::ExpBin, NodeType::Id);
        expect_parse!("\"P\"# Nat32()", Rule::ExpBin, NodeType::Id);
        expect_parse!(
            "\"Placing order \"# Nat32.toText(id)",
            Rule::Exp,
            NodeType::Id
        );
    }
}
