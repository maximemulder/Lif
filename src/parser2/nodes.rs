use crate::elements;
use crate::node::Node;
use crate::parser2::matcher::*;
use crate::parser2::Parser;

pub fn run<'a, 'b>(tokens: &Vec<Node<'a, 'b>>) -> Option<Node<'a, 'b>> {

	let matchers: [&dyn Matcher; 18] = [
		&token(&elements::variables::NUMBER),
		&token(&elements::variables::STRING),
		&token(&elements::variables::IDENTIFIER),
		&choice(&[0, 1, 2]),
		&element(3, &elements::expressions::LITERAL),
		&token(&elements::keywords::IF),
		&sequence(&[5, 11]),
		&element(6, &elements::structures::IF),
		&choice(&[7]),
		&element(8, &elements::structures::STRUCTURE),
		&choice(&[9, 4]),
		&element(10, &elements::productions::EXPRESSION),
		&token(&elements::symbols::SEMICOLON),
		&sequence(&[11, 12]),
		&element(13, &elements::productions::STATEMENT),
		&list(14),
		&element(15, &elements::productions::STATEMENTS),
		&element(16, &elements::productions::PROGRAM),
	 ];

	return if let Some(mut nodes) = matchers[17].go(&mut Parser::new(tokens, &matchers)) {
		nodes.pop()
	} else {
		None
	};
}

/*
pub fn run<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {

	let none = token(&elements::ignores::WHITESPACE);

	let number_token = token(&elements::variables::NUMBER);

	let string_token = token(&elements::variables::STRING);

	let identifier_token = token(&elements::variables::IDENTIFIER);

	let literal_array: [&dyn Matcher; 3] = [&number_token, &string_token, &identifier_token];

	let literal_choice = choice(&literal_array);

	let literal_element = element(&literal_choice, &elements::expressions::LITERAL);

	let if_token = token(&elements::keywords::IF);

	let if_array: [&dyn Matcher; 2] = [&if_token, &none];

	let if_sequence = sequence(&if_array);

	let if_element = element(&if_sequence, &elements::structures::IF);

	let structure_array: [&dyn Matcher; 1] = [&if_element];

	let structure_choice = choice(&structure_array);

	let structure_element = element(&structure_choice, &elements::structures::STRUCTURE);

	let expression_array: [&dyn Matcher; 2] = [&structure_element, &literal_element];

	let expression_choice = choice(&expression_array);

	let expression_element = element(&expression_choice, &elements::productions::EXPRESSION);

	let semicolon_token = token(&elements::symbols::SEMICOLON);

	let statement_array: [&dyn Matcher; 2] = [&expression_element, &semicolon_token];

	let statement_sequence = sequence(&statement_array);

	let statement_element = element(&statement_sequence, &elements::productions::STATEMENT);

	let statements_list = list(&statement_element);

	let statements_element = element(&statements_list, &elements::productions::STATEMENTS);

	let program_element = element(&statements_element, &elements::productions::PROGRAM);

	return if let Some(mut nodes) = program_element.go(parser) {
		nodes.pop()
	} else {
		None
	};
}
*/