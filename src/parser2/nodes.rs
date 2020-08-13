use crate::elements;
use crate::node::Node;
use crate::parser2::arena::Arena;
use crate::parser2::filters::*;
use crate::parser2::rules::*;
use crate::parser2::Parser;

pub fn run<'a, 'b>(tokens: &Vec<Node<'a, 'b>>) -> Option<Node<'a, 'b>> {
	let rules = Arena::<dyn Rule>::new();
	let filters = Arena::<dyn Filter>::new();

	macro_rules! declare {
		( $name:ident ) => {
			let $name = rules.declare();
		}
	}

	macro_rules! define {
		( $index:expr, $rule:expr ) => {
			rules.define($index, $rule);
		};
	}

	macro_rules! create {
		( $rule:expr ) => {
			rules.create($rule)
		}
	}

	macro_rules! token {
		( $name:ident, $element:expr ) => {
			let $name = create!(RuleToken::new($element));
		}
	}

	macro_rules! element {
		( $rule:expr, $element: expr ) => {
			RuleFilter::new($rule, filters.create(FilterElement::new($element)))
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

	declare!(expression);
	declare!(expression_option);
	declare!(expression_list);
	declare!(expression_element);
	declare!(expression_choice);
	declare!(function_element);
	declare!(function_sequence);
	declare!(parameters);
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
	declare!(declaration_element);
	declare!(declaration_sequence);
	declare!(control_element);
	declare!(control_choice);
	declare!(return_element);
	declare!(break_element);
	declare!(continue_element);
	declare!(literal_element);
	declare!(literal_choice);

	let statements = rules.declare();

	let extension = filters.declare();

	let chain = filters.create(FilterExtension::new(
		rules.create(RuleSequence::new(vec![symbol_dot, variable_identifier])),
		filters.create(FilterList::new(vec![
			extension,
			filters.create(FilterElement::new(&elements::productions::EXPRESSION)),
			filters.create(FilterElement::new(&elements::expressions::CHAIN)),
		]))
	));

	let sequence = filters.create(FilterExtension::new(
		rules.create(RuleChoice::new(vec![
			rules.create(RuleSequence::new(vec![symbol_parenthesis_l, expression_list, symbol_parenthesis_r])),
			rules.create(RuleSequence::new(vec![symbol_crotchet_l, expression_list, symbol_crotchet_r])),
		])),
		filters.create(FilterList::new(vec![
			extension,
			filters.create(FilterElement::new(&elements::productions::EXPRESSION)),
			filters.create(FilterElement::new(&elements::expressions::SEQUENCE)),
		]))
	));

	filters.define(extension, FilterList::new(vec![chain, sequence]));

	let block = rules.create(RuleFilter::new(
		rules.create(RuleSequence::new(vec![symbol_brace_l, statements, expression_option, symbol_brace_r])),
		filters.create(FilterElement::new(&elements::structures::BLOCK))
	));

	let r#if = rules.create(RuleFilter::new(
		rules.create(RuleSequence::new(vec![
			keyword_if,
			expression,
			rules.create(RuleFilter::new(
				rules.create(RuleChoice::new(vec![
					rules.create(RuleSequence::new(vec![keyword_then, expression])),
					block,
				])),
				filters.create(FilterElement::new(&elements::structures::IF_BODY))
			)),
			rules.create(RuleOption::new(
				rules.create(RuleFilter::new(
					rules.create(RuleSequence::new(vec![keyword_else, expression])),
					filters.create(FilterElement::new(&elements::structures::IF_ELSE))
				))
			)),
		])),
		filters.create(FilterElement::new(&elements::structures::IF))
	));

	let loop_body = rules.create(RuleFilter::new(
		rules.create(RuleChoice::new(vec![
			rules.create(RuleSequence::new(vec![keyword_do, expression])),
			block
		])),
		filters.create(FilterElement::new(&elements::structures::LOOP_BODY))
	));

	let r#loop = rules.create(RuleFilter::new(
		rules.create(RuleSequence::new(vec![keyword_loop, expression])),
		filters.create(FilterElement::new(&elements::structures::LOOP))
	));

	let r#while = rules.create(RuleFilter::new(
		rules.create(RuleSequence::new(vec![keyword_while, expression, loop_body])),
		filters.create(FilterElement::new(&elements::structures::WHILE))
	));

	let do_while = rules.create(RuleFilter::new(
		rules.create(RuleSequence::new(vec![keyword_do, expression, keyword_while, expression])),
		filters.create(FilterElement::new(&elements::structures::DO_WHILE))
	));

	let for_in = rules.create(RuleFilter::new(
		rules.create(RuleSequence::new(vec![keyword_for, variable_identifier, keyword_in, expression, loop_body])),
		filters.create(FilterElement::new(&elements::structures::FOR_IN))
	));

	let structure = rules.create(RuleFilter::new(
		rules.create(RuleChoice::new(vec![block, r#if, r#loop, r#while, do_while, for_in])),
		filters.create(FilterElement::new(&elements::structures::STRUCTURE))
	));

	let statement = rules.create(RuleFilter::new(
		rules.create(RuleSequence::new(vec![expression, symbol_semicolon])),
		filters.create(FilterElement::new(&elements::productions::STATEMENT))
	));

	rules.define(statements, RuleFilter::new(
		rules.create(RuleList::new(statement)),
		filters.create(FilterElement::new(&elements::productions::STATEMENTS))
	));

	let program = rules.create(RuleFilter::new(
		statements,
		filters.create(FilterElement::new(&elements::productions::PROGRAM))
	));

	define!(expression, RuleAlias::new(operation_12));
	define!(expression_option, RuleOption::new(expression));

	macro_rules! define_list {
		( $name:ident, $node:expr, $element:expr ) => {
			declare!(node_option);
			declare!(node_sequence);
			declare!(more_option);
			declare!(more_sequence);
			define!($name, element!(node_option, $element));
			define!(node_option, RuleOption::new(node_sequence));
			define!(node_sequence, RuleSequence::new(vec![$node, more_option]));
			define!(more_option, RuleOption::new(more_sequence));
			define!(more_sequence, RuleSequence::new(vec![symbol_comma, $node, more_option]));
		};
	}

	define_list!(expression_list, expression, &elements::productions::EXPRESSIONS);

	define!(expression_element, element!(expression_choice, &elements::productions::EXPRESSION));
	define!(expression_choice, RuleChoice::new(vec![function_element, structure, declaration_element, control_element, group_element, literal_element]));

	define!(function_element,  element!(function_sequence, &elements::expressions::FUNCTION));
	define!(function_sequence, RuleSequence::new(vec![keyword_function, symbol_parenthesis_l, parameters, symbol_parenthesis_r, block]));
	define_list!(parameters, variable_identifier, &elements::productions::PARAMETERS);

	macro_rules! define_control {
		( $name:ident, $keyword:expr, $element:expr ) => {
			declare!(control_sequence);
			define!($name, element!(control_sequence, $element));
			define!(control_sequence, RuleSequence::new(vec![$keyword, expression_option]));
		}
	}

	define!(declaration_element,  element!(declaration_sequence, &elements::expressions::DECLARATION));
	define!(declaration_sequence, RuleSequence::new(vec![keyword_let, variable_identifier]));

	define!(control_element, element!(control_choice, &elements::controls::CONTROL));
	define!(control_choice,  RuleChoice::new(vec![return_element, break_element, continue_element]));
	define_control!(return_element,   keyword_return,   &elements::controls::RETURN);
	define_control!(break_element,    keyword_break,    &elements::controls::BREAK);
	define_control!(continue_element, keyword_continue, &elements::controls::CONTINUE);

	define!(group_element,  element!(group_sequence, &elements::expressions::GROUP));
	define!(group_sequence, RuleSequence::new(vec![symbol_parenthesis_l, expression, symbol_parenthesis_r]));
	define!(literal_element, element!(literal_choice, &elements::expressions::LITERAL));
	define!(literal_choice, RuleChoice::new(vec![variable_identifier, variable_string, variable_number]));

	macro_rules! define_operation {
		( $name:ident, $child:expr, $tokens:expr ) => {
			declare!(operation_sequence);
			declare!(operation_choice);

			let filter = filters.declare();

			filters.define(filter, FilterExtension::new(
				operation_sequence,
				filters.create(FilterList::new(vec![
					filter,
					filters.create(FilterElement::new(&elements::productions::EXPRESSION)),
					filters.create(FilterElement::new(&elements::expressions::SEQUENCE))
				]))
			));

			define!($name, RuleFilter::new($child, filter));

			define!(operation_sequence, RuleSequence::new(vec![operation_choice, $child]));
			define!(operation_choice, RuleChoice::new($tokens));
		}
	}

	define_operation!(operation_1,   rules.create(RuleFilter::new(
		expression_element,
		extension,
	)), vec![symbol_asterisk, symbol_slash, symbol_percent, symbol_asterisk_d]);

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

	let program = &rules.get(program);
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
