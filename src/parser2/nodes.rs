use crate::elements;
use crate::node::Node;
use crate::parser2::arena::Arena;
use crate::parser2::rule::*;
use crate::parser2::Parser;

pub fn run<'a, 'b>(tokens: &Vec<Node<'a, 'b>>) -> Option<Node<'a, 'b>> {
	let mut rules = Arena::<&dyn Rule>::new();
	macro_rules! declare {
		( $name:ident ) => {
			let $name = rules.index();
		}
	}

	macro_rules! define {
		( $index:expr, $rule:expr ) => {
			let rule = $rule;
			rules.insert($index, &rule);
		};
	}

	macro_rules! token {
		( $name:ident, $element:expr ) => {
			declare!($name);
			define!($name, token($element));
		}
	}

	macro_rules! define_operation {
		( $name:ident, $child:ident, $tokens:expr ) => {
			declare!(expression_extension);
			declare!(expression_choice);
			define!($name, extension($child, expression_extension, &(|nodes| vec![Node::new_production(&elements::productions::EXPRESSION, nodes)])));
			define!(expression_extension, sequence(vec![expression_choice, $name]));
			define!(expression_choice, choice($tokens));
		}
	}

	token!(token_plus,       &elements::symbols::PLUS);
	token!(token_minus,      &elements::symbols::MINUS);
	token!(token_asterisk,   &elements::symbols::ASTERISK);
	token!(token_slash,      &elements::symbols::SLASH);
	token!(token_brace_l,    &elements::symbols::BRACE_L);
	token!(token_brace_r,    &elements::symbols::BRACE_R);
	token!(token_semicolon,  &elements::symbols::SEMICOLON);
	token!(token_if,         &elements::keywords::IF);
	token!(token_then,       &elements::keywords::THEN);
	token!(token_else,       &elements::keywords::ELSE);
	token!(token_loop,       &elements::keywords::LOOP);
	token!(token_while,      &elements::keywords::WHILE);
	token!(token_do,         &elements::keywords::DO);
	token!(token_number,     &elements::variables::NUMBER);
	token!(token_string,     &elements::variables::STRING);
	token!(token_identifier, &elements::variables::IDENTIFIER);

	declare!(program_element);
	declare!(statements_element);
	declare!(statements_list);
	declare!(statement_element);
	declare!(statement_sequence);
	declare!(expression_option);
	declare!(expression_element);
	declare!(expression_1);
	declare!(structure_element);
	declare!(structure_choice);
	declare!(block_element);
	declare!(block_sequence);
	declare!(if_element);
	declare!(if_sequence);
	declare!(if_body_element);
	declare!(if_body_choice);
	declare!(if_then_sequence);
	declare!(if_else_option);
	declare!(if_else_element);
	declare!(if_else_sequence);
	declare!(loop_element);
	declare!(loop_sequence);
	declare!(loop_body_element);
	declare!(loop_body_choice);
	declare!(loop_do_sequence);
	declare!(while_element);
	declare!(while_sequence);
	declare!(literal_element);
	declare!(literal_choice);

	declare!(expression_2);
	define_operation!(expression_2, expression_1, vec![token_plus, token_minus]);

	declare!(expression_3);
	define_operation!(expression_3, expression_2, vec![token_asterisk, token_slash]);

	define!(program_element, element(statements_element, &elements::productions::PROGRAM));
	define!(statements_element, element(statements_list, &elements::productions::STATEMENTS));
	define!(statements_list, list(statement_element));
	define!(statement_element, element(statement_sequence, &elements::productions::STATEMENT));
	define!(statement_sequence, sequence(vec![expression_element, token_semicolon]));
	define!(token_semicolon, token(&elements::symbols::SEMICOLON));
	define!(expression_option, option(expression_element));
	define!(expression_element, choice(vec![expression_3]));
	define!(expression_1, choice(vec![structure_element, literal_element]));
	define!(structure_element, element(structure_choice, &elements::structures::STRUCTURE));
	define!(structure_choice, choice(vec![block_element, if_element, loop_element, while_element]));
	define!(block_element, element(block_sequence, &elements::structures::BLOCK));
	define!(block_sequence, sequence(vec![token_brace_l, statements_element, expression_option, token_brace_r]));
	define!(if_element, element(if_sequence, &elements::structures::IF));
	define!(if_sequence, sequence(vec![token_if, expression_element, if_body_element, if_else_option]));
	define!(if_body_element, element(if_body_choice, &elements::structures::IF_BODY));
	define!(if_body_choice, choice(vec![if_then_sequence, block_element]));
	define!(if_then_sequence, sequence(vec![token_then, expression_element]));
	define!(if_else_option, option(if_else_element));
	define!(if_else_element, element(if_else_sequence, &elements::structures::IF_ELSE));
	define!(if_else_sequence, sequence(vec![token_else, expression_element]));
	define!(loop_element, element(loop_sequence, &elements::structures::LOOP));
	define!(loop_sequence, sequence(vec![token_loop, expression_element]));
	define!(loop_body_element, element(loop_body_choice, &elements::structures::LOOP_BODY));
	define!(loop_body_choice, choice(vec![loop_do_sequence, block_element]));
	define!(loop_do_sequence, sequence(vec![token_do, expression_element]));
	define!(while_element, element(while_sequence, &elements::structures::WHILE));
	define!(while_sequence, sequence(vec![token_while, expression_element, loop_body_element]));
	define!(literal_element, element(literal_choice, &elements::expressions::LITERAL));
	define!(literal_choice, choice(vec![token_number, token_string, token_identifier]));

	let program = &rules.get(program_element);
	let mut parser = Parser::new(tokens, rules);
	let node = if let Some(mut nodes) = program.rule(&mut parser) {
		nodes.pop()
	} else {
		return None;
	};

	return if parser.done() {
		node
	} else {
		None
	};
}
