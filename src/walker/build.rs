use crate::memory::Ref;
use crate::parser::SNode;
use crate::parser::elements;
use crate::walker::{ ANode, WNode };
use crate::walker::nodes::*;

pub fn program(node: Ref<SNode>) -> WNode {
    WNode::new(node, Program::new(statements(node.front(0))))
}

fn statements(node: Ref<SNode>) -> WNode {
    WNode::new(node, Statements::new(node.children().iter()
        .map(|child| statement(Ref::new(child)))
        .collect()
    ))
}

fn statement(node: Ref<SNode>) -> WNode {
    let child = node.front(0);
    WNode::new(node, Statement::new(match *child.element {
        elements::structures::STRUCTURE   => structure(child),
        elements::controls::CONTROL       => control(child),
        elements::expressions::EXPRESSION => expression(child),
        _ => panic!(),
    }))
}

pub fn expression(node: Ref<SNode>) -> WNode {
    let child = node.front(0);
    match *child.element {
        elements::controls::CONTROL       => control(child),
        elements::jumps::JUMP             => jump(child),
        elements::expressions::LET        => r#let(child),
        elements::expressions::LITERAL    => literal(child),
        elements::expressions::CHAIN      => chain(child),
        elements::expressions::SEQUENCE   => sequence(child),
        elements::expressions::BINOP      => binop(child),
        elements::expressions::PREOP      => preop(child),
        elements::expressions::ASSIGNMENT => assignment(child),
        _ => panic!(),
    }
}

fn r#type(node: Ref<SNode>) -> Option<WNode> {
    node.children().get(1).map(|child| expression(Ref::new(child)))
}

fn name(node: Ref<SNode>) -> Option<Ref<str>> {
    node.children().get(0).map(|child| token(Ref::new(child)))
}

fn literal(node: Ref<SNode>) -> WNode {
    let child = node.front(0);
    WNode::new(node, ALiteral::new(match *child.element {
        elements::keywords::TRUE        => Box::new(r#true(child)),
        elements::keywords::FALSE       => Box::new(r#false(child)),
        elements::variables::INTEGER    => Box::new(integer(child)),
        elements::variables::FLOAT      => Box::new(float(child)),
        elements::variables::STRING     => Box::new(string(child)),
        elements::variables::IDENTIFIER => Box::new(identifier(child)),
        _ => panic!(),
    }))
}

fn r#true(node: Ref<SNode>) -> ANode<ATrue> {
    ANode::new(node, ATrue::new())
}

fn r#false(node: Ref<SNode>) -> ANode<AFalse> {
    ANode::new(node, AFalse::new())
}

fn integer(node: Ref<SNode>) -> ANode<AInteger> {
    ANode::new(node, AInteger::new(node.text()))
}

fn float(node: Ref<SNode>) -> ANode<AFloat> {
    ANode::new(node, AFloat::new(node.text()))
}

fn string(node: Ref<SNode>) -> ANode<AString> {
    ANode::new(node, AString::new(node.text()))
}

fn identifier(node: Ref<SNode>) -> ANode<AIdentifier> {
    ANode::new(node, AIdentifier::new(node.text()))
}

fn structure(node: Ref<SNode>) -> WNode {
    let child = node.front(0);
    WNode::new(node, AStructure::new(match *child.element {
        elements::structures::CLASS    => class(child),
        elements::structures::FUNCTION => function(child),
        _ => panic!(),
    }))
}

fn control(node: Ref<SNode>) -> WNode {
    let child = node.front(0);
    WNode::new(node, AControl::new(match *child.element {
        elements::controls::BLOCK => Box::new(block(child)),
        elements::controls::IF    => Box::new(r#if(child)),
        elements::controls::LOOP  => Box::new(r#loop(child)),
        elements::controls::WHILE => Box::new(r#while(child)),
        elements::controls::FOR   => Box::new(r#for(child)),
        _ => panic!(),
    }))
}

fn block(node: Ref<SNode>) -> ANode<ABlock> {
    ANode::new(node, ABlock::new(statements(node.front(1)), (node.children().len() == 4).then(|| expression(node.front(2)))))
}

fn r#if(node: Ref<SNode>) -> ANode<AIf> {
    ANode::new(node, AIf::new(expression(node.front(1)), block(node.front(2)), node.children().get(4).map(|child| block(Ref::new(child)))))
}

fn r#loop(node: Ref<SNode>) -> ANode<ALoop> {
    ANode::new(node, ALoop::new(block(node.front(1))))
}

fn r#while(node: Ref<SNode>) -> ANode<AWhile> {
    ANode::new(node, AWhile::new(expression(node.front(1)), block(node.front(2))))
}

fn r#for(node: Ref<SNode>) -> ANode<AFor> {
    ANode::new(node, AFor::new(token(node.front(1)), expression(node.front(3)), block(node.front(4))))
}

