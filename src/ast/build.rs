use crate::ast::Pos;
use crate::ast::nodes::*;
use crate::parser::CNode;
use crate::parser::elements;


fn pos(node: &CNode) -> Pos {
    Pos { source: node.code, start: node.left(), length: node.right() - node.left() }
}

pub fn build_program(node: &CNode) -> AProgram {
    AProgram {
        stmts: build_stmts(node.at(0))
    }
}

pub fn build_expr(node: &CNode) -> Box<AExpr> {
    let child = node.at(0);
    match child.element {
        &elements::productions::STRUCTURE  => build_structure(child),
        &elements::expressions::LET        => build_var(child),
        &elements::expressions::LITERAL    => build_literal(child),
        &elements::expressions::CHAIN      => build_chain(child),
        &elements::expressions::SEQUENCE   => build_sequence(child),
        &elements::expressions::PREOP      => build_preop(child),
        &elements::expressions::BINOP      => build_binop(child),
        &elements::expressions::JUMP       => build_jump(child),
        &elements::expressions::ASSIGNMENT => build_assign(child),
        _ => panic!("{}", node.element.name),
    }
}

fn build_structure(node: &CNode) -> Box<AExpr> {
    let child = node.at(0);
    Box::new(match child.element {
        &elements::structures::BLOCK => AExpr::Block(build_block(child)),
        &elements::structures::IF    => AExpr::If(build_if(child)),
        &elements::structures::LOOP  => AExpr::Loop(build_loop(child)),
        &elements::structures::WHILE => AExpr::While(build_while(child)),
        &elements::structures::FOR   => AExpr::For(build_for(child)),
        _ => panic!(),
    })
}

fn build_stmts(node: &CNode) -> Box<[AStmt]> {
    node.children().iter()
        .map(|child| build_stmt(child))
        .collect()
}

fn build_stmt(node: &CNode) -> AStmt {
    let child = node.at(0);
    match child.element {
        &elements::productions::DEFINITION => AStmt::Def(build_def(child)),
        &elements::productions::STRUCTURE  => AStmt::Expr(build_structure(child)),
        &elements::productions::EXPRESSION => AStmt::Expr(build_expr(child)),
        _ => panic!(),
    }
}

fn build_def(node: &CNode) -> ADef {
    let child = node.at(0);
    match child.element {
        &elements::definitions::CLASS    => ADef::Class(build_class(child)),
        &elements::definitions::FUNCTION => ADef::Function(build_function(child)),
        _ => panic!(),
    }
}

fn build_class(node: &CNode) -> AClass {
    AClass {
        pos: pos(node),
        name: node.at(1).string(),
        generics: build_generics(node.at(2)),
        parent: build_option_type(node.at(3)),
        methods: node.at(5).children().iter()
            .map(|child| build_function(child))
            .collect(),
    }
}

fn build_generics(node: &CNode) -> Box<[AGeneric]> {
    node.children().get(1).iter().flat_map(|child| child.children().iter()
        .step_by(2)
        .map(|child| build_generic(child))
        ).collect()
}

fn build_generic(node: &CNode) -> AGeneric {
    AGeneric {
        name: node.string(),
        constraint: None,
    }
}

fn build_function(node: &CNode) -> AFunction {
    AFunction {
        pos: pos(node),
        name: node.at(1).string(),
        generics: build_generics(node.at(2)),
        params: build_params(node.at(3)),
        ret: build_option_type(node.at(4)),
        body: build_block(node.at(5)),
    }
}

fn build_params(node: &CNode) -> Box<[AParameter]> {
    node.at(1).children().iter()
        .step_by(2)
        .map(|child| build_param(child))
        .collect()
}

fn build_param(node: &CNode) -> AParameter {
    AParameter {
        name: node.at(0).string(),
        r#type: build_option_type(node.at(1)),
    }
}

fn build_option_type(node: &CNode) -> Option<Box<AExpr>> {
    node.children().get(1).map(|child| build_expr(child))
}

fn build_block(node: &CNode) -> ABlock {
    ABlock {
        pos: pos(node),
        stmts: build_stmts(node.at(1)),
        expr: (node.children().len() == 4).then(|| build_expr(node.at(2))),
    }
}

fn build_if(node: &CNode) -> AIf {
    AIf {
        pos: pos(node),
        cond: build_expr(node.at(1)),
        then: build_block(node.at(2)),
        r#else: node.children().get(4).map(|child| build_block(child))
    }
}

