use crate::memory::Ref;
use crate::parser::SNode;
use crate::parser::elements;
use crate::walker::WNode;
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
        elements::flows::FLOW             => flow(child),
        elements::expressions::EXPRESSION => expression(child),
        _ => panic!(),
    }))
}

pub fn expression(node: Ref<SNode>) -> WNode {
    let child = node.front(0);
    match *child.element {
        elements::structures::CLASS        => class(child),
        elements::structures::FUNCTION     => function(child),
        elements::flows::FLOW              => flow(child),
        elements::jumps::JUMP              => jump(child),
        elements::expressions::LET         => r#let(child),
        elements::expressions::ARRAY       => array(child),
        elements::expressions::GROUP       => group(child),
        elements::expressions::LITERAL     => literal(child),
        elements::expressions::CHAIN       => chain(child),
        elements::expressions::SEQUENCE    => sequence(child),
        elements::expressions::BINOP       => binop(child),
        elements::expressions::PREOP       => preop(child),
        elements::expressions::ASSIGNMENT  => assignment(child),
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
    match *child.element {
        elements::keywords::TRUE        => r#true(child),
        elements::keywords::FALSE       => r#false(child),
        elements::variables::INTEGER    => integer(child),
        elements::variables::FLOAT      => float(child),
        elements::variables::STRING     => string(child),
        elements::variables::IDENTIFIER => identifier(child),
        _ => panic!(),
    }
}

fn r#true(node: Ref<SNode>) -> WNode {
    WNode::new(node, True::new())
}

fn r#false(node: Ref<SNode>) -> WNode {
    WNode::new(node, False::new())
}

fn integer(node: Ref<SNode>) -> WNode {
    WNode::new(node, Integer::new(node.text()))
}

fn float(node: Ref<SNode>) -> WNode {
    WNode::new(node, Float::new(node.text()))
}

fn string(node: Ref<SNode>) -> WNode {
    WNode::new(node, String::new(node.text()))
}

fn identifier(node: Ref<SNode>) -> WNode {
    WNode::new(node, Identifier::new(node.text()))
}

fn structure(node: Ref<SNode>) -> WNode {
    let child = node.front(0);
    WNode::new(node, Structure::new(match *child.element {
        elements::structures::CLASS    => class_named(child),
        elements::structures::FUNCTION => function_named(child),
        _ => panic!(),
    }))
}

fn flow(node: Ref<SNode>) -> WNode {
    let child = node.front(0);
    match *child.element {
        elements::flows::BLOCK     => block(child),
        elements::flows::IF        => r#if(child),
        elements::flows::LOOP      => r#loop(child),
        elements::flows::WHILE     => r#while(child),
        elements::flows::DO_WHILE  => do_while(child),
        elements::flows::FOR_IN    => for_in(child),
        _ => panic!(),
    }
}

fn block(node: Ref<SNode>) -> WNode {
    WNode::new(node, Block::new(statements(node.front(1)), (node.children().len() == 4).then(|| expression(node.front(2)))))
}

fn r#if(node: Ref<SNode>) -> WNode {
    WNode::new(node, If::new(expression(node.front(1)), block(node.front(2)), node.children().get(4).map(|child| block(Ref::new(child)))))
}

fn r#loop(node: Ref<SNode>) -> WNode {
    WNode::new(node, Loop::new(block(node.front(1))))
}

fn r#while(node: Ref<SNode>) -> WNode {
    WNode::new(node, While::new(expression(node.front(1)), block(node.front(2))))
}

fn do_while(node: Ref<SNode>) -> WNode {
    WNode::new(node, DoWhile::new(block(node.front(1)), expression(node.front(3))))
}

fn for_in(node: Ref<SNode>) -> WNode {
    WNode::new(node, ForIn::new(token(node.front(1)), expression(node.front(3)), block(node.front(4))))
}

fn r#let(node: Ref<SNode>) -> WNode {
    declaration(node.front(1))
}

fn declaration(node: Ref<SNode>) -> WNode {
    WNode::new(node, Declaration::new(token(node.front(0)), r#type(node.front(1))))
}

fn jump(node: Ref<SNode>) -> WNode {
    let child = node.front(0);
    match *child.element {
        elements::jumps::CONTINUE => r#continue(child),
        elements::jumps::BREAK    => r#break(child),
        elements::jumps::RETURN   => r#return(child),
        _ => panic!(),
    }
}

fn r#return(node: Ref<SNode>) -> WNode {
    WNode::new(node, Return::new(node.children().get(1).map(|child| expression(Ref::new(child)))))
}

fn r#break(node: Ref<SNode>) -> WNode {
    WNode::new(node, Break::new(node.children().get(1).map(|child| expression(Ref::new(child)))))
}

fn r#continue(node: Ref<SNode>) -> WNode {
    WNode::new(node, Continue::new(node.children().get(1).map(|child| expression(Ref::new(child)))))
}

fn generics(node: Ref<SNode>) -> Box<[Ref<str>]> {
    node.front(1).children().iter()
        .step_by(2)
        .map(|child| token(Ref::new(child)))
        .collect()
}

fn class(node: Ref<SNode>) -> WNode {
    let name = name(node.front(1));
    let class = WNode::new(node, Class::new(name, r#type(node.back(4)), methods(node.back(2))));
    if node.children().len() >= 7 {
        WNode::new(node, Generic::new(name, generics(node.front(2)), class))
    } else {
        class
    }
}

fn class_named(node: Ref<SNode>) -> WNode {
    let name = Some(token(node.front(1)));
    let class = WNode::new(node, Class::new(name, r#type(node.back(4)), methods(node.back(2))));
    if node.children().len() >= 7 {
        WNode::new(node, Generic::new(name, generics(node.front(2)), class))
    } else {
        class
    }
}

fn methods(node: Ref<SNode>) -> Box<[WNode]> {
    node.children().iter()
        .map(|child| function_named(Ref::new(child)))
        .collect()
}

fn function(node: Ref<SNode>) -> WNode {
    let name = name(node.front(1));
    let function = WNode::new(node, Function::new(name, parameters(node.back(3)), r#type(node.back(2)), block(node.back(1))));
    if node.children().len() >= 6 {
        WNode::new(node, Generic::new(name, generics(node.front(2)), function))
    } else {
        function
    }
}

fn function_named(node: Ref<SNode>) -> WNode {
    let name = Some(token(node.front(1)));
    let function = WNode::new(node, Function::new(name, parameters(node.back(3)), r#type(node.back(2)), block(node.back(1))));
    if node.children().len() >= 6 {
        WNode::new(node, Generic::new(name, generics(node.front(2)), function))
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

fn array(node: Ref<SNode>) -> WNode {
    WNode::new(node, Array::new(expressions(node.front(1))))
}

fn group(node: Ref<SNode>) -> WNode {
    WNode::new(node, Group::new(expression(node.front(1))))
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
