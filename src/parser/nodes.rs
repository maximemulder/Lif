use crate::code::Code;
use crate::elements;
use crate::node::Node;
use crate::parser::arena2::Arena;
use crate::parser::ascent::*;
use crate::parser::descent::*;
use crate::parser::Parser;

pub fn run<'a>(code: &Code, tokens: &[Node<'a>]) -> Option<Node<'a>> {
    let descents = Arena::new();
    let ascents = Arena::new();

    let keyword_as              = descents.define(DescentToken::new(&elements::keywords::AS));
    let keyword_catch           = descents.define(DescentToken::new(&elements::keywords::CATCH));
    let keyword_class           = descents.define(DescentToken::new(&elements::keywords::CLASS));
    let keyword_default         = descents.define(DescentToken::new(&elements::keywords::DEFAULT));
    let keyword_do              = descents.define(DescentToken::new(&elements::keywords::DO));
    let keyword_else            = descents.define(DescentToken::new(&elements::keywords::ELSE));
    let keyword_export          = descents.define(DescentToken::new(&elements::keywords::EXPORT));
    let keyword_false           = descents.define(DescentToken::new(&elements::keywords::FALSE));
    let keyword_finally         = descents.define(DescentToken::new(&elements::keywords::FINALLY));
    let keyword_for             = descents.define(DescentToken::new(&elements::keywords::FOR));
    let keyword_from            = descents.define(DescentToken::new(&elements::keywords::FROM));
    let keyword_function        = descents.define(DescentToken::new(&elements::keywords::FUNCTION));
    let keyword_if              = descents.define(DescentToken::new(&elements::keywords::IF));
    let keyword_import          = descents.define(DescentToken::new(&elements::keywords::IMPORT));
    let keyword_in              = descents.define(DescentToken::new(&elements::keywords::IN));
    let keyword_let             = descents.define(DescentToken::new(&elements::keywords::LET));
    let keyword_loop            = descents.define(DescentToken::new(&elements::keywords::LOOP));
    let keyword_private         = descents.define(DescentToken::new(&elements::keywords::PRIVATE));
    let keyword_protected       = descents.define(DescentToken::new(&elements::keywords::PROTECTED));
    let keyword_public          = descents.define(DescentToken::new(&elements::keywords::PUBLIC));
    let keyword_continue        = descents.define(DescentToken::new(&elements::keywords::CONTINUE));
    let keyword_break           = descents.define(DescentToken::new(&elements::keywords::BREAK));
    let keyword_return          = descents.define(DescentToken::new(&elements::keywords::RETURN));
    let keyword_static          = descents.define(DescentToken::new(&elements::keywords::STATIC));
    let keyword_then            = descents.define(DescentToken::new(&elements::keywords::THEN));
    let keyword_throw           = descents.define(DescentToken::new(&elements::keywords::THROW));
    let keyword_true            = descents.define(DescentToken::new(&elements::keywords::TRUE));
    let keyword_try             = descents.define(DescentToken::new(&elements::keywords::TRY));
    let keyword_while           = descents.define(DescentToken::new(&elements::keywords::WHILE));
    let symbol_plus             = descents.define(DescentToken::new(&elements::symbols::PLUS));
    let symbol_plus_eq          = descents.define(DescentToken::new(&elements::symbols::PLUS_EQ));
    let symbol_minus            = descents.define(DescentToken::new(&elements::symbols::MINUS));
    let symbol_minus_eq         = descents.define(DescentToken::new(&elements::symbols::MINUS_EQ));
    let symbol_asterisk         = descents.define(DescentToken::new(&elements::symbols::ASTERISK));
    let symbol_asterisk_eq      = descents.define(DescentToken::new(&elements::symbols::ASTERISK_EQ));
    let symbol_asterisk_d       = descents.define(DescentToken::new(&elements::symbols::ASTERISK_D));
    let symbol_asterisk_d_eq    = descents.define(DescentToken::new(&elements::symbols::ASTERISK_D_EQ));
    let symbol_slash            = descents.define(DescentToken::new(&elements::symbols::SLASH));
    let symbol_slash_eq         = descents.define(DescentToken::new(&elements::symbols::SLASH_EQ));
    let symbol_percent          = descents.define(DescentToken::new(&elements::symbols::PERCENT));
    let symbol_percent_eq       = descents.define(DescentToken::new(&elements::symbols::PERCENT_EQ));
    let symbol_caret            = descents.define(DescentToken::new(&elements::symbols::CARET));
    let symbol_caret_eq         = descents.define(DescentToken::new(&elements::symbols::CARET_EQ));
    let symbol_exclamation      = descents.define(DescentToken::new(&elements::symbols::EXCLAMATION));
    let symbol_exclamation_eq   = descents.define(DescentToken::new(&elements::symbols::EXCLAMATION_EQ));
    let symbol_equal            = descents.define(DescentToken::new(&elements::symbols::EQUAL));
    let symbol_equal_d          = descents.define(DescentToken::new(&elements::symbols::EQUAL_D));
    let symbol_arrow            = descents.define(DescentToken::new(&elements::symbols::ARROW));
    let symbol_pipe             = descents.define(DescentToken::new(&elements::symbols::PIPE));
    let symbol_pipe_eq          = descents.define(DescentToken::new(&elements::symbols::PIPE_EQ));
    let symbol_pipe_d           = descents.define(DescentToken::new(&elements::symbols::PIPE_D));
    let symbol_pipe_d_eq        = descents.define(DescentToken::new(&elements::symbols::PIPE_D_EQ));
    let symbol_ampersand        = descents.define(DescentToken::new(&elements::symbols::AMPERSAND));
    let symbol_ampersand_eq     = descents.define(DescentToken::new(&elements::symbols::AMPERSAND_EQ));
    let symbol_ampersand_d      = descents.define(DescentToken::new(&elements::symbols::AMPERSAND_D));
    let symbol_ampersand_d_eq   = descents.define(DescentToken::new(&elements::symbols::AMPERSAND_D_EQ));
    let symbol_guillemet_l      = descents.define(DescentToken::new(&elements::symbols::GUILLEMET_L));
    let symbol_guillemet_r      = descents.define(DescentToken::new(&elements::symbols::GUILLEMET_R));
    let symbol_guillemet_l_eq   = descents.define(DescentToken::new(&elements::symbols::GUILLEMET_L_EQ));
    let symbol_guillemet_r_eq   = descents.define(DescentToken::new(&elements::symbols::GUILLEMET_R_EQ));
    let symbol_guillemet_l_d    = descents.define(DescentToken::new(&elements::symbols::GUILLEMET_L_D));
    let symbol_guillemet_r_d    = descents.define(DescentToken::new(&elements::symbols::GUILLEMET_R_D));
    let symbol_guillemet_l_d_eq = descents.define(DescentToken::new(&elements::symbols::GUILLEMET_L_D_EQ));
    let symbol_guillemet_r_d_eq = descents.define(DescentToken::new(&elements::symbols::GUILLEMET_R_D_EQ));
    let symbol_guillemet_l_t    = descents.define(DescentToken::new(&elements::symbols::GUILLEMET_L_T));
    let symbol_guillemet_r_t    = descents.define(DescentToken::new(&elements::symbols::GUILLEMET_R_T));
    let symbol_guillemet_l_t_eq = descents.define(DescentToken::new(&elements::symbols::GUILLEMET_L_T_EQ));
    let symbol_guillemet_r_t_eq = descents.define(DescentToken::new(&elements::symbols::GUILLEMET_R_T_EQ));
    let symbol_parenthesis_l    = descents.define(DescentToken::new(&elements::symbols::PARENTHESIS_L));
    let symbol_parenthesis_r    = descents.define(DescentToken::new(&elements::symbols::PARENTHESIS_R));
    let symbol_brace_l          = descents.define(DescentToken::new(&elements::symbols::BRACE_L));
    let symbol_brace_r          = descents.define(DescentToken::new(&elements::symbols::BRACE_R));
    let symbol_crotchet_l       = descents.define(DescentToken::new(&elements::symbols::CROTCHET_L));
    let symbol_crotchet_r       = descents.define(DescentToken::new(&elements::symbols::CROTCHET_R));
    let symbol_dot              = descents.define(DescentToken::new(&elements::symbols::DOT));
    let symbol_dot_d            = descents.define(DescentToken::new(&elements::symbols::DOT_D));
    let symbol_dot_d_eq         = descents.define(DescentToken::new(&elements::symbols::DOT_D_EQ));
    let symbol_dot_t            = descents.define(DescentToken::new(&elements::symbols::DOT_T));
    let symbol_comma            = descents.define(DescentToken::new(&elements::symbols::COMMA));
    let symbol_tilde            = descents.define(DescentToken::new(&elements::symbols::TILDE));
    let symbol_dollar           = descents.define(DescentToken::new(&elements::symbols::DOLLAR));
    let symbol_interrogation    = descents.define(DescentToken::new(&elements::symbols::INTERROGATION));
    let symbol_colon            = descents.define(DescentToken::new(&elements::symbols::COLON));
    let symbol_semicolon        = descents.define(DescentToken::new(&elements::symbols::SEMICOLON));
    let symbol_backslash        = descents.define(DescentToken::new(&elements::symbols::BACKSLASH));
    let variable_identifier     = descents.define(DescentToken::new(&elements::variables::IDENTIFIER));
    let variable_string         = descents.define(DescentToken::new(&elements::variables::STRING));
    let variable_number         = descents.define(DescentToken::new(&elements::variables::NUMBER));

    let statements = descents.declare();

    let expression = descents.declare();

    let expression_base = descents.declare();

    let operation_base = descents.declare();

    let expression_option = descents.declare();

    let extension = ascents.declare();

    let name = descents.define(DescentElement::new(
        descents.define(DescentOption::new(variable_identifier)),
        &elements::productions::NAME
    ));

    macro_rules! create_control {
        ( $keyword:expr, $element:expr ) => {
            descents.define(DescentElement::new(
                descents.define(DescentSequence::new([$keyword, expression_option])),
                $element
            ))
        }
    }

    macro_rules! create_operation_ascent {
        ( $child:expr, $tokens:expr ) => {{
            let ascent = ascents.declare();
            ascents.define2(ascent, AscentExtension::new(
                descents.define(DescentSequence::new([
                    descents.define(DescentChoice::new($tokens)),
                    $child,
                ])),
                ascents.define(AscentList::new([
                    ascent,
                    ascents.define(AscentElement::new(&elements::expressions::EXPRESSION)),
                    ascents.define(AscentElement::new(&elements::expressions::OPERATION)),
                ]))
            ));

            ascent
        }}
    }

    macro_rules! create_operation {
        ( $child:expr, $tokens:expr ) => {{
            descents.define(DescentAscent::new($child, create_operation_ascent!($child, $tokens)))
        }}
    }

    macro_rules! create_assignment {
        ( $child:expr, $tokens:expr ) => {
            descents.define(DescentAscent::new($child, {
                let ascent = ascents.declare();
                ascents.define2(ascent, AscentExtension::new(
                    descents.define(DescentSequence::new([
                        descents.define(DescentChoice::new($tokens)),
                        $child,
                    ])),
                    ascents.define(AscentList::new([
                        ascent,
                        ascents.define(AscentElement::new(&elements::expressions::EXPRESSION)),
                        ascents.define(AscentElement::new(&elements::expressions::ASSIGNMENT)),
                    ]))
                ));

                ascent
            }))
        }
    }

    macro_rules! create_list {
        ( $element:expr, $separator:expr ) => {{
            descents.define(DescentSequence::new([
                $element,
                descents.define(DescentZeroOrMore::new(
                    descents.define(DescentSequence::new([$separator, $element]))
                ))
            ]))
        }}
    }

    macro_rules! create_list_option {
        ( $element:expr, $separator:expr ) => {{
            descents.define(DescentOption::new(
                create_list!($element, $separator)
            ))
        }}
    }

    let expressions = descents.define(DescentElement::new(
        create_list_option!(expression, symbol_comma),
        &elements::productions::EXPRESSIONS
    ));

    let r#type = descents.define(DescentElement::new(
        descents.define(DescentOption::new(
            descents.define(DescentSequence::new([symbol_colon, operation_base])),
        )),
        &elements::productions::TYPE
    ));

    let literal = descents.define(DescentElement::new(
        descents.define(DescentChoice::new([variable_identifier, variable_string, variable_number, keyword_true, keyword_false])),
        &elements::expressions::LITERAL
    ));

    let array = descents.define(DescentElement::new(
        descents.define(DescentSequence::new([symbol_crotchet_l, expressions, symbol_crotchet_r])),
        &elements::expressions::ARRAY
    ));

    let group = descents.define(DescentElement::new(
        descents.define(DescentSequence::new([symbol_parenthesis_l, expression, symbol_parenthesis_r])),
        &elements::expressions::GROUP
    ));

    let declaration = descents.define(DescentElement::new(
        descents.define(DescentSequence::new([variable_identifier, r#type])),
        &elements::productions::DECLARATION
    ));

    let r#let = descents.define(DescentElement::new(
        descents.define(DescentSequence::new([keyword_let, declaration])),
        &elements::expressions::LET
    ));

    let control = descents.define(DescentElement::new(
        descents.define(DescentChoice::new([
            create_control!(keyword_return,   &elements::controls::RETURN),
            create_control!(keyword_break,    &elements::controls::BREAK),
            create_control!(keyword_continue, &elements::controls::CONTINUE),
        ])),
        &elements::controls::CONTROL
    ));

    let block = descents.define(DescentElement::new(
        descents.define(DescentSequence::new([symbol_brace_l, statements, expression_option, symbol_brace_r])),
        &elements::flows::BLOCK
    ));

    let r#if = descents.define(DescentElement::new(
        descents.define(DescentSequence::new([
            keyword_if,
            expression,
            block,
            descents.define(DescentOption::new(
                descents.define(DescentSequence::new([keyword_else, block])),
            )),
        ])),
        &elements::flows::IF
    ));

    let r#loop = descents.define(DescentElement::new(
        descents.define(DescentSequence::new([keyword_loop, block])),
        &elements::flows::LOOP
    ));

    let r#while = descents.define(DescentElement::new(
        descents.define(DescentSequence::new([keyword_while, expression, block])),
        &elements::flows::WHILE
    ));

    let for_in = descents.define(DescentElement::new(
        descents.define(DescentSequence::new([keyword_for, variable_identifier, keyword_in, expression, block])),
        &elements::flows::FOR_IN
    ));

    let flow = descents.define(DescentElement::new(
        descents.define(DescentChoice::new([block, r#if, r#loop, r#while, for_in])),
        &elements::flows::FLOW
    ));

    let generics = descents.define(DescentOption::new(
        descents.define(DescentSequence::new([
            symbol_guillemet_l,
            descents.define(DescentAscent::new(
                descents.define(DescentOneOrMore::new(
                    create_list!(variable_identifier, symbol_comma)
                )),
                ascents.define(AscentElement::new(&elements::productions::GENERICS))
            )),
            symbol_guillemet_r,
        ]))
    ));

    let parameters = descents.define(DescentElement::new(
        create_list_option!(declaration, symbol_comma),
        &elements::productions::PARAMETERS
    ));

    let function = descents.define(DescentElement::new(
        descents.define(DescentSequence::new([keyword_function, name, generics, symbol_parenthesis_l, parameters, symbol_parenthesis_r, r#type, block])),
        &elements::structures::FUNCTION
    ));

    let function_named = descents.define(DescentElement::new(
        descents.define(DescentSequence::new([keyword_function, variable_identifier, generics, symbol_parenthesis_l, parameters, symbol_parenthesis_r, r#type, block])),
        &elements::structures::FUNCTION
    ));

    let methods = descents.define(DescentElement::new(
        descents.define(DescentZeroOrMore::new(function_named)),
        &elements::productions::METHODS
    ));

    let class = descents.define(DescentElement::new(
        descents.define(DescentSequence::new([keyword_class, name, generics, r#type, symbol_brace_l, methods, symbol_brace_r])),
        &elements::structures::CLASS
    ));

    let class_named = descents.define(DescentElement::new(
        descents.define(DescentSequence::new([keyword_class, variable_identifier, generics, r#type, symbol_brace_l, methods, symbol_brace_r])),
        &elements::structures::CLASS
    ));

    let structure = descents.define(DescentElement::new(
        descents.define(DescentChoice::new([class_named, function_named])),
        &elements::structures::STRUCTURE
    ));

    let chain = ascents.define(AscentExtension::new(
        descents.define(DescentSequence::new([
            symbol_dot,
            variable_identifier,
        ])),
        ascents.define(AscentList::new([
            extension,
            ascents.define(AscentElement::new(&elements::expressions::EXPRESSION)),
            ascents.define(AscentElement::new(&elements::expressions::CHAIN)),
        ]))
    ));

    let sequence = ascents.define(AscentExtension::new(
        descents.define(DescentChoice::new([
            descents.define(DescentSequence::new([symbol_parenthesis_l, expressions, symbol_parenthesis_r])),
            descents.define(DescentSequence::new([symbol_crotchet_l, expressions, symbol_crotchet_r])),
            descents.define(DescentSequence::new([
                symbol_guillemet_l,
                descents.define(DescentElement::new(
                    create_list!(expression_base, symbol_comma),
                    &elements::productions::EXPRESSIONS
                )),
                symbol_guillemet_r,
            ])),
        ])),
        ascents.define(AscentList::new([
            extension,
            ascents.define(AscentElement::new(&elements::expressions::EXPRESSION)),
            ascents.define(AscentElement::new(&elements::expressions::SEQUENCE)),
        ]))
    ));

    ascents.define2(extension, AscentList::new([chain, sequence]));

    descents.define2(expression_base, DescentElement::new(
        descents.define(DescentChoice::new([class, function, flow, control, r#let, array, group, literal])),
        &elements::expressions::EXPRESSION
    ));

    let operation_1 = create_operation!(descents.define(DescentAscent::new(
        expression_base,
        extension,
    )), [symbol_asterisk, symbol_slash, symbol_percent, symbol_asterisk_d]);

    descents.define2(operation_base, DescentAlias::new(operation_1));

    let operation_2  = create_operation!(operation_1, [symbol_plus, symbol_minus]);

    let operation_3  = create_operation!(operation_2, [symbol_guillemet_l_d, symbol_guillemet_r_d, symbol_guillemet_l_t, symbol_guillemet_l_t]);

    let operation_4  = create_operation!(operation_3, [symbol_ampersand]);

    let operation_5  = create_operation!(operation_4, [symbol_caret]);

    let operation_6  = create_operation!(operation_5, [symbol_pipe]);

    let operation_7_l = create_operation_ascent!(operation_6, [symbol_guillemet_l, symbol_guillemet_l_eq]);

    let operation_7_r = create_operation_ascent!(operation_6, [symbol_guillemet_r, symbol_guillemet_r_eq]);

    let operation_7 = descents.define(DescentChoice::new([
        descents.define(DescentSequence::new([
            descents.define(DescentAscent::new(operation_6, operation_7_l)),
            descents.define(DescentPredicateNot::new(
                descents.define(DescentChoice::new([symbol_guillemet_r, symbol_guillemet_r_eq]))
            )),
        ])),
        descents.define(DescentSequence::new([
            descents.define(DescentAscent::new(operation_6, operation_7_r)),
            descents.define(DescentPredicateNot::new(
                descents.define(DescentChoice::new([symbol_guillemet_l, symbol_guillemet_l_eq]))
            )),
        ])),
    ]));

    let operation_8  = create_operation!(operation_7,  [symbol_equal_d, symbol_exclamation_eq]);

    let operation_9  = create_operation!(operation_8,  [symbol_ampersand_d]);

    let operation_10 = create_operation!(operation_9,  [symbol_pipe_d]);

    let operation_11 = create_operation!(operation_10, [symbol_dot_d, symbol_dot_d_eq]);

    let operation_12 = create_assignment!(operation_11, [symbol_equal, symbol_plus_eq, symbol_minus_eq, symbol_asterisk_eq, symbol_slash_eq,
        symbol_percent_eq, symbol_asterisk_d_eq, symbol_guillemet_l_d_eq, symbol_guillemet_r_d_eq, symbol_guillemet_l_t_eq, symbol_guillemet_r_t_eq,
        symbol_ampersand_eq, symbol_caret_eq, symbol_pipe_eq, symbol_ampersand_d_eq, symbol_pipe_d_eq
    ]);

    descents.define2(expression, DescentAlias::new(operation_12));

    descents.define2(expression_option, DescentOption::new(expression));

    let no_semicolon = descents.define(DescentPredicateNot::new(symbol_semicolon));

    let statement = descents.define(DescentElement::new(
        descents.define(DescentChoice::new([
            descents.define(DescentSequence::new([structure, no_semicolon])),
            descents.define(DescentSequence::new([flow, no_semicolon])),
            descents.define(DescentSequence::new([expression, symbol_semicolon])),
        ])),
        &elements::productions::STATEMENT
    ));

    descents.define2(statements, DescentElement::new(
        descents.define(DescentZeroOrMore::new(statement)),
        &elements::productions::STATEMENTS
    ));

    let program = descents.define(DescentElement::new(
        statements,
        &elements::productions::PROGRAM
    ));

    let mut parser = Parser::new(code, tokens, &descents, &ascents);
    parser.parse(program)
}
