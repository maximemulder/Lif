use crate::memory::Ref;
use crate::parser::SNode;
use crate::parser::elements;
use crate::walker::ANode;
use crate::walker::nodes::*;

use std::marker::Unsize;

pub fn traitify<T: ?Sized, N: Unsize<T>>(builder: impl FnOnce(Ref<SNode>) -> ANode<N>) -> impl FnOnce(Ref<SNode>) -> Box<ANode<T>> {
    |node| Box::<ANode<N>>::new(builder(node))
}

pub fn program(node: Ref<SNode>) -> ANode<AProgram> {
    ANode::new(node, AProgram::new(statements(node.front(0))))
}

fn statements(node: Ref<SNode>) -> ANode<AStatements> {
    ANode::new(node, AStatements::new(node.children().iter()
        .map(|child| statement(Ref::new(child)))
        .collect()
    ))
}

fn statement(node: Ref<SNode>) -> ANode<AStatement> {
    let child = node.front(0);
    ANode::new(node, AStatement::new(match *child.element {
        elements::structures::STRUCTURE   => Box::new(structure(child)),
        elements::controls::CONTROL       => Box::new(control(child)),
        elements::expressions::EXPRESSION => Box::new(expression(child)),
        _ => panic!(),
    }))
}

pub fn expression(node: Ref<SNode>) -> ANode<AExpression> {
    let child = node.front(0);
    ANode::new(node, AExpression::new(match *child.element {
        elements::controls::CONTROL       => Box::new(control(child)),
        elements::jumps::JUMP             => Box::new(jump(child)),
        elements::expressions::LET        => Box::new(r#let(child)),
        elements::expressions::LITERAL    => Box::new(literal(child)),
        elements::expressions::CHAIN      => Box::new(chain(child)),
        elements::expressions::SEQUENCE   => Box::new(sequence(child)),
        elements::expressions::BINOP      => Box::new(binop(child)),
        elements::expressions::PREOP      => Box::new(preop(child)),
        elements::expressions::ASSIGNMENT => Box::new(assignment(child)),
        _ => panic!(),
    }))
}

fn r#type(node: Ref<SNode>) -> ANode<AType> {
    ANode::new(node, AType::new(node.children().get(1).map(|child| expression(Ref::new(child)))))
}

fn name(node: Ref<SNode>) -> Option<Ref<str>> {
    node.children().get(0).map(|child| token(Ref::new(child)))
}

fn literal(node: Ref<SNode>) -> ANode<ALiteral> {
    let child = node.front(0);
    ANode::new(node, ALiteral::new(match *child.element {
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

fn structure(node: Ref<SNode>) -> ANode<AStructure> {
    let child = node.front(0);
    ANode::new(node, AStructure::new(match *child.element {
        elements::structures::CLASS    => class(child),
        elements::structures::FUNCTION => function(child),
        _ => panic!(),
    }))
}

fn control(node: Ref<SNode>) -> ANode<AControl> {
    let child = node.front(0);
    ANode::new(node, AControl::new(match *child.element {
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

fn r#let(node: Ref<SNode>) -> ANode<ALet> {
    ANode::new(node, ALet::new(declaration(node.front(1))))
}

fn declaration(node: Ref<SNode>) -> ANode<ADeclaration> {
    ANode::new(node, ADeclaration::new(token(node.front(0)), r#type(node.front(1))))
}

fn jump(node: Ref<SNode>) -> ANode<AJump> {
    let child = node.front(0);
    ANode::new(node, AJump::new(match *child.element {
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

fn rest(node: Ref<SNode>) -> Option<(Ref<str>, ANode<AType>)> {
    node.children().get(1).map(|child| parameter(Ref::new(child)))
}

fn parameters(node: Ref<SNode>) -> (Box<[(Ref<str>, ANode<AType>)]>, Option<(Ref<str>, ANode<AType>)>) {
    let parameters = node.front(1).children().iter()
        .step_by(2)
        .map(|child| parameter(Ref::new(child)))
        .collect();

    (parameters, rest(node.back(2)))
}

fn parameter(node: Ref<SNode>) -> (Ref<str>, ANode<AType>) {
    (token(node.front(0)), r#type(node.front(1)))
}

fn chain(node: Ref<SNode>) -> ANode<AChain> {
    ANode::new(node, AChain::new(expression(node.front(0)), token(node.front(2))))
}

fn sequence(node: Ref<SNode>) -> ANode<ASequence> {
    ANode::new(node, ASequence::new(expression(node.front(0)), token(node.front(1)), expressions(node.front(2)), token(node.front(3))))
}

fn expressions(node: Ref<SNode>) -> Box<[ANode<AExpression>]> {
    node.children().iter()
        .step_by(2)
        .map(|child| expression(Ref::new(child)))
        .collect()
}

fn assignment(node: Ref<SNode>) -> ANode<AAssignment> {
    ANode::new(node, AAssignment::new(expression(node.front(0)), expression(node.front(2)), token(node.front(1))))
}

fn preop(node: Ref<SNode>) -> ANode<APreop> {
    ANode::new(node, APreop::new(token(node.front(0)), expression(node.front(1))))
}

fn binop(node: Ref<SNode>) -> ANode<ABinop> {
    ANode::new(node, ABinop::new(expression(node.front(0)), token(node.front(1)), expression(node.front(2))))
}

fn token(node: Ref<SNode>) -> Ref<str> {
    node.text()
}
