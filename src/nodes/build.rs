use crate::elements;
use crate::memory::Ref;
use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::program::Program;
use crate::nodes::r#if::If;
use crate::nodes::r#loop::Loop;
use crate::nodes::r#while::While;
use crate::nodes::do_while::DoWhile;
use crate::nodes::for_in::ForIn;
use crate::nodes::statement::Statement;
use crate::nodes::statements::Statements;
use crate::nodes::assignment::Assignment;
use crate::nodes::preop::Preop;
use crate::nodes::binop::Binop;
use crate::nodes::chain::Chain;
use crate::nodes::sequence::Sequence;
use crate::nodes::declaration::Declaration;
use crate::nodes::generic::Generic;
use crate::nodes::structure::Structure;
use crate::nodes::class::Class;
use crate::nodes::function::Function;
use crate::nodes::array::Array;
use crate::nodes::block::Block;
use crate::nodes::group::Group;
use crate::nodes::r#true::True;
use crate::nodes::r#false::False;
use crate::nodes::integer::Integer;
use crate::nodes::float::Float;
use crate::nodes::string::String;
use crate::nodes::identifier::Identifier;
use crate::nodes::r#return::Return;
use crate::nodes::r#break::Break;
use crate::nodes::r#continue::Continue;

pub fn program(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Program::new(statements(node.front(0))))
}

fn statements(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Statements::new(node.children().iter()
        .map(|child| statement(Ref::new(child)))
        .collect()
    ))
}

fn statement(node: Ref<SyntaxNode>) -> Node {
    let child = node.front(0);
    Node::new(node, Statement::new(match *child.element {
        elements::structures::STRUCTURE   => structure(child),
        elements::flows::FLOW             => flow(child),
        elements::expressions::EXPRESSION => expression(child),
        _ => panic!(),
    }))
}

pub fn expression(node: Ref<SyntaxNode>) -> Node {
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

fn r#type(node: Ref<SyntaxNode>) -> Option<Node> {
    node.children().get(1).map(|child| expression(Ref::new(child)))
}

fn name(node: Ref<SyntaxNode>) -> Option<Ref<str>> {
    node.children().get(0).map(|child| token(Ref::new(child)))
}

fn literal(node: Ref<SyntaxNode>) -> Node {
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

fn r#true(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, True::new())
}

fn r#false(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, False::new())
}

fn integer(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Integer::new(node.text()))
}

fn float(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Float::new(node.text()))
}

fn string(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, String::new(node.text()))
}

fn identifier(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Identifier::new(node.text()))
}

fn structure(node: Ref<SyntaxNode>) -> Node {
    let child = node.front(0);
    Node::new(node, Structure::new(match *child.element {
        elements::structures::CLASS    => class_named(child),
        elements::structures::FUNCTION => function_named(child),
        _ => panic!(),
    }))
}

fn flow(node: Ref<SyntaxNode>) -> Node {
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

fn block(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Block::new(statements(node.front(1)), if node.children().len() == 4 {
        Some(expression(node.front(2)))
    } else {
        None
    }))
}

fn r#if(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, If::new(expression(node.front(1)), block(node.front(2)), node.children().get(4).map(|child| block(Ref::new(child)))))
}

fn r#loop(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Loop::new(block(node.front(1))))
}

fn r#while(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, While::new(expression(node.front(1)), block(node.front(2))))
}

fn do_while(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, DoWhile::new(block(node.front(1)), expression(node.front(3))))
}

fn for_in(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, ForIn::new(token(node.front(1)), expression(node.front(3)), block(node.front(4))))
}

fn r#let(node: Ref<SyntaxNode>) -> Node {
    declaration(node.front(1))
}

fn declaration(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Declaration::new(token(node.front(0)), r#type(node.front(1))))
}

fn jump(node: Ref<SyntaxNode>) -> Node {
    let child = node.front(0);
    match *child.element {
        elements::jumps::CONTINUE => r#continue(child),
        elements::jumps::BREAK    => r#break(child),
        elements::jumps::RETURN   => r#return(child),
        _ => panic!(),
    }
}

fn r#return(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Return::new(node.children().get(1).map(|child| expression(Ref::new(child)))))
}

