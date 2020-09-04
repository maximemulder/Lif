use crate::elements;
use crate::node::Node;
use crate::parser2::arena::Arena;
use crate::parser2::filters::*;
use crate::parser2::rules::*;
use crate::parser2::Parser;

pub fn run<'a, 'b>(tokens: &Vec<Node<'a, 'b>>) -> Option<Node<'a, 'b>> {
	let rules = Arena::<dyn Rule>::new();
	let filters = Arena::<dyn Filter>::new();

	let keyword_as              = rules.create(RuleToken::new(&elements::keywords::AS));
	let keyword_catch           = rules.create(RuleToken::new(&elements::keywords::CATCH));
	let keyword_class           = rules.create(RuleToken::new(&elements::keywords::CLASS));
	let keyword_default         = rules.create(RuleToken::new(&elements::keywords::DEFAULT));
	let keyword_do              = rules.create(RuleToken::new(&elements::keywords::DO));
	let keyword_else            = rules.create(RuleToken::new(&elements::keywords::ELSE));
	let keyword_export          = rules.create(RuleToken::new(&elements::keywords::EXPORT));
	let keyword_finally         = rules.create(RuleToken::new(&elements::keywords::FINALLY));
	let keyword_for             = rules.create(RuleToken::new(&elements::keywords::FOR));
	let keyword_from            = rules.create(RuleToken::new(&elements::keywords::FROM));
	let keyword_function        = rules.create(RuleToken::new(&elements::keywords::FUNCTION));
	let keyword_if              = rules.create(RuleToken::new(&elements::keywords::IF));
	let keyword_import          = rules.create(RuleToken::new(&elements::keywords::IMPORT));
	let keyword_in              = rules.create(RuleToken::new(&elements::keywords::IN));
	let keyword_let             = rules.create(RuleToken::new(&elements::keywords::LET));
	let keyword_loop            = rules.create(RuleToken::new(&elements::keywords::LOOP));
	let keyword_private         = rules.create(RuleToken::new(&elements::keywords::PRIVATE));
	let keyword_protected       = rules.create(RuleToken::new(&elements::keywords::PROTECTED));
	let keyword_public          = rules.create(RuleToken::new(&elements::keywords::PUBLIC));
	let keyword_continue        = rules.create(RuleToken::new(&elements::keywords::CONTINUE));
	let keyword_break           = rules.create(RuleToken::new(&elements::keywords::BREAK));
	let keyword_return          = rules.create(RuleToken::new(&elements::keywords::RETURN));
	let keyword_static          = rules.create(RuleToken::new(&elements::keywords::STATIC));
	let keyword_then            = rules.create(RuleToken::new(&elements::keywords::THEN));
	let keyword_throw           = rules.create(RuleToken::new(&elements::keywords::THROW));
	let keyword_try             = rules.create(RuleToken::new(&elements::keywords::TRY));
	let keyword_while           = rules.create(RuleToken::new(&elements::keywords::WHILE));
	let symbol_plus             = rules.create(RuleToken::new(&elements::symbols::PLUS));
	let symbol_plus_eq          = rules.create(RuleToken::new(&elements::symbols::PLUS_EQ));
	let symbol_minus            = rules.create(RuleToken::new(&elements::symbols::MINUS));
	let symbol_minus_eq         = rules.create(RuleToken::new(&elements::symbols::MINUS_EQ));
	let symbol_asterisk         = rules.create(RuleToken::new(&elements::symbols::ASTERISK));
	let symbol_asterisk_eq      = rules.create(RuleToken::new(&elements::symbols::ASTERISK_EQ));
	let symbol_asterisk_d       = rules.create(RuleToken::new(&elements::symbols::ASTERISK_D));
	let symbol_asterisk_d_eq    = rules.create(RuleToken::new(&elements::symbols::ASTERISK_D_EQ));
	let symbol_slash            = rules.create(RuleToken::new(&elements::symbols::SLASH));
	let symbol_slash_eq         = rules.create(RuleToken::new(&elements::symbols::SLASH_EQ));
	let symbol_percent          = rules.create(RuleToken::new(&elements::symbols::PERCENT));
	let symbol_percent_eq       = rules.create(RuleToken::new(&elements::symbols::PERCENT_EQ));
	let symbol_caret            = rules.create(RuleToken::new(&elements::symbols::CARET));
	let symbol_caret_eq         = rules.create(RuleToken::new(&elements::symbols::CARET_EQ));
	let symbol_exclamation      = rules.create(RuleToken::new(&elements::symbols::EXCLAMATION));
	let symbol_exclamation_eq   = rules.create(RuleToken::new(&elements::symbols::EXCLAMATION_EQ));
	let symbol_equal            = rules.create(RuleToken::new(&elements::symbols::EQUAL));
	let symbol_equal_d          = rules.create(RuleToken::new(&elements::symbols::EQUAL_D));
	let symbol_arrow            = rules.create(RuleToken::new(&elements::symbols::ARROW));
	let symbol_pipe             = rules.create(RuleToken::new(&elements::symbols::PIPE));
	let symbol_pipe_eq          = rules.create(RuleToken::new(&elements::symbols::PIPE_EQ));
	let symbol_pipe_d           = rules.create(RuleToken::new(&elements::symbols::PIPE_D));
	let symbol_pipe_d_eq        = rules.create(RuleToken::new(&elements::symbols::PIPE_D_EQ));
	let symbol_ampersand        = rules.create(RuleToken::new(&elements::symbols::AMPERSAND));
	let symbol_ampersand_eq     = rules.create(RuleToken::new(&elements::symbols::AMPERSAND_EQ));
	let symbol_ampersand_d      = rules.create(RuleToken::new(&elements::symbols::AMPERSAND_D));
	let symbol_ampersand_d_eq   = rules.create(RuleToken::new(&elements::symbols::AMPERSAND_D_EQ));
	let symbol_guillemet_l      = rules.create(RuleToken::new(&elements::symbols::GUILLEMET_L));
	let symbol_guillemet_r      = rules.create(RuleToken::new(&elements::symbols::GUILLEMET_R));
	let symbol_guillemet_l_eq   = rules.create(RuleToken::new(&elements::symbols::GUILLEMET_L_EQ));
	let symbol_guillemet_r_eq   = rules.create(RuleToken::new(&elements::symbols::GUILLEMET_R_EQ));
	let symbol_guillemet_l_d    = rules.create(RuleToken::new(&elements::symbols::GUILLEMET_L_D));
	let symbol_guillemet_r_d    = rules.create(RuleToken::new(&elements::symbols::GUILLEMET_R_D));
	let symbol_guillemet_l_d_eq = rules.create(RuleToken::new(&elements::symbols::GUILLEMET_L_D_EQ));
	let symbol_guillemet_r_d_eq = rules.create(RuleToken::new(&elements::symbols::GUILLEMET_R_D_EQ));
	let symbol_guillemet_l_t    = rules.create(RuleToken::new(&elements::symbols::GUILLEMET_L_T));
	let symbol_guillemet_r_t    = rules.create(RuleToken::new(&elements::symbols::GUILLEMET_R_T));
	let symbol_guillemet_l_t_eq = rules.create(RuleToken::new(&elements::symbols::GUILLEMET_L_T_EQ));
	let symbol_guillemet_r_t_eq = rules.create(RuleToken::new(&elements::symbols::GUILLEMET_R_T_EQ));
	let symbol_parenthesis_l    = rules.create(RuleToken::new(&elements::symbols::PARENTHESIS_L));
	let symbol_parenthesis_r    = rules.create(RuleToken::new(&elements::symbols::PARENTHESIS_R));
	let symbol_brace_l          = rules.create(RuleToken::new(&elements::symbols::BRACE_L));
	let symbol_brace_r          = rules.create(RuleToken::new(&elements::symbols::BRACE_R));
	let symbol_crotchet_l       = rules.create(RuleToken::new(&elements::symbols::CROTCHET_L));
	let symbol_crotchet_r       = rules.create(RuleToken::new(&elements::symbols::CROTCHET_R));
	let symbol_dot              = rules.create(RuleToken::new(&elements::symbols::DOT));
	let symbol_dot_d            = rules.create(RuleToken::new(&elements::symbols::DOT_D));
	let symbol_dot_d_eq         = rules.create(RuleToken::new(&elements::symbols::DOT_D_EQ));
	let symbol_dot_t            = rules.create(RuleToken::new(&elements::symbols::DOT_T));
	let symbol_comma            = rules.create(RuleToken::new(&elements::symbols::COMMA));
	let symbol_tilde            = rules.create(RuleToken::new(&elements::symbols::TILDE));
	let symbol_dollar           = rules.create(RuleToken::new(&elements::symbols::DOLLAR));
	let symbol_interrogation    = rules.create(RuleToken::new(&elements::symbols::INTERROGATION));
	let symbol_colon            = rules.create(RuleToken::new(&elements::symbols::COLON));
	let symbol_semicolon        = rules.create(RuleToken::new(&elements::symbols::SEMICOLON));
	let symbol_backslash        = rules.create(RuleToken::new(&elements::symbols::BACKSLASH));
	let variable_identifier     = rules.create(RuleToken::new(&elements::variables::IDENTIFIER));
	let variable_string         = rules.create(RuleToken::new(&elements::variables::STRING));
	let variable_number         = rules.create(RuleToken::new(&elements::variables::NUMBER));

	let statements = rules.declare();

	let expression = rules.declare();

	let expression_option = rules.declare();

	let extension = filters.declare();

	macro_rules! create_control {
		( $keyword:expr, $element:expr ) => {
			rules.create(RuleFilter::new(
				rules.create(RuleSequence::new(vec![$keyword, expression_option])),
				filters.create(FilterElement::new($element))
			))
		}
	}

	macro_rules! create_operation {
		( $child:expr, $tokens:expr ) => {{
			let filter = filters.declare();

			filters.define(filter, FilterExtension::new(
				rules.create(RuleSequence::new(vec![
					rules.create(RuleChoice::new($tokens)),
					$child,
				])),
				filters.create(FilterList::new(vec![
					filter,
					filters.create(FilterElement::new(&elements::productions::EXPRESSION)),
					filters.create(FilterElement::new(&elements::expressions::OPERATION)),
				]))
			));

			rules.create(RuleFilter::new($child, filter))
		}}
	}

	let literal = rules.create(RuleFilter::new(
		rules.create(RuleChoice::new(vec![variable_identifier, variable_string, variable_number])),
		filters.create(FilterElement::new(&elements::expressions::LITERAL))
	));

	let group = rules.create(RuleFilter::new(
		rules.create(RuleSequence::new(vec![symbol_parenthesis_l, expression, symbol_parenthesis_r])),
		filters.create(FilterElement::new(&elements::expressions::GROUP))
	));

	let declaration = rules.create(RuleFilter::new(
		rules.create(RuleSequence::new(vec![keyword_let, variable_identifier])),
		filters.create(FilterElement::new(&elements::expressions::DECLARATION))
	));

	let control = rules.create(RuleFilter::new(
		rules.create(RuleChoice::new(vec![
			create_control!(keyword_return,   &elements::controls::RETURN),
			create_control!(keyword_break,    &elements::controls::BREAK),
			create_control!(keyword_continue, &elements::controls::CONTINUE),
		])),
		filters.create(FilterElement::new(&elements::controls::CONTROL))
	));

	let block = rules.create(RuleFilter::new(
		rules.create(RuleSequence::new(vec![symbol_brace_l, statements, expression_option, symbol_brace_r])),
		filters.create(FilterElement::new(&elements::structures::BLOCK))
	));

	let function = rules.create(RuleFilter::new(
		rules.create(RuleSequence::new(vec![
			keyword_function,
			symbol_parenthesis_l,
			rules.create(RuleFilter::new(
				rules.create(RuleList::new(variable_identifier, symbol_comma)),
				filters.create(FilterElement::new(&elements::productions::PARAMETERS))
			)),
			symbol_parenthesis_r,
			block,
		])),
		filters.create(FilterElement::new(&elements::expressions::FUNCTION))
	));

	let r#if = rules.create(RuleFilter::new(
		rules.create(RuleSequence::new(vec![
			keyword_if,
			expression,
			block,
			rules.create(RuleOption::new(
				rules.create(RuleSequence::new(vec![keyword_else, block])),
			)),
		])),
		filters.create(FilterElement::new(&elements::structures::IF))
	));

	let r#loop = rules.create(RuleFilter::new(
		rules.create(RuleSequence::new(vec![keyword_loop, block])),
		filters.create(FilterElement::new(&elements::structures::LOOP))
	));

	let r#while = rules.create(RuleFilter::new(
		rules.create(RuleSequence::new(vec![keyword_while, expression, block])),
		filters.create(FilterElement::new(&elements::structures::WHILE))
	));

	let for_in = rules.create(RuleFilter::new(
		rules.create(RuleSequence::new(vec![keyword_for, variable_identifier, keyword_in, expression, block])),
		filters.create(FilterElement::new(&elements::structures::FOR_IN))
	));

	let structure = rules.create(RuleFilter::new(
		rules.create(RuleChoice::new(vec![block, r#if, r#loop, r#while, for_in])),
		filters.create(FilterElement::new(&elements::structures::STRUCTURE))
	));

	let expressions = rules.create(RuleFilter::new(
		rules.create(RuleList::new(expression, symbol_comma)),
		filters.create(FilterElement::new(&elements::productions::EXPRESSIONS))
	));

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
			rules.create(RuleSequence::new(vec![symbol_parenthesis_l, expressions, symbol_parenthesis_r])),
			rules.create(RuleSequence::new(vec![symbol_crotchet_l, expressions, symbol_crotchet_r])),
		])),
		filters.create(FilterList::new(vec![
			extension,
			filters.create(FilterElement::new(&elements::productions::EXPRESSION)),
			filters.create(FilterElement::new(&elements::expressions::SEQUENCE)),
		]))
	));

	filters.define(extension, FilterList::new(vec![chain, sequence]));

	let expression_base = rules.create(RuleFilter::new(
		rules.create(RuleChoice::new(vec![function, structure, declaration, control, group, literal])),
		filters.create(FilterElement::new(&elements::productions::EXPRESSION))
	));

	let operation_1 = create_operation!(rules.create(RuleFilter::new(
		expression_base,
		extension,
	)), vec![symbol_asterisk, symbol_slash, symbol_percent, symbol_asterisk_d]);

	let operation_2  = create_operation!(operation_1,  vec![symbol_plus, symbol_minus]);

	let operation_3  = create_operation!(operation_2,  vec![symbol_guillemet_l_d, symbol_guillemet_r_d, symbol_guillemet_l_t, symbol_guillemet_l_t]);

	let operation_4  = create_operation!(operation_3,  vec![symbol_ampersand]);

	let operation_5  = create_operation!(operation_4,  vec![symbol_caret]);

	let operation_6  = create_operation!(operation_5,  vec![symbol_pipe]);

	let operation_7  = create_operation!(operation_6,  vec![symbol_guillemet_l, symbol_guillemet_r, symbol_guillemet_l_eq, symbol_guillemet_l_eq]);

	let operation_8  = create_operation!(operation_7,  vec![symbol_equal_d, symbol_exclamation_eq]);

	let operation_9  = create_operation!(operation_8,  vec![symbol_ampersand_d]);

	let operation_10 = create_operation!(operation_9,  vec![symbol_pipe_d]);

	let operation_11 = create_operation!(operation_10, vec![symbol_dot_d, symbol_dot_d_eq]);

	let operation_12 = create_operation!(operation_11, vec![symbol_equal, symbol_plus_eq, symbol_minus_eq, symbol_asterisk_eq, symbol_slash_eq,
		symbol_percent_eq, symbol_asterisk_d_eq, symbol_guillemet_l_d_eq, symbol_guillemet_r_d_eq, symbol_guillemet_l_t_eq, symbol_guillemet_r_t_eq,
		symbol_ampersand_eq, symbol_caret_eq, symbol_pipe_eq, symbol_ampersand_d_eq, symbol_pipe_d_eq
	]);

	rules.define(expression, RuleAlias::new(operation_12));

	rules.define(expression_option, RuleOption::new(expression));

	let statement = rules.create(RuleFilter::new(
		rules.create(RuleChoice::new(vec![
			rules.create(RuleSequence::new(vec![structure, rules.create(RuleOption::new(symbol_semicolon))])),
			rules.create(RuleSequence::new(vec![expression, symbol_semicolon])),
		])),
		filters.create(FilterElement::new(&elements::productions::STATEMENT))
	));

	rules.define(statements, RuleFilter::new(
		rules.create(RuleList::new(statement, rules.create(RuleNone::new()))),
		filters.create(FilterElement::new(&elements::productions::STATEMENTS))
	));

	let program = rules.create(RuleFilter::new(
		statements,
		filters.create(FilterElement::new(&elements::productions::PROGRAM))
	));

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
