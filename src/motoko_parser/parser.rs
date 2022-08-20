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
    Import,
    ImportList,
    Declaration,
    DeclarationList,
    LineComment,
    DocComment,
    BlockComment,
    SpacedComment,
    PatternPlain,
    PatternNullary,
    PatternBin,
    PatternUn,
    EOI,
    WHITESPACE,
    Semicolon,
    COMMENT,
    Comment,
    Id,
    Lit,
    Nat,
    Text,
    Num,
    ObjSort,
    ObjBody,
    EqualSign,
    Visibility,
    LineCommentContent,
    DocCommentContent,
    BlockCommentContent,
    ShouldNewline,
    PatternField,
    Pattern,
    SharedPattern,
    SharedPattern2,
    Type,
    TypeVariant,
    TypeTag,
    TypeNoBin,
    TypeUn,
    TypeNullary,
    TypePre,
    TypeArgs,
    TypeItem,
    TypeBind,
    TypeObj,
    TypeField,
    DeclarationVar,
    DeclarationNonVar,
    ClassBody,
    Block,
    DeclarationField,
    Exp,
    ExpBin,
    ExpBinContinue,
    ExpNest,
    ExpList,
    ExpNonDec,
    ExpNonVar,
    VarExpNonVar,
    ExpNullary,
    ExpPlain,
    ExpPost,
    ExpPostFirst,
    ExpPostContinue,
    ExpPostList,
    ExpUn,
    ExpObj,
    ExpField,
    FuncBody,
    BinOp,
    BinAssign,
    RelOp,
    UnOp,
    Case,
    Catch,
    Stability,
    FuncSort,
    TypeBindList,
    ColonEqual,
    Dot,
    Colon,
    Questionmark,
    HashTag,
    Arrow,
    RoundBracketOpen,
    RoundBracketClose,
    CurlyBracketOpen,
    CurlyBracketClose,
    SquareBracketOpen,
    SquareBracketClose,
    AngleBracketOpen,
    AngleBracketClose,
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
    KeywordUnderscore,
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

    pub fn get_one_child(&self, node_type: &NodeType) -> Option<Node> {
        for child in self.children.iter() {
            if child.node_type == *node_type {
                return Some(child.clone());
            }
        }
        None
    }

    pub fn has_descendant(&self, node_type: &NodeType) -> bool {
        self.get_one_descendant(node_type).is_some()
    }

    pub fn has_child(&self, node_type: &NodeType) -> bool {
        self.get_one_child(node_type).is_some()
    }

    pub fn starts_with(&self, node_type: &NodeType) -> bool {
        if self.node_type == *node_type {
            return true;
        }
        if let Some(n) = self.children.first() {
            n.starts_with(node_type)
        } else {
            false
        }
    }

    pub fn is_first_child(&self, node_type: &NodeType) -> bool {
        if let Some(child) = self.children.first() {
            child.node_type == *node_type
        } else {
            false
        }
    }

    pub fn is_last_child(&self, node_type: &NodeType) -> bool {
        if let Some(child) = self.children.last() {
            child.node_type == *node_type
        } else {
            false
        }
    }

    pub fn is_surrounded_by(
        &self,
        node_type_pre: &NodeType,
        node_type_post: &NodeType,
        decend: bool,
    ) -> bool {
        if self.children.len() == 1 && decend {
            self.children
                .first()
                .unwrap()
                .is_surrounded_by(node_type_pre, node_type_post, decend)
        } else {
            self.is_first_child(node_type_pre) && self.is_last_child(node_type_post)
        }
    }

    pub fn is_parenthesized(&self, decend: bool) -> bool {
        self.is_surrounded_by(
            &NodeType::RoundBracketOpen,
            &NodeType::RoundBracketClose,
            decend,
        ) || self.is_surrounded_by(
            &NodeType::CurlyBracketOpen,
            &NodeType::CurlyBracketClose,
            decend,
        ) || self.is_surrounded_by(
            &NodeType::SquareBracketOpen,
            &NodeType::SquareBracketClose,
            decend,
        )
    }

    pub fn children_without_outer(&self) -> Vec<Node> {
        let mut children = self.children.clone();
        if children.len() > 0 {
            children.remove(0);
        }
        if children.len() > 0 {
            children.remove(children.len() - 1);
        }
        children
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
    fn test_is_parenthesized() {
        let nodes = match parse_with("{id=null}", Rule::ExpPost) {
            Ok(nodes) => nodes,
            Err(err) => {
                println!("Parsing error:\n{}", err);
                panic!("Couldn't parse");
            }
        };
        let node = nodes.first().expect("schould contain node, but was empty");
        println!("----- success ------\n{:?}\n----- end -----", node);
        assert_eq!(node.is_parenthesized(true), true);
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
        expect_parse!("(this)", Rule::ExpPost, NodeType::Id);
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

    #[test]
    fn test_exponential_recursion_issue() {
        // encrypted_notes_motoko/test/test.mo took forever to parse (23 minutes)

        // $ dprint fmt
        // WARNING: Formatting is slow for /tmp/motoko/examples/motoko/encrypted-notes-dapp/src/encrypted_notes_motoko/test/test.mo
        //
        // real	22m25.692s
        // user	22m28.057s
        // sys	0m0.088s

        expect_parse!(
            r#"
            import Debug "mo:base/Debug";

            import Option "mo:base/Option";
            import Iter "mo:base/Iter";
            import Array "mo:base/Array";
            import List "mo:base/List";
            import Text "mo:base/Text";
            import Principal "mo:base/Principal";
            
            import M "mo:matchers/Matchers";
            import T "mo:matchers/Testable";
            import Suite "mo:matchers/Suite";
            import HM "mo:matchers/matchers/Hashmap";
            
            import En "../types";
            import UserStore "../user_store";
            "#,
            Rule::Motoko,
            NodeType::Import
        );
        expect_parse!(
            r#"
            // Custom [TestableItem] for serialized [UserStore] entries.
            // See https://github.com/kritzcreek/motoko-matchers/blob/master/src/Testable.mo
            func tuple4(ta: Principal, tb: En.PublicKey, tc: En.DeviceAlias, td: ?En.Ciphertext): 
                T.TestableItem<UserStore.StableUserStoreEntry> = {
                
                item = (ta, tb, tc, td);

                display = func ((a, b, c, d): UserStore.StableUserStoreEntry): Text =
                    "(" # Principal.toText(a) # ", " # b # ", " # c # ", " # Option.get(d, "<none>") # ")";

                equals = func ((a1, b1, c1, d1): UserStore.StableUserStoreEntry, 
                            (a2, b2, c2, d2): UserStore.StableUserStoreEntry): Bool =
                    Principal.equal(a1, a2) and 
                    Text.equal(b1, b2) and 
                    Text.equal(c1, c2) and 
                    d1 == d2;
            };
            "#,
            Rule::Motoko,
            NodeType::FuncBody
        );
        expect_parse!(
            r#"
            // Custom [TestableItem] for values of type [Principal].
            // See https://github.com/kritzcreek/motoko-matchers/blob/master/src/Testable.mo
            func princ(p: Principal): T.TestableItem<Principal> = {
                item = p;

                display = Principal.toText;

                equals = Principal.equal;
            };
            "#,
            Rule::Motoko,
            NodeType::FuncBody
        );
        expect_parse!(
            r#"
            func user_store(us: UserStore.UserStore): T.TestableItem<UserStore.UserStore> = {
                item = us;
            
                display = func (us: UserStore.UserStore): Text = 
                    "UserStore(principal = " # Principal.toText(us.get_principal()) # ") {\n"
                    # "    device_list =     " # Array.foldLeft<(En.DeviceAlias, En.PublicKey), Text>(
                        Iter.toArray(us.device_list.entries()),
                        "",
                        func (buf: Text, (alias, pk): (En.DeviceAlias, En.PublicKey)): Text = buf # "  (" # alias # " -> " # pk # ")") 
                    # ";\n"
                    # "    ciphertext_list = " # Array.foldLeft<(En.PublicKey, En.Ciphertext), Text>(
                        Iter.toArray(us.ciphertext_list.entries()),
                        "",
                        func (buf: Text, (pk, ct): (En.PublicKey, En.Ciphertext)): Text = buf # "  (" # pk # " -> " # ct # ")")
                    # ";\n"
                    # "}";
           
                equals = func (us1: UserStore.UserStore, us2: UserStore.UserStore): Bool {
                    let s1: List.List<UserStore.StableUserStoreEntry> = List.fromArray(us1.serialize());
                    let s2: List.List<UserStore.StableUserStoreEntry> = List.fromArray(us2.serialize());
                    (List.size(s1) == List.size(s2))
                    and List.all(
                        List.zip(s1, s2), 
                        func ((a, b): (UserStore.StableUserStoreEntry, UserStore.StableUserStoreEntry)): Bool {
                            a.0 == b.0 and  // Principal
                            a.1 == b.1 and  // En.PublicKey
                            a.2 == b.2 and  // En.DeviceAlias
                            a.3 == b.3      // ?En.Ciphertext
                        })
                }
            };
            "#,
            Rule::Motoko,
            NodeType::FuncBody
        );
        expect_parse!(
            r#"
            func PopulateUserStore(
                principal: Principal, 
                dev_keys: [En.PublicKey], 
                dev_aliases: [En.DeviceAlias], 
                ciphertexts: [?En.Ciphertext],
                permutation: [Nat]): UserStore.UserStore {
            
                let store = UserStore.UserStore(principal, 10);
                for (i in Iter.fromArray(permutation)) {
                    let key = dev_keys[i];
                    let alias = dev_aliases[i];
                    store.device_list.put(alias, key);
                    switch (ciphertexts[i]) {
                        case (null) {};
                        case (?ciphertext) {
                            store.ciphertext_list.put(key, ciphertext);
                        };
                    };
                };
                store
            };
            
            func C(t: Text): ?Text = Option.make(t);
            
            let USER_1 = Principal.fromText("2vxsx-fae");
            let USER_1_dev_keys    = [ "A1", "B1", "C1", "D1" ];
            let USER_1_dev_aliases = [ "a1", "b1", "c1", "d1" ];
            let USER_1_ciphertexts = [ C("Ax"), C("Bx"), C("Cx"), C("Dx") ];
            let USER_1_permuts = [
                [0, 1, 2, 3],  // default order
                [3, 2, 1, 0],  // test permutation 1
                [2, 0, 3, 1],  // test permutation 2
            ];
            func USER_1_store(permutation: [Nat]): UserStore.UserStore = 
                PopulateUserStore(
                    USER_1, 
                    USER_1_dev_keys, 
                    USER_1_dev_aliases, 
                    USER_1_ciphertexts, 
                    permutation);
            
            let USER_2 = Principal.fromText("2vxsx-fae");  // TODO: use a distinct principal for USER_2
            let USER_2_dev_keys    = [ "A2", "B2", "C2", "D2", "E2" ];
            let USER_2_dev_aliases = [ "a2", "b2", "c2", "d2", "e2" ];
            let USER_2_ciphertexts = [ null, null, C("Cy"), C("Dy"), null ];
            let USER_2_permuts = [
                [0, 1, 2, 3, 4],  // default order
                [4, 3, 2, 1, 0],  // test permutation 1
                [3, 0, 4, 2, 1],  // test permutation 2
            ];
            func USER_2_store(permutation: [Nat]): UserStore.UserStore = 
                PopulateUserStore(
                    USER_2, 
                    USER_2_dev_keys, 
                    USER_2_dev_aliases, 
                    USER_2_ciphertexts, 
                    permutation);            
            "#,
            Rule::Motoko,
            NodeType::FuncBody
        );

        expect_parse!(
            r#"
            M.array([]);
            "#,
            Rule::Motoko,
            NodeType::Declaration
        );
        expect_parse!(
            r#"
            Array.append(
                Array.map(USER_1_permuts, func (perm: [Nat]): Suite.Suite =
                    Suite.test(
                        "Serializing a user store with 4 fully-synced devices",
                        USER_1_store(perm).serialize(),
                        M.array([
                            M.equals(tuple4(USER_1, "A1", "a1", Option.make("Ax"))),
                            M.equals(tuple4(USER_1, "B1", "b1", Option.make("Bx"))),
                            M.equals(tuple4(USER_1, "C1", "c1", Option.make("Cx"))),
                            M.equals(tuple4(USER_1, "D1", "d1", Option.make("Dx"))),
                        ]))),
                Array.map(USER_2_permuts, func (perm: [Nat]): Suite.Suite =
                    Suite.test(
                        "Serializing a user store with 5 devices only 3 of which are synced",
                        USER_2_store(perm).serialize(),
                        M.array([
                            M.equals(tuple4(USER_2, "A2", "a2", null)),
                            M.equals(tuple4(USER_2, "B2", "b2", null)),
                            M.equals(tuple4(USER_2, "C2", "c2", Option.make("Cy"))),
                            M.equals(tuple4(USER_2, "D2", "d2", Option.make("Dy"))),
                            M.equals(tuple4(USER_2, "E2", "e2", null)),
                        ]))))
            "#,
            Rule::Motoko,
            NodeType::Type
        );
        expect_parse!(
            r#"
            Suite.run(
                Suite.suite("UserStore", [
                    Suite.suite("UserStore.serialize",
                        Array.append(
                            Array.map(USER_1_permuts, func (perm: [Nat]): Suite.Suite =
                                Suite.test(
                                    "Serializing a user store with 4 fully-synced devices",
                                    USER_1_store(perm).serialize(),
                                    M.array([
                                        M.equals(tuple4(USER_1, "A1", "a1", Option.make("Ax"))),
                                        M.equals(tuple4(USER_1, "B1", "b1", Option.make("Bx"))),
                                        M.equals(tuple4(USER_1, "C1", "c1", Option.make("Cx"))),
                                        M.equals(tuple4(USER_1, "D1", "d1", Option.make("Dx"))),
                                    ]))),
                            Array.map(USER_2_permuts, func (perm: [Nat]): Suite.Suite =
                                Suite.test(
                                    "Serializing a user store with 5 devices only 3 of which are synced",
                                    USER_2_store(perm).serialize(),
                                    M.array([
                                        M.equals(tuple4(USER_2, "A2", "a2", null)),
                                        M.equals(tuple4(USER_2, "B2", "b2", null)),
                                        M.equals(tuple4(USER_2, "C2", "c2", Option.make("Cy"))),
                                        M.equals(tuple4(USER_2, "D2", "d2", Option.make("Dy"))),
                                        M.equals(tuple4(USER_2, "E2", "e2", null)),
                                    ]))))),
                    Suite.suite("UserStore.deserialize",
                        Array.append(
                            [
                                Suite.test(
                                    "Smoke test",
                                    UserStore.deserialize(USER_1_store(USER_1_permuts[0]).serialize(), 10),
                                    M.allOf([
                                        HM.hasKey<Principal, UserStore.UserStore>(princ(USER_1)),
                                        M.not_(HM.atKey<Principal, UserStore.UserStore>(princ(USER_1), M.equals(user_store(USER_2_store(USER_2_permuts[0])))))
                                    ]))
                            ],
                            Array.append(
                                Array.map(USER_1_permuts, func (perm: [Nat]): Suite.Suite =
                                    Suite.test(
                                        "Deserialize a serialized user store with 4 fully-synced devices",
                                        UserStore.deserialize(USER_1_store(perm).serialize(), 10),
                                        M.allOf([
                                            HM.hasKey<Principal, UserStore.UserStore>(princ(USER_1)),
                                            HM.atKey<Principal, UserStore.UserStore>(princ(USER_1), M.equals(user_store(USER_1_store(perm))))
                                        ]))),
                                Array.map(USER_2_permuts, func (perm: [Nat]): Suite.Suite =
                                    Suite.test(
                                        "Deserialize a serialized user store with 5 devices only 3 of which are synced",
                                        UserStore.deserialize(USER_2_store(perm).serialize(), 10),
                                        M.allOf([
                                            HM.hasKey<Principal, UserStore.UserStore>(princ(USER_2)),
                                            HM.atKey<Principal, UserStore.UserStore>(princ(USER_2), M.equals(user_store(USER_2_store(perm))))
                                        ]))))))
                ]));
            "#,
            Rule::Motoko,
            NodeType::Type
        );
    }

    #[test]
    fn test_reusion_bug_types() {
        expect_parse!(
            "type T3  = ?(Nat,?(Nat,?(Int,T3)))",
            Rule::Declaration,
            NodeType::Type
        );

        expect_parse!(
            "type T7  = ?(Nat,?(Nat,?(Nat,?(Nat,?(Nat,?(Nat,?(Int,T7)))))))",
            Rule::Declaration,
            NodeType::Type
        );

        expect_parse!(
            "type T11 = (Nat,(Nat,(Nat,(Nat,(Nat,(Nat,(Nat,(Nat,(Nat,(Nat,(Int,T11)))))))))))",
            Rule::Declaration,
            NodeType::Type
        );

        expect_parse!(
            "type T11 = ?(Nat,?(Nat,?(Nat,?(Nat,?(Nat,?(Nat,?(Nat,?(Nat,?(Nat,?(Nat,?(Int,T11)))))))))))",
            Rule::Declaration,
            NodeType::Type
        );
    }

    #[test]
    fn test_exp_un() {
        expect_parse!("[var {#}]", Rule::TypeNullary, NodeType::TypeVariant);
        expect_parse!("[var] : [var {#}]", Rule::Motoko, NodeType::TypeVariant);

        expect_parse!(
            "let _ : [var {#} and {#a : Int}] = [var] : [var {#}];",
            Rule::Motoko,
            NodeType::TypeVariant
        );
    }

    #[test]
    fn test_exp_non_dec() {
        expect_parse!(
            "module { public func empty<K, V>() : Trie<K, V> { false }; }",
            Rule::Motoko,
            NodeType::KeywordFalse
        );

        expect_parse!("Trie<K, V>", Rule::Type, NodeType::Id);
        //TODO negative test: expect_parse_partial!("Trie<K, V> { #empty; }", Rule::Type, NodeType::Id);

        expect_parse!(
            "module { public func empty<K, V>() : Trie<K, V> { #empty; }; }",
            Rule::Declaration,
            NodeType::ExpNonDec
        );
    }

    #[test]
    fn test_parenthesis() {
        expect_parse!(
            "case (?(k3, v3)) { put(#empty, k3, k3_eq, v3).0 }",
            Rule::Case,
            NodeType::Nat
        );
        expect_parse!(
            "case (?(k3, v3)) { (put(#empty, k3, k3_eq, v3)).0 }",
            Rule::Case,
            NodeType::Nat
        );
    }

    #[test]
    fn test_bin_op() {
        expect_parse!("424242 : Nat64 == 1", Rule::Exp, NodeType::Lit);
        expect_parse!("\"a\" # \"b\"", Rule::Exp, NodeType::Text);
        expect_parse!("\"a\" # \"b\" # \"c\"", Rule::Exp, NodeType::Text);

        expect_parse!("\"a\" < \"b\"", Rule::Exp, NodeType::Text);
        expect_parse!("\"a\" # \"b\" # \"c\" < \"d\"", Rule::Exp, NodeType::Text);
        expect_parse!("\"a\" < \"b\" # \"c\" # \"d\"", Rule::Exp, NodeType::Text);
        expect_parse!("\"a\" > \"b\" # \"c\" # \"d\"", Rule::Exp, NodeType::Text);
        expect_parse!("\"a\" # \"b\" # \"c\" > \"d\"", Rule::Exp, NodeType::Text);
    }

    #[test]
    fn test_float() {
        expect_parse!("0x644.", Rule::Lit, NodeType::Lit);
    }
}
