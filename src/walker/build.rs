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
        elements::productions::DEFINITION => Box::new(definition(child)),
        elements::productions::STRUCTURE  => Box::new(structure(child)),
        elements::productions::EXPRESSION => Box::new(expression(child)),
        _ => panic!(),
    }))
}

pub fn expression(node: Ref<SNode>) -> ANode<AExpression> {
    let child = node.front(0);
    ANode::new(node, AExpression::new(match *child.element {
        elements::productions::STRUCTURE  => Box::new(structure(child)),
        elements::expressions::JUMP       => Box::new(jump(child)),
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

fn literal(node: Ref<SNode>) -> ANode<ALiteral> {
    let child = node.front(0);
    ANode::new(node, ALiteral::new(match *child.element {
        elements::keywords::TRUE       => Box::new(boolean(child)),
        elements::keywords::FALSE      => Box::new(boolean(child)),
        elements::literals::INTEGER    => Box::new(integer(child)),
        elements::literals::FLOAT      => Box::new(float(child)),
        elements::literals::STRING     => Box::new(string(child)),
        elements::literals::IDENTIFIER => Box::new(identifier(child)),
        _ => panic!(),
    }))
}

fn boolean(node: Ref<SNode>) -> ANode<ABoolean> {
    ANode::new(node, ABoolean::new(node.text()))
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

fn definition(node: Ref<SNode>) -> ANode<ADefinition> {
    let child = node.front(0);
    ANode::new(node, ADefinition::new(match *child.element {
        elements::definitions::CLASS    => Box::new(class(child)),
        elements::definitions::FUNCTION => Box::new(function(child)),
        _ => panic!(),
    }))
}

fn structure(node: Ref<SNode>) -> ANode<AStructure> {
    let child = node.front(0);
    ANode::new(node, AStructure::new(match *child.element {
        elements::structures::BLOCK => Box::new(block(child)),
        elements::structures::IF    => Box::new(r#if(child)),
        elements::structures::LOOP  => Box::new(r#loop(child)),
        elements::structures::WHILE => Box::new(r#while(child)),
        elements::structures::FOR   => Box::new(r#for(child)),
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
    ANode::new(node, AJump::new(token(node.front(0)), node.children().get(1).map(|child| expression(Ref::new(child)))))
}

fn generics(node: Ref<SNode>) -> ANode<AGenerics> {
    ANode::new(node, AGenerics::new(if let Some(child) = node.children().get(1) {
        child.children().iter()
            .step_by(2)
            .map(|child| token(Ref::new(child)))
            .collect()
    } else {
        Box::new([])
    }))
}

fn class(node: Ref<SNode>) -> ANode<AClass> {
    ANode::new(node, AClass::new(
        token(node.front(1)),
        generics(node.front(2)),
        r#type(node.back(4)),
        methods(node.back(2))
    ))
}

fn methods(node: Ref<SNode>) -> Box<[ANode<AFunction>]> {
    node.children().iter()
        .map(|child| function(Ref::new(child)))
        .collect()
}

fn function(node: Ref<SNode>) -> ANode<AFunction> {
    ANode::new(node, AFunction::new(
        token(node.front(1)),
        generics(node.front(2)),
        parameters(node.front(3)),
        r#type(node.front(4)),
        block(node.front(5))
    ))
}

fn rest(node: Ref<SNode>) -> Option<ANode<ADeclaration>> {
    node.children().get(1).map(|child| declaration(Ref::new(child)))
}

fn parameters(node: Ref<SNode>) -> ANode<AParameters> {
    ANode::new(node, AParameters::new(node.front(1).children().iter().step_by(2).map(|child| declaration(Ref::new(child))).collect(), rest(node.back(2))))
}

fn chain(node: Ref<SNode>) -> ANode<AChain> {
    ANode::new(node, AChain::new(expression(node.front(0)), token(node.front(2))))
}

fn sequence(node: Ref<SNode>) -> ANode<ASequence> {
    ANode::new(node, ASequence::new(expression(node.front(0)), token(node.front(1)), expressions(node.front(2)), ))
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
