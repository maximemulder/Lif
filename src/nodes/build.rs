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
use crate::nodes::integer::Integer;
use crate::nodes::identifier::Identifier;
use crate::nodes::string::String;
use crate::nodes::r#true::True;
use crate::nodes::r#false::False;
use crate::nodes::r#return::Return;
use crate::nodes::r#break::Break;
use crate::nodes::r#continue::Continue;

pub fn program<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Program::new(statements(node.front(0))))
}

fn statements<'a>(node: Ref<SyntaxNode>) -> Node {
    let mut statements = Vec::new();
    for child in node.children() {
        statements.push(statement(Ref::from_ref(child)));
    }

    Node::new(node, Statements::new(statements.into_boxed_slice()))
}

fn statement<'a>(node: Ref<SyntaxNode>) -> Node {
    let child = node.front(0);
    Node::new(node, Statement::new(match *child.element {
        elements::structures::STRUCTURE   => structure(child),
        elements::flows::FLOW             => flow(child),
        elements::expressions::EXPRESSION => expression(child),
        _ => panic!(),
    }))
}

pub fn expression<'a>(node: Ref<SyntaxNode>) -> Node {
    let child = node.front(0);
    match *child.element {
        elements::structures::CLASS        => class(child),
        elements::structures::FUNCTION     => function(child),
        elements::flows::FLOW              => flow(child),
        elements::controls::CONTROL        => control(child),
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

fn r#type<'a>(node: Ref<SyntaxNode>) -> Option<Node> {
    node.children().get(1).map(|child| expression(Ref::from_ref(child)))
}

fn name<'a>(node: Ref<SyntaxNode>) -> Option<Ref<str>> {
    node.children().get(0).map(|child| token(Ref::from_ref(child)))
}

fn literal<'a>(node: Ref<SyntaxNode>) -> Node {
    let child = node.front(0);
    match *child.element {
        elements::keywords::TRUE        => r#true(child),
        elements::keywords::FALSE       => r#false(child),
        elements::variables::NUMBER     => integer(child),
        elements::variables::STRING     => string(child),
        elements::variables::IDENTIFIER => identifier(child),
        _ => panic!(),
    }
}

fn r#true<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, True::new())
}

fn r#false<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, False::new())
}

fn integer<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Integer::new(node.text().parse::<isize>().unwrap()))
}

fn string<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, String::new(node.text()))
}

fn identifier<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Identifier::new(node.text()))
}

fn structure<'a>(node: Ref<SyntaxNode>) -> Node {
    let child = node.front(0);
    Node::new(node, Structure::new(match *child.element {
        elements::structures::CLASS    => class_named(child),
        elements::structures::FUNCTION => function_named(child),
        _ => panic!(),
    }))
}

fn flow<'a>(node: Ref<SyntaxNode>) -> Node {
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

fn block<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Block::new(statements(node.front(1)), if node.children().len() == 4 {
        Some(expression(node.front(2)))
    } else {
        None
    }))
}

fn r#if<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, If::new(expression(node.front(1)), block(node.front(2)), node.children().get(4).map(|child| block(Ref::from_ref(child)))))
}

fn r#loop<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Loop::new(block(node.front(1))))
}

fn r#while<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, While::new(expression(node.front(1)), block(node.front(2))))
}

fn do_while<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, DoWhile::new(block(node.front(1)), expression(node.front(3))))
}

fn for_in<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, ForIn::new(token(node.front(1)), expression(node.front(3)), block(node.front(4))))
}

fn r#let<'a>(node: Ref<SyntaxNode>) -> Node {
    declaration(node.front(1))
}

fn declaration<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Declaration::new(token(node.front(0)), r#type(node.front(1))))
}

fn control<'a>(node: Ref<SyntaxNode>) -> Node {
    let child = node.front(0);
    match *child.element {
        elements::controls::RETURN   => r#return(child),
        elements::controls::BREAK    => r#break(child),
        elements::controls::CONTINUE => r#continue(child),
        _ => panic!(),
    }
}

fn r#return<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Return::new(node.children().get(1).map(|child| expression(Ref::from_ref(child)))))
}

