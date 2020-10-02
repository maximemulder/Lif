use crate::elements;
use crate::nodes::{ Node, SyntaxNode };
use crate::nodes::expression::Expression;
use crate::nodes::program::Program;
use crate::nodes::r#if::If;
use crate::nodes::r#loop::Loop;
use crate::nodes::r#while::While;
use crate::nodes::do_while::DoWhile;
use crate::nodes::for_in::ForIn;
use crate::nodes::statement::Statement;
use crate::nodes::statements::Statements;
use crate::nodes::structure::Structure;
use crate::nodes::operation::Operation;
use crate::nodes::chain::Chain;
use crate::nodes::method::Method;
use crate::nodes::sequence::Sequence;
use crate::nodes::declaration::Declaration;
use crate::nodes::generic::Generic;
use crate::nodes::function::Function;
use crate::nodes::array::Array;
use crate::nodes::block::Block;
use crate::nodes::group::Group;
use crate::nodes::integer::Integer;
use crate::nodes::identifier::Identifier;
use crate::nodes::string::String;
use crate::nodes::r#return::Return;
use crate::nodes::r#break::Break;
use crate::nodes::r#continue::Continue;

pub fn program<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Program<'a> {
	return Program::new(node, statements(text, &node.children()[0]));
}

fn statements<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Statements<'a> {
	let mut statements = Vec::new();
	for child in node.children() {
		statements.push(statement(text, child));
	}

	return Statements::new(node, statements);
}

fn statement<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Statement<'a> {
	let child = &node.children()[0];
	return Statement::new(node, match child.element {
		&elements::expressions::EXPRESSION => Box::new(expression(text, child)),
		&elements::structures::STRUCTURE   => Box::new(structure(text, child)),
		_ => panic!(),
	});
}

fn expression<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Expression<'a> {
	let child = &node.children()[0];
	return Expression::new(node, match child.element {
		&elements::expressions::LITERAL     => literal(text, child),
		&elements::structures::STRUCTURE    => Box::new(structure(text, child)),
		&elements::expressions::LET         => Box::new(r#let(text, child)),
		&elements::controls::CONTROL        => control(text, child),
		&elements::expressions::FUNCTION    => function(text, child),
		&elements::expressions::GROUP       => Box::new(group(text, child)),
		&elements::expressions::CHAIN       => Box::new(chain(text, child)),
		&elements::expressions::ARRAY       => Box::new(array(text, child)),
		&elements::expressions::METHOD      => Box::new(method(text, child)),
		&elements::expressions::SEQUENCE    => Box::new(sequence(text, child)),
		&elements::expressions::OPERATION   => Box::new(operation(text, child)),
		_ => panic!(),
	});
}

fn literal<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Box<dyn Node<'a> + 'a> {
	let child = &node.children()[0];
	return match child.element {
		&elements::variables::NUMBER     => Box::new(integer(text, child)),
		&elements::variables::STRING     => Box::new(string(text, child)),
		&elements::variables::IDENTIFIER => Box::new(identifier(text, child)),
		_ => panic!(),
	};
}

fn integer<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Integer<'a> {
	return Integer::new(node, text[node.left() .. node.right()].parse::<usize>().unwrap());
}

fn string<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> String<'a> {
	return String::new(node, &text[node.left() + 1 .. node.right() - 1]);
}

fn identifier<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Identifier<'a> {
	return Identifier::new(node, &text[node.left() .. node.right()]);
}

fn structure<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Structure<'a> {	let child = &node.children()[0];
	return Structure::new(node, match child.element {
		&elements::structures::BLOCK    => Box::new(block(text, child)),
		&elements::structures::IF       => Box::new(r#if(text, child)),
		&elements::structures::LOOP     => Box::new(r#loop(text, child)),
		&elements::structures::WHILE    => Box::new(r#while(text, child)),
		&elements::structures::DO_WHILE => Box::new(do_while(text, child)),
		&elements::structures::FOR_IN   => Box::new(for_in(text, child)),
		_ => panic!(),
	});
}

fn block<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Block<'a> {
	return Block::new(node, statements(text, &node.children()[1]), if node.children().len() == 4 {
		Some(expression(text, &node.children()[2]))
	} else {
		None
	});
}

fn r#if<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> If<'a> {
	return If::new(node, expression(text, &node.children()[1]), block(text, &node.children()[2]), node.children().get(4).map(|child| block(text, child)));
}