fn build_loop(node: &CNode) -> ALoop {
    ALoop {
        pos: pos(node),
        body: build_block(node.at(1)),
    }
}

fn build_while(node: &CNode) -> AWhile {
    AWhile {
        pos: pos(node),
        cond: build_expr(node.at(1)),
        body: build_block(node.at(2)),
    }
}

fn build_for(node: &CNode) -> AFor {
    AFor {
        pos: pos(node),
        element: node.at(1).string(),
        list: build_expr(node.at(3)),
        body: build_block(node.at(4)),
    }
}

fn build_var(node: &CNode) -> Box<AExpr> {
    Box::new(AExpr::Var(AExprVar {
        pos: pos(node),
        ident: node.at(1).at(0).string(),
        r#type: build_option_type(node.at(1).at(1)),
    }))
}

fn build_literal(node: &CNode) -> Box<AExpr> {
    let child = node.at(0);
    let pos = pos(node);
    Box::new(match child.element {
        &elements::keywords::VOID       => AExpr::Void(AExprVoid { pos }),
        &elements::keywords::TRUE       => AExpr::Bool(AExprBool { pos, bool: true }),
        &elements::keywords::FALSE      => AExpr::Bool(AExprBool { pos, bool: false }),
        &elements::literals::INTEGER    => AExpr::Int(AExprInt { pos, literal: child.string() }),
        &elements::literals::FLOAT      => AExpr::Float(AExprFloat { pos, literal: child.string() }),
        &elements::literals::STRING     => AExpr::String(AExprString { pos, literal: child.string() }),
        &elements::literals::IDENTIFIER => AExpr::Ident(AExprIdent { pos, ident: child.string() }),
        _ => panic!(),
    })
}

fn build_chain(node: &CNode) -> Box<AExpr> {
    Box::new(AExpr::Chain(AExprChain {
        pos: pos(node),
        expr: build_expr(node.at(0)),
        member: node.at(2).string(),
    }))
}

fn build_sequence(node: &CNode) -> Box<AExpr> {
    let expr = build_expr(node.at(0));
    let pos = pos(node);
    let args = node.at(2).children().iter()
        .step_by(2)
        .map(|child| build_expr(child))
        .collect::<Box<[_]>>();

    match node.at(1).text().as_ref() {
        "[" => Box::new(AExpr::Apply(AExprApply {
            pos,
            expr,
            args,
        })),
        "(" => Box::new(AExpr::Call(AExprCall {
            pos,
            expr,
            args,
        })),
        _ => unreachable!(),
    }
}

fn build_preop(node: &CNode) -> Box<AExpr> {
    Box::new(AExpr::Preop(AExprPreop {
        pos: pos(node),
        op: node.at(0).string(),
        expr: build_expr(node.at(1)),
    }))
}

fn build_binop(node: &CNode) -> Box<AExpr> {
    let pos = pos(node);
    let op = node.at(1).string();
    match op.as_ref() {
        "||" => Box::new(AExpr::Or(AExprOr {
            pos,
            left: build_expr(node.at(0)),
            right: build_expr(node.at(2)),
        })),
        "&&" => Box::new(AExpr::And(AExprAnd {
            pos,
            left: build_expr(node.at(0)),
            right: build_expr(node.at(2)),
        })),
        _ => Box::new(AExpr::Binop(AExprBinop {
            pos,
            op,
            left: build_expr(node.at(0)),
            right: build_expr(node.at(2)),
        }))
    }
}

fn build_jump(node: &CNode) -> Box<AExpr> {
    let child = node.at(0);
    let pos = pos(node);
    let expr = node.children().get(1).map(|child| build_expr(child));
    Box::new(match child.element {
        &elements::keywords::CONTINUE => AExpr::Continue(AExprContinue { pos, expr }),
        &elements::keywords::BREAK    => AExpr::Break(AExprBreak { pos, expr }),
        &elements::keywords::RETURN   => AExpr::Return(AExprReturn { pos, expr }),
        _ => todo!(),
    })
}

fn build_assign(node: &CNode) -> Box<AExpr> {
    let left = build_expr(node.at(0));
    let right = build_expr(node.at(2));
    Box::new(AExpr::Assign(AExprAssign { pos: pos(node), left, right }))
}
