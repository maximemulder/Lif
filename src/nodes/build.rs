use crate::elements;
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
use crate::nodes::operation::Operation;
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

pub fn program<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Program::new(statements(text, &node.children()[0])))
}

fn statements<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    let mut statements = Vec::new();
    for child in node.children() {
        statements.push(statement(text, child));
    }

    Node::new(node, Statements::new(statements))
}

fn statement<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    let child = &node.children()[0];
    Node::new(node, Statement::new(match *child.element {
        elements::structures::STRUCTURE   => structure(text, child),
        elements::flows::FLOW             => flow(text, child),
        elements::expressions::EXPRESSION => expression(text, child),
        _ => panic!(),
    }))
}

fn expression<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    let child = &node.children()[0];
    match *child.element {
        elements::structures::CLASS        => class(text, child),
        elements::structures::FUNCTION     => function(text, child),
        elements::flows::FLOW              => flow(text, child),
        elements::controls::CONTROL        => control(text, child),
        elements::expressions::LET         => r#let(text, child),
        elements::expressions::ARRAY       => array(text, child),
        elements::expressions::GROUP       => group(text, child),
        elements::expressions::LITERAL     => literal(text, child),
        elements::expressions::CHAIN       => chain(text, child),
        elements::expressions::SEQUENCE    => sequence(text, child),
        elements::expressions::OPERATION   => operation(text, child),
        elements::expressions::ASSIGNMENT  => assignment(text, child),
        _ => panic!(),
    }
}

fn r#type<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Option<Node<'a>> {
    node.children().get(1).map(|child| expression(text, child))
}

fn name<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Option<&'a str> {
    node.children().get(0).map(|child| token(text, child))
}

fn literal<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    let child = &node.children()[0];
    match *child.element {
        elements::keywords::TRUE        => r#true(text, child),
        elements::keywords::FALSE       => r#false(text, child),
        elements::variables::NUMBER     => integer(text, child),
        elements::variables::STRING     => string(text, child),
        elements::variables::IDENTIFIER => identifier(text, child),
        _ => { panic!() },
    }
}

fn r#true<'a>(_: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, True::new())
}

fn r#false<'a>(_: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, False::new())
}

fn integer<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Integer::new(text[node.left() .. node.right()].parse::<usize>().unwrap()))
}

fn string<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, String::new(&text[node.left() + 1 .. node.right() - 1]))
}

fn identifier<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Identifier::new(&text[node.left() .. node.right()]))
}

fn structure<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    let child = &node.children()[0];
    Node::new(node, Structure::new(match *child.element {
        elements::structures::CLASS    => class_named(text, child),
        elements::structures::FUNCTION => function_named(text, child),
        _ => panic!(),
    }))
}

fn flow<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    let child = &node.children()[0];
    match *child.element {
        elements::flows::BLOCK     => block(text, child),
        elements::flows::IF        => r#if(text, child),
        elements::flows::LOOP      => r#loop(text, child),
        elements::flows::WHILE     => r#while(text, child),
        elements::flows::DO_WHILE  => do_while(text, child),
        elements::flows::FOR_IN    => for_in(text, child),
        _ => panic!(),
    }
}

fn block<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Block::new(statements(text, &node.children()[1]), if node.children().len() == 4 {
        Some(expression(text, &node.children()[2]))
    } else {
        None
    }))
}

fn r#if<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, If::new(expression(text, &node.children()[1]), block(text, &node.children()[2]), node.children().get(4).map(|child| block(text, child))))
}

fn r#loop<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Loop::new(block(text, &node.children()[1])))
}

fn r#while<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, While::new(expression(text, &node.children()[1]), block(text, &node.children()[2])))
}

fn do_while<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, DoWhile::new(block(text, &node.children()[1]), expression(text, &node.children()[3])))
}

fn for_in<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, ForIn::new(token(text, &node.children()[1]), expression(text, &node.children()[3]), block(text, &node.children()[4])))
}

