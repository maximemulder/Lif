use crate::elements;
use crate::node::Node;
use crate::parser2::arena::Arena;
use crate::parser2::filters::*;
use crate::parser2::rules::*;
use crate::parser2::Parser;

pub fn run<'a, 'b>(tokens: &Vec<Node<'a, 'b>>) -> Option<Node<'a, 'b>> {
	let mut rules = Arena::<dyn Rule>::new();
	let mut filters = Arena::<dyn Filter>::new();

	macro_rules! declare {
		( $name:ident ) => {
			let $name = rules.reserve();
		}
	}

	macro_rules! define {
		( $index:expr, $rule:expr ) => {
			rules.insert($index, $rule);
		};
	}

	macro_rules! create {
		( $rule:expr ) => {
			rules.push($rule)
		}
	}

	macro_rules! token {
		( $name:ident, $element:expr ) => {
			let $name = create!(token($element));
		}
	}

	macro_rules! element {
		( $rule:expr, $element: expr ) => {
			filter($rule, filters.push(FilterElement { element: $element }))
		};
	}

	token!(keyword_as,              &elements::keywords::AS);
	token!(keyword_catch,           &elements::keywords::CATCH);
	token!(keyword_class,           &elements::keywords::CLASS);
	token!(keyword_default,         &elements::keywords::DEFAULT);
	token!(keyword_do,              &elements::keywords::DO);
	token!(keyword_else,            &elements::keywords::ELSE);
	token!(keyword_export,          &elements::keywords::EXPORT);
	token!(keyword_finally,         &elements::keywords::FINALLY);
	token!(keyword_for,             &elements::keywords::FOR);
	token!(keyword_from,            &elements::keywords::FROM);
	token!(keyword_function,        &elements::keywords::FUNCTION);
	token!(keyword_if,              &elements::keywords::IF);
	token!(keyword_import,          &elements::keywords::IMPORT);
	token!(keyword_in,              &elements::keywords::IN);
	token!(keyword_let,             &elements::keywords::LET);
	token!(keyword_loop,            &elements::keywords::LOOP);
	token!(keyword_private,         &elements::keywords::PRIVATE);
	token!(keyword_protected,       &elements::keywords::PROTECTED);
	token!(keyword_public,          &elements::keywords::PUBLIC);
	token!(keyword_continue,        &elements::keywords::CONTINUE);
	token!(keyword_break,           &elements::keywords::BREAK);
	token!(keyword_return,          &elements::keywords::RETURN);
	token!(keyword_static,          &elements::keywords::STATIC);
	token!(keyword_then,            &elements::keywords::THEN);
	token!(keyword_throw,           &elements::keywords::THROW);
	token!(keyword_try,             &elements::keywords::TRY);
	token!(keyword_while,           &elements::keywords::WHILE);
	token!(symbol_plus,             &elements::symbols::PLUS);
	token!(symbol_plus_eq,          &elements::symbols::PLUS_EQ);
	token!(symbol_minus,            &elements::symbols::MINUS);
	token!(symbol_minus_eq,         &elements::symbols::MINUS_EQ);
	token!(symbol_asterisk,         &elements::symbols::ASTERISK);
	token!(symbol_asterisk_eq,      &elements::symbols::ASTERISK_EQ);
	token!(symbol_asterisk_d,       &elements::symbols::ASTERISK_D);
	token!(symbol_asterisk_d_eq,    &elements::symbols::ASTERISK_D_EQ);
	token!(symbol_slash,            &elements::symbols::SLASH);
	token!(symbol_slash_eq,         &elements::symbols::SLASH_EQ);
	token!(symbol_percent,          &elements::symbols::PERCENT);
	token!(symbol_percent_eq,       &elements::symbols::PERCENT_EQ);
	token!(symbol_caret,            &elements::symbols::CARET);
	token!(symbol_caret_eq,         &elements::symbols::CARET_EQ);
	token!(symbol_exclamation,      &elements::symbols::EXCLAMATION);
	token!(symbol_exclamation_eq,   &elements::symbols::EXCLAMATION_EQ);
	token!(symbol_equal,            &elements::symbols::EQUAL);
	token!(symbol_equal_d,          &elements::symbols::EQUAL_D);
	token!(symbol_arrow,            &elements::symbols::ARROW);
	token!(symbol_pipe,             &elements::symbols::PIPE);
	token!(symbol_pipe_eq,          &elements::symbols::PIPE_EQ);
	token!(symbol_pipe_d,           &elements::symbols::PIPE_D);
	token!(symbol_pipe_d_eq,        &elements::symbols::PIPE_D_EQ);
	token!(symbol_ampersand,        &elements::symbols::AMPERSAND);
	token!(symbol_ampersand_eq,     &elements::symbols::AMPERSAND_EQ);
	token!(symbol_ampersand_d,      &elements::symbols::AMPERSAND_D);
	token!(symbol_ampersand_d_eq,   &elements::symbols::AMPERSAND_D_EQ);
	token!(symbol_guillemet_l,      &elements::symbols::GUILLEMET_L);
	token!(symbol_guillemet_r,      &elements::symbols::GUILLEMET_R);
	token!(symbol_guillemet_l_eq,   &elements::symbols::GUILLEMET_L_EQ);
	token!(symbol_guillemet_r_eq,   &elements::symbols::GUILLEMET_R_EQ);
	token!(symbol_guillemet_l_d,    &elements::symbols::GUILLEMET_L_D);
	token!(symbol_guillemet_r_d,    &elements::symbols::GUILLEMET_R_D);
	token!(symbol_guillemet_l_d_eq, &elements::symbols::GUILLEMET_L_D_EQ);
	token!(symbol_guillemet_r_d_eq, &elements::symbols::GUILLEMET_R_D_EQ);
	token!(symbol_guillemet_l_t,    &elements::symbols::GUILLEMET_L_T);
	token!(symbol_guillemet_r_t,    &elements::symbols::GUILLEMET_R_T);
	token!(symbol_guillemet_l_t_eq, &elements::symbols::GUILLEMET_L_T_EQ);
	token!(symbol_guillemet_r_t_eq, &elements::symbols::GUILLEMET_R_T_EQ);
	token!(symbol_parenthesis_l,    &elements::symbols::PARENTHESIS_L);
	token!(symbol_parenthesis_r,    &elements::symbols::PARENTHESIS_R);
	token!(symbol_brace_l,          &elements::symbols::BRACE_L);
	token!(symbol_brace_r,          &elements::symbols::BRACE_R);
	token!(symbol_crotchet_l,       &elements::symbols::CROTCHET_L);
	token!(symbol_crotchet_r,       &elements::symbols::CROTCHET_R);
	token!(symbol_dot,              &elements::symbols::DOT);
	token!(symbol_dot_d,            &elements::symbols::DOT_D);
	token!(symbol_dot_d_eq,         &elements::symbols::DOT_D_EQ);
	token!(symbol_dot_t,            &elements::symbols::DOT_T);
	token!(symbol_comma,            &elements::symbols::COMMA);
	token!(symbol_tilde,            &elements::symbols::TILDE);
	token!(symbol_dollar,           &elements::symbols::DOLLAR);
	token!(symbol_interrogation,    &elements::symbols::INTERROGATION);
	token!(symbol_colon,            &elements::symbols::COLON);
	token!(symbol_semicolon,        &elements::symbols::SEMICOLON);
	token!(symbol_backslash,        &elements::symbols::BACKSLASH);
	token!(variable_identifier,     &elements::variables::IDENTIFIER);
	token!(variable_string,         &elements::variables::STRING);
	token!(variable_number,         &elements::variables::NUMBER);

	declare!(program_element);
	declare!(statements_element);
	declare!(statements_list);
	declare!(statement_element);
	declare!(statement_sequence);
	declare!(expression);
	declare!(expression_option);
	declare!(expression_list);
	declare!(expression_element);
	declare!(expression_choice);
	declare!(function_element);
	declare!(function_sequence);
	declare!(parameters);
	declare!(call);
	declare!(call_choice);
	declare!(call_sequence_1);
	declare!(call_sequence_2);
	declare!(operation_1);
	declare!(operation_2);
	declare!(operation_3);
	declare!(operation_4);
	declare!(operation_5);
	declare!(operation_6);
	declare!(operation_7);
	declare!(operation_8);
	declare!(operation_9);
	declare!(operation_10);
	declare!(operation_11);
	declare!(operation_12);
	declare!(group_element);
	declare!(group_sequence);
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
	declare!(do_while_element);
	declare!(do_while_sequence);
	declare!(for_in_element);
	declare!(for_in_sequence);
	declare!(declaration_element);
	declare!(declaration_sequence);
	declare!(control_element);
	declare!(control_choice);
	declare!(return_element);
	declare!(break_element);
	declare!(continue_element);
	declare!(literal_element);
	declare!(literal_choice);

	define!(program_element, element!(statements_element, &elements::productions::PROGRAM));
	define!(statements_element, element!(statements_list, &elements::productions::STATEMENTS));
	define!(statements_list, list(statement_element));
	define!(statement_element, element!(statement_sequence, &elements::productions::STATEMENT));
	define!(statement_sequence, sequence(vec![expression, symbol_semicolon]));

	define!(expression, alias(operation_12));
	define!(expression_option, option(expression));

	macro_rules! define_list {
		( $name:ident, $node:expr, $element:expr ) => {
			declare!(node_option);
			declare!(node_sequence);
			declare!(more_option);
			declare!(more_sequence);
			define!($name, element!(node_option, $element));
			define!(node_option, option(node_sequence));
			define!(node_sequence, sequence(vec![$node, more_option]));
			define!(more_option, option(more_sequence));
			define!(more_sequence, sequence(vec![symbol_comma, $node, more_option]));
		};
	}

	define_list!(expression_list, expression, &elements::productions::EXPRESSIONS);

	define!(expression_element, element!(expression_choice, &elements::productions::EXPRESSION));
	define!(expression_choice, choice(vec![function_element, structure_element, declaration_element, control_element, group_element, literal_element]));

	define!(function_element,  element!(function_sequence, &elements::expressions::FUNCTION));
	define!(function_sequence, sequence(vec![keyword_function, symbol_parenthesis_l, parameters, symbol_parenthesis_r, block_element]));
	define_list!(parameters, variable_identifier, &elements::productions::PARAMETERS);

	define!(structure_element, element!(structure_choice, &elements::structures::STRUCTURE));
	define!(structure_choice, choice(vec![block_element, if_element, loop_element, while_element, do_while_element, for_in_element]));
	define!(block_element, element!(block_sequence, &elements::structures::BLOCK));
	define!(block_sequence, sequence(vec![symbol_brace_l, statements_element, expression_option, symbol_brace_r]));
	define!(if_element, element!(if_sequence, &elements::structures::IF));
	define!(if_sequence, sequence(vec![keyword_if, expression, if_body_element, if_else_option]));
	define!(if_body_element, element!(if_body_choice, &elements::structures::IF_BODY));
	define!(if_body_choice, choice(vec![if_then_sequence, block_element]));
	define!(if_then_sequence, sequence(vec![keyword_then, expression]));
	define!(if_else_option, option(if_else_element));
	define!(if_else_element, element!(if_else_sequence, &elements::structures::IF_ELSE));
	define!(if_else_sequence, sequence(vec![keyword_else, expression]));
	define!(loop_element, element!(loop_sequence, &elements::structures::LOOP));
	define!(loop_sequence, sequence(vec![keyword_loop, expression]));
	define!(loop_body_element, element!(loop_body_choice, &elements::structures::LOOP_BODY));
	define!(loop_body_choice, choice(vec![loop_do_sequence, block_element]));
	define!(loop_do_sequence, sequence(vec![keyword_do, expression]));
	define!(while_element, element!(while_sequence, &elements::structures::WHILE));
	define!(while_sequence, sequence(vec![keyword_while, expression, loop_body_element]));
	define!(do_while_element, element!(while_sequence, &elements::structures::DO_WHILE));
	define!(do_while_sequence, sequence(vec![keyword_do, expression, keyword_while, expression]));
	define!(for_in_element, element!(for_in_sequence, &elements::structures::FOR_IN));
	define!(for_in_sequence, sequence(vec![keyword_for, variable_identifier, keyword_in, expression]));

	macro_rules! define_control {
		( $name:ident, $keyword:expr, $element:expr ) => {
			declare!(control_sequence);
			define!($name, element!(control_sequence, $element));
			define!(control_sequence, sequence(vec![$keyword, expression_option]));
		}
	}

	define!(declaration_element,  element!(declaration_sequence, &elements::expressions::DECLARATION));
	define!(declaration_sequence, sequence(vec![keyword_let, variable_identifier]));

	define!(control_element, element!(control_choice, &elements::controls::CONTROL));
	define!(control_choice,  choice(vec![return_element, break_element, continue_element]));
	define_control!(return_element,   keyword_return,   &elements::controls::RETURN);
	define_control!(break_element,    keyword_break,    &elements::controls::BREAK);
	define_control!(continue_element, keyword_continue, &elements::controls::CONTINUE);

	define!(group_element,  element!(group_sequence, &elements::expressions::GROUP));
	define!(group_sequence, sequence(vec![symbol_parenthesis_l, expression, symbol_parenthesis_r]));
	define!(literal_element, element!(literal_choice, &elements::expressions::LITERAL));
	define!(literal_choice, choice(vec![variable_identifier, variable_string, variable_number]));

	macro_rules! define_operation {
		( $name:ident, $child:expr, $tokens:expr ) => {
			declare!(operation_sequence);
			declare!(operation_choice);
			define!($name, recurse($child, operation_sequence, &(|nodes| vec![
				Node::new_production(&elements::productions::EXPRESSION, vec![
					Node::new_production(&elements::expressions::OPERATION, nodes)
				])
			])));

			define!(operation_sequence, sequence(vec![operation_choice, $child]));
			define!(operation_choice, choice($tokens));
		}
	}

	define_operation!(operation_1,  call,         vec![symbol_asterisk, symbol_slash, symbol_percent, symbol_asterisk_d]);
	define_operation!(operation_2,  operation_1,  vec![symbol_plus, symbol_minus]);
	define_operation!(operation_3,  operation_2,  vec![symbol_guillemet_l_d, symbol_guillemet_r_d, symbol_guillemet_l_t, symbol_guillemet_l_t]);
	define_operation!(operation_4,  operation_3,  vec![symbol_ampersand]);
	define_operation!(operation_5,  operation_4,  vec![symbol_caret]);
	define_operation!(operation_6,  operation_5,  vec![symbol_pipe]);
	define_operation!(operation_7,  operation_6,  vec![symbol_guillemet_l, symbol_guillemet_r, symbol_guillemet_l_eq, symbol_guillemet_l_eq]);
	define_operation!(operation_8,  operation_7,  vec![symbol_equal_d, symbol_exclamation_eq]);
	define_operation!(operation_9,  operation_8,  vec![symbol_ampersand_d]);
	define_operation!(operation_10, operation_9,  vec![symbol_pipe_d]);
	define_operation!(operation_11, operation_10, vec![symbol_dot_d, symbol_dot_d_eq]);
	define_operation!(operation_12, operation_11, vec![symbol_equal, symbol_plus_eq, symbol_minus_eq, symbol_asterisk_eq, symbol_slash_eq,
		symbol_percent_eq, symbol_asterisk_d_eq, symbol_guillemet_l_d_eq, symbol_guillemet_r_d_eq, symbol_guillemet_l_t_eq, symbol_guillemet_r_t_eq,
		symbol_ampersand_eq, symbol_caret_eq, symbol_pipe_eq, symbol_ampersand_d_eq, symbol_pipe_d_eq
	]);

	define!(call, recurse(expression_element, call_choice, &(|nodes|  vec![
		Node::new_production(&elements::productions::EXPRESSION, vec![
			Node::new_production(&elements::expressions::SEQUENCE, nodes)
		])
	])));

	define!(call_choice, choice(vec![call_sequence_1, call_sequence_2]));
	define!(call_sequence_1, sequence(vec![symbol_parenthesis_l, expression_list, symbol_parenthesis_r]));
	define!(call_sequence_2, sequence(vec![symbol_crotchet_l, expression_list, symbol_crotchet_r]));

	let program = &rules.get(program_element);
	let mut parser = Parser::new(tokens, &rules, &filters);
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
