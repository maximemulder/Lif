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
	declare!(expression_element);
	declare!(expression_choice);
	declare!(structure_element);
	declare!(structure_choice);
	declare!(if_token);
	declare!(if_sequence);
	declare!(if_element);
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
	define!(expression_element, element(expression_choice, &elements::productions::EXPRESSION));
	define!(expression_choice, choice(vec![structure_element, literal_element]));
	define!(structure_element, element(structure_choice, &elements::structures::STRUCTURE));
	define!(structure_choice, choice(vec![if_element]));
	define!(if_element, element(if_sequence, &elements::structures::IF));
	define!(if_sequence, sequence(vec![if_token, expression_element]));
	define!(if_token, token(&elements::keywords::IF));
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
