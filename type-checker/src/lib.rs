mod cst {
    pub enum TokenKind {
        LowerIdent,
        Keyword,
        Punctuation,
    }

    #[derive(PartialEq, Eq)]
    pub enum NodeKind {
        ExprLet,
        ExprUnit,
    }

    pub struct Token<'a> {
        pub kind: TokenKind,
        pub value: &'a str,
    }

    pub struct Node<'a> {
        pub kind: NodeKind,
        pub children: Vec<Child<'a>>,
    }

    pub enum Child<'a> {
        Token(Token<'a>),
        Node(Node<'a>),
    }
}

struct EVar<'a>(&'a str);

enum Type {
    Unit,
}

struct Let<'a> {
    node: &'a cst::Node<'a>,
    binder: EVar<'a>,
    bindee: &'a cst::Node<'a>,
    body: &'a cst::Node<'a>,
}

impl<'a> From<Let<'a>> for &'a cst::Node<'a> {
    fn from(value: Let<'a>) -> Self {
        value.node
    }
}

impl<'a> TryFrom<&'a cst::Node<'a>> for Let<'a> {
    type Error = ();

    fn try_from(node: &'a cst::Node<'a>) -> std::result::Result<Self, Self::Error> {
        if node.kind != cst::NodeKind::ExprLet {
            return Err(());
        }
        if let [cst::Child::Token(cst::Token {
            kind: cst::TokenKind::Keyword,
            value: "let",
        }), cst::Child::Token(cst::Token {
            kind: cst::TokenKind::LowerIdent,
            value: binder,
        }), cst::Child::Token(cst::Token {
            kind: cst::TokenKind::Punctuation,
            value: "=",
        }), cst::Child::Node(bindee), cst::Child::Token(cst::Token {
            kind: cst::TokenKind::Keyword,
            value: "in",
        }), cst::Child::Node(body)] = &node.children[..]
        {
            Ok(Let {
                node,
                binder: EVar(binder),
                bindee,
                body,
            })
        } else {
            Err(())
        }
    }
}

trait Pattern<'a>: Into<&'a cst::Node<'a>> + TryFrom<&'a cst::Node<'a>> {
    const KIND: cst::NodeKind;
}

impl<'a> Pattern<'a> for Let<'a> {
    const KIND: cst::NodeKind = cst::NodeKind::ExprLet;
}

struct Context;

impl Context {
    fn extend_evar(&self, _evar: EVar, _typ: Type) -> Self {
        todo!()
    }
}

enum TypeError {}

type Result<T> = std::result::Result<T, TypeError>;

#[derive(Clone, Copy)]
enum CheckMode {
    Bidi,
}

#[derive(Clone, Copy)]
enum InferMode {
    Simple,
    Bidi,
}

trait Checker {
    fn lookup_evar(&self, ctx: &Context, evar: EVar) -> Result<Type>;
    fn check(&self, ctx: &Context, mode: CheckMode, node: &cst::Node, typ: Type) -> Result<()>;
    fn infer(&self, ctx: &Context, mode: InferMode, node: &cst::Node) -> Result<Type>;
    fn equal(&self, found: Type, expected: Type) -> Result<()>;
}

trait InferRule {
    type Pattern<'a>: Pattern<'a>;

    const NAME: &'static str;
    const MODE: InferMode;

    fn infer<'a>(checker: &dyn Checker, ctx: &Context, pattern: Self::Pattern<'a>) -> Result<Type>;
}

mod rules {
    use super::*;

    pub enum TLet {}

    impl InferRule for TLet {
        type Pattern<'a> = Let<'a>;

        const NAME: &'static str = "T-Let";

        const MODE: InferMode = InferMode::Simple;

        fn infer<'a>(checker: &dyn Checker, ctx: &Context, let_: Let<'a>) -> Result<Type> {
            let t1 = checker.infer(ctx, Self::MODE, let_.bindee)?;
            let ctx1 = ctx.extend_evar(let_.binder, t1);
            checker.infer(&ctx1, Self::MODE, let_.body)
        }
    }
}

struct RulesDB {
    infer_rules: Vec<(
        cst::NodeKind,
        Box<dyn Fn(&dyn Checker, &Context, &cst::Node) -> Option<Result<Type>>>,
    )>,
}

