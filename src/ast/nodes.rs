use crate::ast::Pos;

pub struct AProgram {
    pub stmts: Box<[AStmt]>,
}

pub enum ADef {
    Class(AClass),
    Function(AFunction),
}

pub struct AClass {
    pub pos: Pos,
    pub name: Box<str>,
    pub parent: Option<Box<AExpr>>,
    pub generics: Box<[AGeneric]>,
    pub methods: Box<[AFunction]>,
}

pub struct AFunction {
    pub pos: Pos,
    pub name: Box<str>,
    pub generics: Box<[AGeneric]>,
    pub params: Box<[AParameter]>,
    pub rest: Option<AParameter>,
    pub ret: Option<Box<AExpr>>,
    pub body: ABlock,
}

pub struct AGeneric {
    pub name: Box<str>,
    pub constraint: Option<Box<AExpr>>,
}

pub enum AMember {
    Attribute(AAttribute),
    Method(AFunction),
}

pub struct AAttribute {
    name: Box<str>,
    r#type: Option<AExpr>,
}

pub struct AParameter {
    pub name: Box<str>,
    pub r#type: Option<Box<AExpr>>,
}

pub enum AStmt {
    Expr(Box<AExpr>),
    Def(ADef),
}

pub struct ABlock {
    pub pos: Pos,
    pub stmts: Box<[AStmt]>,
    pub expr: Option<Box<AExpr>>,
}

pub struct AIf {
    pub pos: Pos,
    pub cond: Box<AExpr>,
    pub then: ABlock,
    pub r#else: Option<ABlock>,
}

pub struct ALoop {
    pub pos: Pos,
    pub body: ABlock,
}

pub struct AWhile {
    pub pos: Pos,
    pub cond: Box<AExpr>,
    pub body: ABlock,
}

pub struct AFor {
    pub pos: Pos,
    pub element: Box<str>,
    pub list: Box<AExpr>,
    pub body: ABlock,
}

pub enum AExpr {
    Void(AExprVoid),
    Bool(AExprBool),
    Int(AExprInt),
    Float(AExprFloat),
    String(AExprString),
    Ident(AExprIdent),
    Var(AExprVar),
    Chain(AExprChain),
    Apply(AExprApply),
    Call(AExprCall),
    Preop(AExprPreop),
    Binop(AExprBinop),
    Or(AExprOr),
    And(AExprAnd),
    Continue(AExprContinue),
    Break(AExprBreak),
    Return(AExprReturn),
    Block(ABlock),
    If(AIf),
    Loop(ALoop),
    While(AWhile),
    For(AFor),
    Assign(AExprAssign),
}

pub struct AExprVoid {
    pub pos: Pos,
}

pub struct AExprBool {
    pub pos: Pos,
    pub bool: bool,
}

pub struct AExprInt {
    pub pos: Pos,
    pub literal: Box<str>,
}

pub struct AExprFloat {
    pub pos: Pos,
    pub literal: Box<str>,
}

pub struct AExprString {
    pub pos: Pos,
    pub literal: Box<str>,
}

pub struct AExprIdent {
    pub pos: Pos,
    pub ident: Box<str>,
}

pub struct AExprVar {
    pub pos: Pos,
    pub ident: Box<str>,
    pub r#type: Option<Box<AExpr>>,
}

pub struct AExprChain {
    pub pos: Pos,
    pub expr: Box<AExpr>,
    pub member: Box<str>,
}

pub struct AExprApply {
    pub pos: Pos,
    pub expr: Box<AExpr>,
    pub args: Box<[Box<AExpr>]>,
}

pub struct AExprCall {
    pub pos: Pos,
    pub expr: Box<AExpr>,
    pub args: Box<[Box<AExpr>]>,
}

pub struct AExprPreop {
    pub pos: Pos,
    pub op: Box<str>,
    pub expr: Box<AExpr>,
}

pub struct AExprBinop {
    pub pos: Pos,
    pub op: Box<str>,
    pub left: Box<AExpr>,
    pub right: Box<AExpr>,
}

pub struct AExprOr {
    pub pos: Pos,
    pub left: Box<AExpr>,
    pub right: Box<AExpr>,
}

pub struct AExprAnd {
    pub pos: Pos,
    pub left: Box<AExpr>,
    pub right: Box<AExpr>,
}

pub struct AExprContinue {
    pub pos: Pos,
    pub expr: Option<Box<AExpr>>,
}

pub struct AExprBreak {
    pub pos: Pos,
    pub expr: Option<Box<AExpr>>,
}

pub struct AExprReturn {
    pub pos: Pos,
    pub expr: Option<Box<AExpr>>,
}

pub struct AExprAssign {
    pub pos: Pos,
    pub left: Box<AExpr>,
    pub right: Box<AExpr>,
}

pub enum ARef {
    Var(AVar),
    Expr(Box<AExpr>),
}

pub struct AVar {
    pub name: Box<str>,
    pub r#type: Option<AExpr>,
}