fn r#break<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Break::new(node.children().get(1).map(|child| expression(Ref::from_ref(child)))))
}

fn r#continue<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Continue::new(node.children().get(1).map(|child| expression(Ref::from_ref(child)))))
}

fn generics<'a>(node: Ref<SyntaxNode>) -> Box<[Ref<str>]> {
    let mut identifiers = Vec::new();
    for child in node.front(1).children().iter().step_by(2)  {
        identifiers.push(token(Ref::from_ref(child)));
    }

    identifiers.into_boxed_slice()
}

fn class<'a>(node: Ref<SyntaxNode>) -> Node {
    let name = name(node.front(1));
    let class = Node::new(node, Class::new(name, r#type(node.back(4)), methods(node.back(2))));
    if node.length() >= 7 {
        Node::new(node, Generic::new(name, generics(node.front(2)), class))
    } else {
        class
    }
}

fn class_named<'a>(node: Ref<SyntaxNode>) -> Node {
    let name = Some(token(node.front(1)));
    let class = Node::new(node, Class::new(name, r#type(node.back(4)), methods(node.back(2))));
    if node.length() >= 7 {
        Node::new(node, Generic::new(name, generics(node.front(2)), class))
    } else {
        class
    }
}

fn methods<'a>(node: Ref<SyntaxNode>) -> Box<[Node]> {
    let mut functions = Vec::new();
    for child in node.children().iter() {
        functions.push(function_named(Ref::from_ref(child)));
    }

    functions.into_boxed_slice()
}

fn function<'a>(node: Ref<SyntaxNode>) -> Node {
    let name = name(node.front(1));
    let function = Node::new(node, Function::new(name, parameters(node.back(3)), r#type(node.back(2)), block(node.back(1))));
    if node.length() >= 6 {
        Node::new(node, Generic::new(name, generics(node.front(2)), function))
    } else {
        function
    }
}

fn function_named<'a>(node: Ref<SyntaxNode>) -> Node {
    let name = Some(token(node.front(1)));
    let function = Node::new(node, Function::new(name, parameters(node.back(3)), r#type(node.back(2)), block(node.back(1))));
    if node.length() >= 6 {
        Node::new(node, Generic::new(name, generics(node.front(2)), function))
    } else {
        function
    }
}

fn rest<'a>(node: Ref<SyntaxNode>) -> Option<(Ref<str>, Option<Node>)> {
    node.children().get(1).map(|child| parameter(Ref::from_ref(child)))
}

fn parameters<'a>(node: Ref<SyntaxNode>) -> (Box<[(Ref<str>, Option<Node>)]>, Option<(Ref<str>, Option<Node>)>) {
    let mut parameters = Vec::new();
    for child in node.front(1).children().iter().step_by(2)  {
        parameters.push(parameter(Ref::from_ref(child)));
    }

    (parameters.into_boxed_slice(), rest(node.back(2)))
}

fn parameter<'a>(node: Ref<SyntaxNode>) -> (Ref<str>, Option<Node>) {
    (token(node.front(0)), r#type(node.front(1)))
}

fn array<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Array::new(expressions(node.front(1))))
}

fn group<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Group::new(expression(node.front(1))))
}

fn chain<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Chain::new(expression(node.front(0)), token(node.front(2))))
}

fn sequence<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Sequence::new(expression(node.front(0)), token(node.front(1)), expressions(node.front(2)), token(node.front(3))))
}

fn expressions<'a>(node: Ref<SyntaxNode>) -> Box<[Node]> {
    let mut expressions = Vec::new();
    for child in node.children().iter().step_by(2)  {
        expressions.push(expression(Ref::from_ref(child)));
    }

    expressions.into_boxed_slice()
}

fn assignment<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Assignment::new(expression(node.front(0)), expression(node.front(2)), token(node.front(1))))
}

fn binop<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Binop::new(expression(node.front(0)), token(node.front(1)), expression(node.front(2))))
}

fn preop<'a>(node: Ref<SyntaxNode>) -> Node {
    Node::new(node, Preop::new(token(node.front(0)), expression(node.front(1))))
}

fn token<'a>(node: Ref<SyntaxNode>) -> Ref<str> {
    node.text()
}
