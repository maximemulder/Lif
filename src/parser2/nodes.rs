use crate::elements;
use crate::node::Node;
use crate::parser2::matcher::*;
use crate::parser2::Parser;

/*pub fn production<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Vec<Node<'a, 'b>>> {
	use crate::elements;
	let a = token(&elements::variables::IDENTIFIER);
	let b = element(&a, &elements::expressions::GROUP);
	let c = optional(&b);
	let d: [&dyn Matcher; 2] = [&token(&elements::keywords::DO), &c];
	let e = sequence(&d);
	let f = element(&e, &elements::productions::PROGRAM);
	return f.go(parser);
	/* let a = element(
		&sequence(&[
			&token(&elements::keywords::DO),
			&optional(
				&element(
					&token(&elements::variables::IDENTIFIER),
					&elements::expressions::GROUP
				)
			)
		]),
		&elements::productions::PROGRAM,
	);
	return a.go(parser); */
}*/

/*pub fn run<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {
	return if let Some(nodes) = PROGRAM_ELEMENT.go(parser) {
		nodes.pop()
	} else {
		None
	};
}*/

pub fn run<'a, 'b>(parser: &mut Parser<'a, 'b, '_>) -> Option<Node<'a, 'b>> {

	let number_token = token(&elements::variables::NUMBER);

	let string_token = token(&elements::variables::STRING);

	let identifier_token = token(&elements::variables::IDENTIFIER);

	let literal_array: [&dyn Matcher; 3] = [&number_token, &string_token, &identifier_token];

	let literal_choice = choice(&literal_array);

	let literal_element = element(&literal_choice, &elements::expressions::LITERAL);

	let expression_array: [&dyn Matcher; 1] = [&literal_element];

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