fn r#break(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Break::new(node.children().get(1).map(|child| expression(Ref::new(child)))))
}

fn r#continue(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Continue::new(node.children().get(1).map(|child| expression(Ref::new(child)))))
}

fn generics(node: Ref<SyntaxNode>) -> Box<[Ref<str>]> {
    node.front(1).children().iter()
        .step_by(2)
        .map(|child| token(Ref::new(child)))
        .collect()
}

fn class(node: Ref<SyntaxNode>) -> Node {
    let name = name(node.front(1));
    let class = Node::new(node, Class::new(name, r#type(node.back(4)), methods(node.back(2))));
    if node.length() >= 7 {
        Node::new(node, Generic::new(name, generics(node.front(2)), class))
    } else {
        class
    }
}

fn class_named(node: Ref<SyntaxNode>) -> Node {
    let name = Some(token(node.front(1)));
    let class = Node::new(node, Class::new(name, r#type(node.back(4)), methods(node.back(2))));
    if node.length() >= 7 {
        Node::new(node, Generic::new(name, generics(node.front(2)), class))
    } else {
        class
    }
}

fn methods(node: Ref<SyntaxNode>) -> Box<[Node]> {
    node.children().iter()
        .map(|child| function_named(Ref::new(child)))
        .collect()
}

fn function(node: Ref<SyntaxNode>) -> Node {
    let name = name(node.front(1));
    let function = Node::new(node, Function::new(name, parameters(node.back(3)), r#type(node.back(2)), block(node.back(1))));
    if node.length() >= 6 {
        Node::new(node, Generic::new(name, generics(node.front(2)), function))
    } else {
        function
    }
}

fn function_named(node: Ref<SyntaxNode>) -> Node {
    let name = Some(token(node.front(1)));
    let function = Node::new(node, Function::new(name, parameters(node.back(3)), r#type(node.back(2)), block(node.back(1))));
    if node.length() >= 6 {
        Node::new(node, Generic::new(name, generics(node.front(2)), function))
    } else {
        function
    }
}

fn rest(node: Ref<SyntaxNode>) -> Option<(Ref<str>, Option<Node>)> {
    node.children().get(1).map(|child| parameter(Ref::new(child)))
}

fn parameters(node: Ref<SyntaxNode>) -> (Box<[(Ref<str>, Option<Node>)]>, Option<(Ref<str>, Option<Node>)>) {
    let parameters = node.front(1).children().iter()
        .step_by(2)
        .map(|child| parameter(Ref::new(child)))
        .collect();

    (parameters, rest(node.back(2)))
}

fn parameter(node: Ref<SyntaxNode>) -> (Ref<str>, Option<Node>) {
    (token(node.front(0)), r#type(node.front(1)))
}

fn array(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Array::new(expressions(node.front(1))))
}

fn group(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Group::new(expression(node.front(1))))
}

fn chain(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Chain::new(expression(node.front(0)), token(node.front(2))))
}

fn sequence(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Sequence::new(expression(node.front(0)), token(node.front(1)), expressions(node.front(2)), token(node.front(3))))
}

fn expressions(node: Ref<SyntaxNode>) -> Box<[Node]> {
    node.children().iter()
        .step_by(2)
        .map(|child| expression(Ref::new(child)))
        .collect()
}

fn assignment(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Assignment::new(expression(node.front(0)), expression(node.front(2)), token(node.front(1))))
}

fn binop(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Binop::new(expression(node.front(0)), token(node.front(1)), expression(node.front(2))))
}

fn preop(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Preop::new(token(node.front(0)), expression(node.front(1))))
}

fn token(node: Ref<SyntaxNode>) -> Ref<str> {
    node.text()
}