fn r#loop<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Loop<'a> {
	return Loop::new(node, block(text, &node.children()[1]));
}

fn r#while<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> While<'a> {
	return While::new(node, expression(text, &node.children()[1]), block(text, &node.children()[2]));
}

fn do_while<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> DoWhile<'a> {
	return DoWhile::new(node, block(text, &node.children()[1]), expression(text, &node.children()[3]));
}

fn for_in<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> ForIn<'a> {
	return ForIn::new(node, token(text, &node.children()[1]), expression(text, &node.children()[3]), block(text, &node.children()[4]));
}

fn r#let<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Declaration<'a> {
	return declaration(text, &node.children()[1]);
}

fn declaration<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Declaration<'a> {
	return Declaration::new(node, token(text, &node.children()[0]), node.children().get(2).map(|child| expression(text, child)));
}

fn control<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Box<dyn Node<'a> + 'a> {
	let child = &node.children()[0];
	return match child.element {
		&elements::controls::RETURN   => Box::new(r#return(text, child)),
		&elements::controls::BREAK    => Box::new(r#break(text, child)),
		&elements::controls::CONTINUE => Box::new(r#continue(text, child)),
		_ => panic!(),
	};
}

fn r#return<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Return<'a> {
	return Return::new(node, node.children().get(1).map(|child| expression(text, child)));
}

fn r#break<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Break<'a> {
	return Break::new(node, node.children().get(1).map(|child| expression(text, child)));
}

fn r#continue<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Continue<'a> {
	return Continue::new(node, node.children().get(1).map(|child| expression(text, child)));
}

fn generics<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Vec<&'a str> {
	let mut identifiers = Vec::new();
	for (i, child) in node.children().iter().enumerate()  {
		if i % 2 == 1 {
			continue;
		}

		identifiers.push(token(text, child));
	}

	return identifiers;
}

fn function<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Box<dyn Node<'a> + 'a> {
	let children = node.children();
	let function = Box::new(Function::new(node, parameters(text, &children[if children.len() < 8 {
		2
	} else {
		5
	}]), if children[children.len() - 2].element == &elements::expressions::EXPRESSION {
		Some(expression(text, &children[children.len() - 2]))
	} else {
		None
	}, block(text, &children.last().unwrap())));

	return if children.len() >= 8 {
		Box::new(Generic::new(node, generics(text, &children[2]), function))
	} else {
		function
	};
}

fn parameters<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Vec<Declaration<'a>> {
	let mut declarations = Vec::new();
	for (i, child) in node.children().iter().enumerate()  {
		if i % 2 == 1 {
			continue;
		}

		declarations.push(declaration(text, child));
	}

	return declarations;
}

fn array<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Array<'a> {
	return Array::new(node, expressions(text, &node.children()[1]));
}

fn group<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Group<'a> {
	return Group::new(node, expression(text, &node.children()[1]));
}

fn chain<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Chain<'a> {
	return Chain::new(node, expression(text, &node.children()[0]), token(text, &node.children()[2]));
}

fn method<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Method<'a> {
	return Method::new(node, expression(text, &node.children()[0]), token(text, &node.children()[2]), expressions(text, &node.children()[4]));
}

fn sequence<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Sequence<'a> {
	return Sequence::new(node, expression(text, &node.children()[0]), token(text, &node.children()[1]), expressions(text, &node.children()[2]), token(text, &node.children()[3]));
}

fn expressions<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Vec<Expression<'a>> {
	let mut expressions = Vec::new();
	for (i, child) in node.children().iter().enumerate()  {
		if i % 2 == 1 {
			continue;
		}

		expressions.push(expression(text, child));
	}

	return expressions;
}

fn operation<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> Operation<'a> {
	return Operation::new(node, expression(text, &node.children()[0]), expression(text, &node.children()[2]), token(text, &node.children()[1]));
}

fn token<'a>(text: &'a str, node: &'a SyntaxNode<'a>) -> &'a str {
	return &text[node.left() .. node.right()];
}