fn r#let(node: Ref<SNode>) -> WNode {
    declaration(node.front(1))
}

fn declaration(node: Ref<SNode>) -> WNode {
    WNode::new(node, Declaration::new(token(node.front(0)), r#type(node.front(1))))
}

fn jump(node: Ref<SNode>) -> WNode {
    let child = node.front(0);
    WNode::new(node, AJump::new(match *child.element {
        elements::jumps::CONTINUE => Box::new(r#continue(child)),
        elements::jumps::BREAK    => Box::new(r#break(child)),
        elements::jumps::RETURN   => Box::new(r#return(child)),
        _ => panic!(),
    }))
}

fn r#continue(node: Ref<SNode>) -> ANode<AContinue> {
    ANode::new(node, AContinue::new(node.children().get(1).map(|child| expression(Ref::new(child)))))
}

fn r#break(node: Ref<SNode>) -> ANode<ABreak> {
    ANode::new(node, ABreak::new(node.children().get(1).map(|child| expression(Ref::new(child)))))
}

fn r#return(node: Ref<SNode>) -> ANode<AReturn> {
    ANode::new(node, AReturn::new(node.children().get(1).map(|child| expression(Ref::new(child)))))
}

fn generics(node: Ref<SNode>) -> Box<[Ref<str>]> {
    node.front(1).children().iter()
        .step_by(2)
        .map(|child| token(Ref::new(child)))
        .collect()
}

fn class(node: Ref<SNode>) -> Box<ANode<dyn AStructureTrait>> {
    let name = Some(token(node.front(1)));
    let class = Box::new(ANode::new(node, AClass::new(name, r#type(node.back(4)), methods(node.back(2)))));
    if node.children().len() >= 7 {
        Box::new(ANode::new(node, AGeneric::new(name, generics(node.front(2)), class)))
    } else {
        class
    }
}

fn methods(node: Ref<SNode>) -> Box<[Box<ANode<dyn AStructureTrait>>]> {
    node.children().iter()
        .map(|child| function(Ref::new(child)))
        .collect()
}

fn function(node: Ref<SNode>) -> Box<ANode<dyn AStructureTrait>> {
    let name = Some(token(node.front(1)));
    let function = Box::new(ANode::new(node, AFunction::new(name, parameters(node.back(3)), r#type(node.back(2)), block(node.back(1)))));
    if node.children().len() >= 6 {
        Box::new(ANode::new(node, AGeneric::new(name, generics(node.front(2)), function)))
    } else {
        function
    }
}

fn rest(node: Ref<SNode>) -> Option<(Ref<str>, Option<WNode>)> {
    node.children().get(1).map(|child| parameter(Ref::new(child)))
}

fn parameters(node: Ref<SNode>) -> (Box<[(Ref<str>, Option<WNode>)]>, Option<(Ref<str>, Option<WNode>)>) {
    let parameters = node.front(1).children().iter()
        .step_by(2)
        .map(|child| parameter(Ref::new(child)))
        .collect();

    (parameters, rest(node.back(2)))
}

fn parameter(node: Ref<SNode>) -> (Ref<str>, Option<WNode>) {
    (token(node.front(0)), r#type(node.front(1)))
}

fn chain(node: Ref<SNode>) -> WNode {
    WNode::new(node, Chain::new(expression(node.front(0)), token(node.front(2))))
}

fn sequence(node: Ref<SNode>) -> WNode {
    WNode::new(node, Sequence::new(expression(node.front(0)), token(node.front(1)), expressions(node.front(2)), token(node.front(3))))
}

fn expressions(node: Ref<SNode>) -> Box<[WNode]> {
    node.children().iter()
        .step_by(2)
        .map(|child| expression(Ref::new(child)))
        .collect()
}

fn assignment(node: Ref<SNode>) -> WNode {
    WNode::new(node, Assignment::new(expression(node.front(0)), expression(node.front(2)), token(node.front(1))))
}

fn binop(node: Ref<SNode>) -> WNode {
    WNode::new(node, Binop::new(expression(node.front(0)), token(node.front(1)), expression(node.front(2))))
}

fn preop(node: Ref<SNode>) -> WNode {
    WNode::new(node, Preop::new(token(node.front(0)), expression(node.front(1))))
}

fn token(node: Ref<SNode>) -> Ref<str> {
    node.text()
}