impl RulesDB {
    fn new() -> Self {
        Self {
            infer_rules: Vec::new(),
        }
    }
    fn add_infer_rule<Rule: InferRule>(&mut self) {
        let kind = Rule::Pattern::KIND;
        let rule = |checker: &dyn Checker, ctx: &Context, node: &cst::Node| {
            match node.try_into() {
                Err(_) => None,
                Ok(pattern) => Some(Rule::infer(checker, ctx, pattern))
            }
         };
        self.infer_rules.push((kind, Box::new(rule)));
    }
}

pub fn test() {
    let mut db = RulesDB::new();
    db.add_infer_rule::<rules::TLet>();
}

/* Haskell Prototype:

{-# LANGUAGE AllowAmbiguousTypes, TypeFamilies #-}

import Data.Typeable
import qualified Data.Kind as Kind

data Type = TUnit
  deriving (Show)

data NodeKind = NKUnit | NKZero
  deriving (Eq)

data CST = CST{ kind :: NodeKind }

class Typeable n => IsASTNode n where
  nodeKind :: n -> NodeKind
  fromCST :: CST -> Maybe n

data ExprUnit = ExprUnit
  deriving (Typeable)

instance IsASTNode ExprUnit where
  nodeKind _ = NKUnit
  fromCST c = if kind c == NKUnit then Just ExprUnit else Nothing

data ExprZero = ExprZero
  deriving (Typeable)

instance IsASTNode ExprZero where
  nodeKind _ = NKZero
  fromCST c = if kind c == NKZero then Just ExprZero else Nothing


data TypeError = NoRuleFound | ZeroNotAllowed
  deriving (Show)

class Typeable m => IsRuleMode m where
  type Input m
  type Output m

data AlwaysInfer

data BidiInfer

data BidiCheck

instance IsRuleMode AlwaysInfer where
  type Input AlwaysInfer = ()
  type Output AlwaysInfer = Type

instance IsRuleMode BidiInfer where
  type Input BidiInfer = ()
  type Output BidiInfer = Type

instance IsRuleMode BidiCheck where
  type Input BidiCheck = Type
  type Output BidiCheck = ()

class (IsRuleMode (RuleMode r), IsASTNode (ASTNode r)) => IsRule r where
  type RuleMode r
  type ASTNode r

  runRule :: r -> ASTNode r -> Input (RuleMode r) -> Either TypeError (Output (RuleMode r))

data RuleTUnit

instance IsRule RuleTUnit where
  type RuleMode RuleTUnit = AlwaysInfer
  type ASTNode RuleTUnit = ExprUnit

  runRule _ ExprUnit () = Right TUnit

data RuleTZero

instance IsRule RuleTZero where
  type RuleMode RuleTZero = AlwaysInfer
  type ASTNode RuleTZero = ExprZero

  runRule _ ExprZero () = Left ZeroNotAllowed


data AnyRule where
  AnyRule :: IsRule r => r -> AnyRule

rules :: [AnyRule]
rules =
  [ AnyRule (undefined :: RuleTUnit)
  , AnyRule (undefined :: RuleTZero)
  ]

run :: forall (m :: Kind.Type). IsRuleMode m => CST -> Input m -> Either TypeError (Output m)
run cst input = go rules
  where
    go [] = Left NoRuleFound
    go (AnyRule r:rs) = case matchRuleMode @m r of
      Nothing -> go rs
      Just Refl -> case tryRule r of
        Nothing -> go rs
        Just res -> res

    tryRule :: forall r. (IsRule r, RuleMode r ~ m) => r -> Maybe (Either TypeError (Output (RuleMode r)))
    tryRule r
      | kind cst /= nodeKind (undefined :: ASTNode r) = Nothing
      | otherwise = case fromCST cst :: Maybe (ASTNode r) of
        Nothing -> Nothing
        Just node -> Just (runRule r node input)

    matchRuleMode :: forall m r. (IsRuleMode m, IsRule r) => r -> Maybe (m :~: RuleMode r)
    matchRuleMode _ = eqT @m @(RuleMode r)


main :: IO ()
main = print $ run @AlwaysInfer (CST NKUnit) ()
*/
