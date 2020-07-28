use crate::elements;
use crate::node::Node;
use crate::parser2::arena::Arena;
use crate::parser2::rule::*;
use crate::parser2::Parser;

pub fn run<'a, 'b>(tokens: &Vec<Node<'a, 'b>>) -> Option<Node<'a, 'b>> {
	let mut rules = Arena::<dyn Rule>::new();
	macro_rules! declare {
		( $name:ident ) => {
			let $name = rules.index();
		}
	}

	macro_rules! define {
		( $index:expr, $rule:expr ) => {
			rules.insert($index, $rule);
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
			define!($name, extension($child, expression_extension, &(|nodes| vec![
				Node::new_production(&elements::productions::EXPRESSION, vec![
					Node::new_production(&elements::expressions::OPERATION, nodes)
				])
			])));
			define!(expression_extension, sequence(vec![expression_choice, $name]));
			define!(expression_choice, choice($tokens));
		}
	}

	token!(token_plus,       &elements::symbols::PLUS);
	token!(token_minus,      &elements::symbols::MINUS);
	token!(token_asterisk,   &elements::symbols::ASTERISK);
	token!(token_slash,      &elements::symbols::SLASH);
	token!(token_percent,    &elements::symbols::PERCENT);
	token!(token_asterisk_d, &elements::symbols::ASTERISK_D);
	token!(guillemet_l_d,    &elements::symbols::GUILLEMET_L_D);
	token!(guillemet_r_d,    &elements::symbols::GUILLEMET_R_D);
	token!(guillemet_l_t,    &elements::symbols::GUILLEMET_L_T);
	token!(guillemet_r_t,    &elements::symbols::GUILLEMET_R_T);
	token!(guillemet_l,      &elements::symbols::GUILLEMET_L);
	token!(guillemet_r,      &elements::symbols::GUILLEMET_R);
	token!(guillemet_l_eq,   &elements::symbols::GUILLEMET_L_EQ);
	token!(guillemet_r_eq,   &elements::symbols::GUILLEMET_R_EQ);
	token!(equal_d,          &elements::symbols::EQUAL_D);
	token!(exclamation_eq,   &elements::symbols::EXCLAMATION_EQ);
	token!(ampersand,        &elements::symbols::AMPERSAND);
	token!(caret,            &elements::symbols::CARET);
	token!(pipe,             &elements::symbols::PIPE);
	token!(ampersand_d,      &elements::symbols::AMPERSAND_D);
	token!(pipe_d,           &elements::symbols::PIPE_D);
	token!(dot_d,            &elements::symbols::DOT_D);
	token!(dot_d_eq,         &elements::symbols::DOT_D_EQ);
	token!(equal,            &elements::symbols::EQUAL);
	token!(plus_eq,          &elements::symbols::PLUS_EQ);
	token!(minus_eq,         &elements::symbols::MINUS_EQ);
	token!(asterisk_eq,      &elements::symbols::ASTERISK_EQ);
	token!(slash_eq,         &elements::symbols::SLASH_EQ);
	token!(percent_eq,       &elements::symbols::PERCENT_EQ);
	token!(asterisk_d_eq,    &elements::symbols::ASTERISK_D_EQ);
	token!(guillemet_l_d_eq, &elements::symbols::GUILLEMET_L_D_EQ);
	token!(guillemet_r_d_eq, &elements::symbols::GUILLEMET_R_D_EQ);
	token!(guillemet_l_t_eq, &elements::symbols::GUILLEMET_L_T_EQ);
	token!(guillemet_r_t_eq, &elements::symbols::GUILLEMET_R_T_EQ);
	token!(ampersand_eq,     &elements::symbols::AMPERSAND_EQ);
	token!(caret_eq,         &elements::symbols::CARET_EQ);
	token!(pipe_eq,          &elements::symbols::PIPE_EQ);
	token!(ampersand_d_eq,   &elements::symbols::AMPERSAND_D_EQ);
	token!(pipe_d_eq,        &elements::symbols::PIPE_D_EQ);
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
	declare!(expression_1_element);
	declare!(expression_1_choice);
	declare!(expression_2);
	declare!(expression_3);
	declare!(expression_4);
	declare!(expression_5);
	declare!(expression_6);
	declare!(expression_7);
	declare!(expression_8);
	declare!(expression_9);
	declare!(expression_10);
	declare!(expression_11);
	declare!(expression_12);
	declare!(expression_13);
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

	define!(program_element, element(statements_element, &elements::productions::PROGRAM));
	define!(statements_element, element(statements_list, &elements::productions::STATEMENTS));
	define!(statements_list, list(statement_element));
	define!(statement_element, element(statement_sequence, &elements::productions::STATEMENT));
	define!(statement_sequence, sequence(vec![expression_element, token_semicolon]));
	define!(token_semicolon, token(&elements::symbols::SEMICOLON));
	define!(expression_option, option(expression_element));
	define!(expression_element, choice(vec![expression_13]));
	define!(expression_1_element, element(expression_1_choice, &elements::productions::EXPRESSION));
	define!(expression_1_choice, choice(vec![structure_element, literal_element]));
	define_operation!(expression_2, expression_1_element, vec![token_asterisk, token_slash, token_percent, token_asterisk_d]);
	define_operation!(expression_3, expression_2, vec![token_plus, token_minus]);
	define_operation!(expression_4, expression_3, vec![guillemet_l_d, guillemet_r_d, guillemet_l_t, guillemet_l_t]);
	define_operation!(expression_5, expression_4, vec![ampersand]);
	define_operation!(expression_6, expression_5, vec![caret]);
	define_operation!(expression_7, expression_6, vec![pipe]);
	define_operation!(expression_8, expression_7, vec![guillemet_l, guillemet_r, guillemet_l_eq, guillemet_l_eq]);
	define_operation!(expression_9, expression_8, vec![equal_d, exclamation_eq]);
	define_operation!(expression_10, expression_9, vec![ampersand_d]);
	define_operation!(expression_11, expression_10, vec![pipe_d]);
	define_operation!(expression_12, expression_11, vec![dot_d, dot_d_eq]);
	define_operation!(expression_13, expression_12, vec![equal, plus_eq, minus_eq, asterisk_eq, slash_eq, percent_eq, asterisk_d_eq, guillemet_l_d_eq, guillemet_r_d_eq, guillemet_l_t_eq, guillemet_r_t_eq, ampersand_eq, caret_eq, pipe_eq, ampersand_d_eq, pipe_d_eq]);
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
	let mut parser = Parser::new(tokens, &rules);
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
