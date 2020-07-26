use crate::elements;
use crate::node::Node;
use crate::parser2::arena::Arena;
use crate::parser2::matcher::*;
use crate::parser2::Parser;

pub fn run<'a, 'b>(tokens: &Vec<Node<'a, 'b>>) -> Option<Node<'a, 'b>> {
	let mut matchers = Arena::<&dyn Matcher>::new();
	macro_rules! declare {
		( $name:ident ) => {
			let $name = matchers.index();
		}
	}

	macro_rules! define {
		( $index:expr, $rule:expr ) => {
			let variable = $rule;
			matchers.insert($index, &variable);
		};
	}

	declare!(program_element);
	declare!(statements_element);
	declare!(statements_list);
	declare!(statement_element);
	declare!(statement_sequence);
	declare!(semicolon_token);
	declare!(expression_option);
	declare!(expression_element);
	declare!(expression_choice);
	declare!(structure_element);
	declare!(structure_choice);
	declare!(block_element);
	declare!(block_sequence);
	declare!(block_token_left);
	declare!(block_token_right);
	declare!(if_element);
	declare!(if_sequence);
	declare!(if_token);
	declare!(if_body_element);
	declare!(if_body_choice);
	declare!(if_then_sequence);
	declare!(if_then_token);
	declare!(if_else_option);
	declare!(if_else_element);
	declare!(if_else_sequence);
	declare!(if_else_token);
	declare!(loop_element);
	declare!(loop_sequence);
	declare!(loop_token);
	declare!(loop_body_element);
	declare!(loop_body_choice);
	declare!(loop_do_sequence);
	declare!(loop_do_token);
	declare!(while_element);
	declare!(while_sequence);
	declare!(while_token);
	declare!(literal_element);
	declare!(literal_choice);
	declare!(identifier_token);
	declare!(string_token);
	declare!(number_token);

	define!(program_element, element(statements_element, &elements::productions::PROGRAM));
	define!(statements_element, element(statements_list, &elements::productions::STATEMENTS));
	define!(statements_list, list(statement_element));
	define!(statement_element, element(statement_sequence, &elements::productions::STATEMENT));
	define!(statement_sequence, sequence(vec![expression_element, semicolon_token]));
	define!(semicolon_token, token(&elements::symbols::SEMICOLON));
	define!(expression_option, option(expression_element));
	define!(expression_element, element(expression_choice, &elements::productions::EXPRESSION));
	define!(expression_choice, choice(vec![structure_element, literal_element]));
	define!(structure_element, element(structure_choice, &elements::structures::STRUCTURE));
	define!(structure_choice, choice(vec![block_element, if_element, loop_element, while_element]));
	define!(block_element, element(block_sequence, &elements::structures::BLOCK));
	define!(block_sequence, sequence(vec![block_token_left, statements_element, expression_option, block_token_right]));
	define!(block_token_left, token(&elements::symbols::BRACE_L));
	define!(block_token_right, token(&elements::symbols::BRACE_R));
	define!(if_element, element(if_sequence, &elements::structures::IF));
	define!(if_sequence, sequence(vec![if_token, expression_element, if_body_element, if_else_option]));
	define!(if_token, token(&elements::keywords::IF));
	define!(if_body_element, element(if_body_choice, &elements::structures::IF_BODY));
	define!(if_body_choice, choice(vec![if_then_sequence, block_element]));
	define!(if_then_sequence, sequence(vec![if_then_token, expression_element]));
	define!(if_then_token, token(&elements::keywords::THEN));
	define!(if_else_option, option(if_else_element));
	define!(if_else_element, element(if_else_sequence, &elements::structures::IF_ELSE));
	define!(if_else_sequence, sequence(vec![if_else_token, expression_element]));
	define!(if_else_token, token(&elements::keywords::ELSE));
	define!(loop_element, element(loop_sequence, &elements::structures::LOOP));
	define!(loop_sequence, sequence(vec![loop_token, expression_element]));
	define!(loop_token, token(&elements::keywords::LOOP));
	define!(loop_body_element, element(loop_body_choice, &elements::structures::LOOP_BODY));
	define!(loop_body_choice, choice(vec![loop_do_sequence, block_element]));
	define!(loop_do_sequence, sequence(vec![loop_do_token, expression_element]));
	define!(loop_do_token, token(&elements::keywords::DO));
	define!(while_element, element(while_sequence, &elements::structures::WHILE));
	define!(while_sequence, sequence(vec![while_token, expression_element, loop_body_element]));
	define!(while_token, token(&elements::keywords::WHILE));
	define!(literal_element, element(literal_choice, &elements::expressions::LITERAL));
	define!(literal_choice, choice(vec![number_token, string_token, identifier_token]));
	define!(identifier_token, token(&elements::variables::IDENTIFIER));
	define!(string_token, token(&elements::variables::STRING));
	define!(number_token, token(&elements::variables::NUMBER));

	return if let Some(mut nodes) = matchers.get(program_element).go(&mut Parser::new(tokens, matchers)) {
		nodes.pop()
	} else {
		None
	};
}
