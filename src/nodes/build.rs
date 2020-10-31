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

pub fn program<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Program::new(statements(&node.children()[0])))
}

fn statements<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    let mut statements = Vec::new();
    for child in node.children() {
        statements.push(statement(child));
    }

    Node::new(node, Statements::new(statements))
}

fn statement<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    let child = &node.children()[0];
    Node::new(node, Statement::new(match *child.element {
        elements::structures::STRUCTURE   => structure(child),
        elements::flows::FLOW             => flow(child),
        elements::expressions::EXPRESSION => expression(child),
        _ => panic!(),
    }))
}

fn expression<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    let child = &node.children()[0];
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
        elements::expressions::OPERATION   => operation(child),
        elements::expressions::ASSIGNMENT  => assignment(child),
        _ => panic!(),
    }
}

fn r#type<'a>(node: &'a SyntaxNode<'a>) -> Option<Node<'a>> {
    node.children().get(1).map(|child| expression(child))
}

fn name<'a>(node: &'a SyntaxNode<'a>) -> Option<&'a str> {
    node.children().get(0).map(|child| token(child))
}

fn literal<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    let child = &node.children()[0];
    match *child.element {
        elements::keywords::TRUE        => r#true(child),
        elements::keywords::FALSE       => r#false(child),
        elements::variables::NUMBER     => integer(child),
        elements::variables::STRING     => string(child),
        elements::variables::IDENTIFIER => identifier(child),
        _ => { panic!() },
    }
}

fn r#true<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, True::new())
}

fn r#false<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, False::new())
}

fn integer<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Integer::new(node.text().parse::<isize>().unwrap()))
}

fn string<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, String::new(node.text()))
}

fn identifier<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Identifier::new(node.text()))
}

fn structure<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    let child = &node.children()[0];
    Node::new(node, Structure::new(match *child.element {
        elements::structures::CLASS    => class_named(child),
        elements::structures::FUNCTION => function_named(child),
        _ => panic!(),
    }))
}

fn flow<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    let child = &node.children()[0];
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

fn block<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Block::new(statements(&node.children()[1]), if node.children().len() == 4 {
        Some(expression(&node.children()[2]))
    } else {
        None
    }))
}

fn r#if<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, If::new(expression(&node.children()[1]), block(&node.children()[2]), node.children().get(4).map(|child| block(child))))
}

fn r#loop<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Loop::new(block(&node.children()[1])))
}

fn r#while<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, While::new(expression(&node.children()[1]), block(&node.children()[2])))
}

fn do_while<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, DoWhile::new(block(&node.children()[1]), expression(&node.children()[3])))
}

fn for_in<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, ForIn::new(token(&node.children()[1]), expression(&node.children()[3]), block(&node.children()[4])))
}

fn r#let<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    declaration(&node.children()[1])
}

fn declaration<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Declaration::new(token(&node.children()[0]), r#type(&node.children()[1])))
}

fn control<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    let child = &node.children()[0];
    match *child.element {
        elements::controls::RETURN   => r#return(child),
        elements::controls::BREAK    => r#break(child),
        elements::controls::CONTINUE => r#continue(child),
        _ => panic!(),
    }
}

fn r#return<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Return::new(node.children().get(1).map(|child| expression(child))))
}

fn r#break<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Break::new(node.children().get(1).map(|child| expression(child))))
}

fn r#continue<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Continue::new(node.children().get(1).map(|child| expression(child))))
}

fn generics<'a>(node: &'a SyntaxNode<'a>) -> Box<[&'a str]> {
    let mut identifiers = Vec::new();
    for child in node.children().iter().step_by(2)  {
        identifiers.push(token(child));
    }

    identifiers.into_boxed_slice()
}

fn class<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    let children = node.children();
    let name = name(&children[1]);
    let class = Node::new(node, Class::new(name, r#type(&children[children.len() - 4]), methods(&children[children.len() - 2])));
    if children.len() >= 7 {
        Node::new(node, Generic::new(name, generics(&children[3]), class))
    } else {
        class
    }
}

fn class_named<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    let children = node.children();
    let name = Some(token(&children[1]));
    let class = Node::new(node, Class::new(name, r#type(&children[children.len() - 4]), methods(&children[children.len() - 2])));
    if children.len() >= 7 {
        Node::new(node, Generic::new(name, generics(&children[3]), class))
    } else {
        class
    }
}

fn methods<'a>(node: &'a SyntaxNode<'a>) -> Box<[Node<'a>]> {
    let mut functions = Vec::new();
    for child in node.children().iter() {
        functions.push(function_named(child));
    }

    functions.into_boxed_slice()
}

fn function<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    let children = node.children();
    let name = name(&children[1]);
    let function = Node::new(node, Function::new(name, parameters(&children[children.len() - 4]), r#type(&children[children.len() - 2]), block(&children.last().unwrap())));

    if children.len() >= 9 {
        Node::new(node, Generic::new(name, generics(&children[3]), function))
    } else {
        function
    }
}

fn function_named<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    let children = node.children();
    let name = Some(token(&children[1]));
    let function = Node::new(node, Function::new(name, parameters(&children[children.len() - 4]), r#type(&children[children.len() - 2]), block(&children.last().unwrap())));

    if children.len() >= 9 {
        Node::new(node, Generic::new(name, generics(&children[3]), function))
    } else {
        function
    }
}

fn parameters<'a>(node: &'a SyntaxNode<'a>) -> Box<[Node<'a>]> {
    let mut declarations = Vec::new();
    for child in node.children().iter().step_by(2)  {
        declarations.push(declaration(child));
    }

    declarations.into_boxed_slice()
}

fn array<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Array::new(expressions(&node.children()[1])))
}

fn group<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Group::new(expression(&node.children()[1])))
}

fn chain<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Chain::new(expression(&node.children()[0]), token(&node.children()[2])))
}

fn sequence<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Sequence::new(expression(&node.children()[0]), token(&node.children()[1]), expressions(&node.children()[2]), token(&node.children()[3])))
}

fn expressions<'a>(node: &'a SyntaxNode<'a>) -> Box<[Node<'a>]> {
    let mut expressions = Vec::new();
    for child in node.children().iter().step_by(2)  {
        expressions.push(expression(child));
    }

    expressions.into_boxed_slice()
}

fn assignment<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Assignment::new(expression(&node.children()[0]), expression(&node.children()[2]), token(&node.children()[1])))
}

fn operation<'a>(node: &'a SyntaxNode<'a>) -> Node<'a> {
    Node::new(node, Operation::new(expression(&node.children()[0]), expression(&node.children()[2]), token(&node.children()[1])))
}

fn token<'a>(node: &'a SyntaxNode<'a>) -> &'a str {
    node.text()
}