fn r#let<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    declaration(text, &node.children()[1])
}

fn declaration<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Declaration::new(token(text, &node.children()[0]), r#type(text, &node.children()[1])))
}

fn control<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    let child = &node.children()[0];
    match *child.element {
        elements::controls::RETURN   => r#return(text, child),
        elements::controls::BREAK    => r#break(text, child),
        elements::controls::CONTINUE => r#continue(text, child),
        _ => panic!(),
    }
}

fn r#return<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Return::new(node.children().get(1).map(|child| expression(text, child))))
}

fn r#break<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Break::new(node.children().get(1).map(|child| expression(text, child))))
}

fn r#continue<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Continue::new(node.children().get(1).map(|child| expression(text, child))))
}

fn generics<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Box<[&'a str]> {
    let mut identifiers = Vec::new();
    for child in node.children().iter().step_by(2)  {
        identifiers.push(token(text, child));
    }

    identifiers.into_boxed_slice()
}

fn class<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    let children = node.children();
    let name = name(text, &children[1]);
    let class = Node::new(node, Class::new(name, r#type(text, &children[children.len() - 4]), methods(text, &children[children.len() - 2])));
    if children.len() >= 7 {
        Node::new(node, Generic::new(name, generics(text, &children[3]), class))
    } else {
        class
    }
}

fn class_named<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    let children = node.children();
    let name = Some(token(text, &children[1]));
    let class = Node::new(node, Class::new(name, r#type(text, &children[children.len() - 4]), methods(text, &children[children.len() - 2])));
    if children.len() >= 7 {
        Node::new(node, Generic::new(name, generics(text, &children[3]), class))
    } else {
        class
    }
}

fn methods<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Box<[Node<'a>]> {
    let mut functions = Vec::new();
    for child in node.children().iter() {
        functions.push(function_named(text, child));
    }

    functions.into_boxed_slice()
}

fn function<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    let children = node.children();
    let name = name(text, &children[1]);
    let function = Node::new(node, Function::new(name, parameters(text, &children[children.len() - 4]), r#type(text, &children[children.len() - 2]), block(text, &children.last().unwrap())));

    if children.len() >= 9 {
        Node::new(node, Generic::new(name, generics(text, &children[3]), function))
    } else {
        function
    }
}

fn function_named<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    let children = node.children();
    let name = Some(token(text, &children[1]));
    let function = Node::new(node, Function::new(name, parameters(text, &children[children.len() - 4]), r#type(text, &children[children.len() - 2]), block(text, &children.last().unwrap())));

    if children.len() >= 9 {
        Node::new(node, Generic::new(name, generics(text, &children[3]), function))
    } else {
        function
    }
}

fn parameters<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Box<[Node<'a>]> {
    let mut declarations = Vec::new();
    for child in node.children().iter().step_by(2)  {
        declarations.push(declaration(text, child));
    }

    declarations.into_boxed_slice()
}

fn array<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Array::new(expressions(text, &node.children()[1])))
}

fn group<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Group::new(expression(text, &node.children()[1])))
}

fn chain<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Chain::new(expression(text, &node.children()[0]), token(text, &node.children()[2])))
}

fn sequence<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Sequence::new(expression(text, &node.children()[0]), token(text, &node.children()[1]), expressions(text, &node.children()[2]), token(text, &node.children()[3])))
}

fn expressions<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Box<[Node<'a>]> {
    let mut expressions = Vec::new();
    for child in node.children().iter().step_by(2)  {
        expressions.push(expression(text, child));
    }

    expressions.into_boxed_slice()
}

fn assignment<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Assignment::new(expression(text, &node.children()[0]), expression(text, &node.children()[2]), token(text, &node.children()[1])))
}

fn operation<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Operation::new(expression(text, &node.children()[0]), expression(text, &node.children()[2]), token(text, &node.children()[1])))
}

fn token<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> &'a str {
    &text[node.left() .. node.right()]
}
