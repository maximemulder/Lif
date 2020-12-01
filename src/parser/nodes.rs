use crate::elements;
use crate::parser::arena::Arena;
use crate::parser::ascent::*;
use crate::parser::descent::*;

pub fn get() -> (Arena::<dyn Descent>, Arena::<dyn Ascent>, usize) {
    let descents = Arena::<dyn Descent>::new();
    let ascents = Arena::<dyn Ascent>::new();

    let keyword_as              = descents.create(DescentToken::new(&elements::keywords::AS));
    let keyword_catch           = descents.create(DescentToken::new(&elements::keywords::CATCH));
    let keyword_class           = descents.create(DescentToken::new(&elements::keywords::CLASS));
    let keyword_default         = descents.create(DescentToken::new(&elements::keywords::DEFAULT));
    let keyword_do              = descents.create(DescentToken::new(&elements::keywords::DO));
    let keyword_else            = descents.create(DescentToken::new(&elements::keywords::ELSE));
    let keyword_export          = descents.create(DescentToken::new(&elements::keywords::EXPORT));
    let keyword_false           = descents.create(DescentToken::new(&elements::keywords::FALSE));
    let keyword_finally         = descents.create(DescentToken::new(&elements::keywords::FINALLY));
    let keyword_for             = descents.create(DescentToken::new(&elements::keywords::FOR));
    let keyword_from            = descents.create(DescentToken::new(&elements::keywords::FROM));
    let keyword_function        = descents.create(DescentToken::new(&elements::keywords::FUNCTION));
    let keyword_if              = descents.create(DescentToken::new(&elements::keywords::IF));
    let keyword_import          = descents.create(DescentToken::new(&elements::keywords::IMPORT));
    let keyword_in              = descents.create(DescentToken::new(&elements::keywords::IN));
    let keyword_let             = descents.create(DescentToken::new(&elements::keywords::LET));
    let keyword_loop            = descents.create(DescentToken::new(&elements::keywords::LOOP));
    let keyword_private         = descents.create(DescentToken::new(&elements::keywords::PRIVATE));
    let keyword_protected       = descents.create(DescentToken::new(&elements::keywords::PROTECTED));
    let keyword_public          = descents.create(DescentToken::new(&elements::keywords::PUBLIC));
    let keyword_continue        = descents.create(DescentToken::new(&elements::keywords::CONTINUE));
    let keyword_break           = descents.create(DescentToken::new(&elements::keywords::BREAK));
    let keyword_return          = descents.create(DescentToken::new(&elements::keywords::RETURN));
    let keyword_static          = descents.create(DescentToken::new(&elements::keywords::STATIC));
    let keyword_then            = descents.create(DescentToken::new(&elements::keywords::THEN));
    let keyword_throw           = descents.create(DescentToken::new(&elements::keywords::THROW));
    let keyword_true            = descents.create(DescentToken::new(&elements::keywords::TRUE));
    let keyword_try             = descents.create(DescentToken::new(&elements::keywords::TRY));
    let keyword_while           = descents.create(DescentToken::new(&elements::keywords::WHILE));
    let symbol_plus             = descents.create(DescentToken::new(&elements::symbols::PLUS));
    let symbol_plus_eq          = descents.create(DescentToken::new(&elements::symbols::PLUS_EQ));
    let symbol_minus            = descents.create(DescentToken::new(&elements::symbols::MINUS));
    let symbol_minus_eq         = descents.create(DescentToken::new(&elements::symbols::MINUS_EQ));
    let symbol_asterisk         = descents.create(DescentToken::new(&elements::symbols::ASTERISK));
    let symbol_asterisk_eq      = descents.create(DescentToken::new(&elements::symbols::ASTERISK_EQ));
    let symbol_asterisk_d       = descents.create(DescentToken::new(&elements::symbols::ASTERISK_D));
    let symbol_asterisk_d_eq    = descents.create(DescentToken::new(&elements::symbols::ASTERISK_D_EQ));
    let symbol_slash            = descents.create(DescentToken::new(&elements::symbols::SLASH));
    let symbol_slash_eq         = descents.create(DescentToken::new(&elements::symbols::SLASH_EQ));
    let symbol_percent          = descents.create(DescentToken::new(&elements::symbols::PERCENT));
    let symbol_percent_eq       = descents.create(DescentToken::new(&elements::symbols::PERCENT_EQ));
    let symbol_caret            = descents.create(DescentToken::new(&elements::symbols::CARET));
    let symbol_caret_eq         = descents.create(DescentToken::new(&elements::symbols::CARET_EQ));
    let symbol_exclamation      = descents.create(DescentToken::new(&elements::symbols::EXCLAMATION));
    let symbol_exclamation_eq   = descents.create(DescentToken::new(&elements::symbols::EXCLAMATION_EQ));
    let symbol_equal            = descents.create(DescentToken::new(&elements::symbols::EQUAL));
    let symbol_equal_d          = descents.create(DescentToken::new(&elements::symbols::EQUAL_D));
    let symbol_arrow            = descents.create(DescentToken::new(&elements::symbols::ARROW));
    let symbol_pipe             = descents.create(DescentToken::new(&elements::symbols::PIPE));
    let symbol_pipe_eq          = descents.create(DescentToken::new(&elements::symbols::PIPE_EQ));
    let symbol_pipe_d           = descents.create(DescentToken::new(&elements::symbols::PIPE_D));
    let symbol_pipe_d_eq        = descents.create(DescentToken::new(&elements::symbols::PIPE_D_EQ));
    let symbol_ampersand        = descents.create(DescentToken::new(&elements::symbols::AMPERSAND));
    let symbol_ampersand_eq     = descents.create(DescentToken::new(&elements::symbols::AMPERSAND_EQ));
    let symbol_ampersand_d      = descents.create(DescentToken::new(&elements::symbols::AMPERSAND_D));
    let symbol_ampersand_d_eq   = descents.create(DescentToken::new(&elements::symbols::AMPERSAND_D_EQ));
    let symbol_guillemet_l      = descents.create(DescentToken::new(&elements::symbols::GUILLEMET_L));
    let symbol_guillemet_r      = descents.create(DescentToken::new(&elements::symbols::GUILLEMET_R));
    let symbol_guillemet_l_eq   = descents.create(DescentToken::new(&elements::symbols::GUILLEMET_L_EQ));
    let symbol_guillemet_r_eq   = descents.create(DescentToken::new(&elements::symbols::GUILLEMET_R_EQ));
    let symbol_guillemet_l_d    = descents.create(DescentToken::new(&elements::symbols::GUILLEMET_L_D));
    let symbol_guillemet_r_d    = descents.create(DescentToken::new(&elements::symbols::GUILLEMET_R_D));
    let symbol_guillemet_l_d_eq = descents.create(DescentToken::new(&elements::symbols::GUILLEMET_L_D_EQ));
    let symbol_guillemet_r_d_eq = descents.create(DescentToken::new(&elements::symbols::GUILLEMET_R_D_EQ));
    let symbol_guillemet_l_t    = descents.create(DescentToken::new(&elements::symbols::GUILLEMET_L_T));
    let symbol_guillemet_r_t    = descents.create(DescentToken::new(&elements::symbols::GUILLEMET_R_T));
    let symbol_guillemet_l_t_eq = descents.create(DescentToken::new(&elements::symbols::GUILLEMET_L_T_EQ));
    let symbol_guillemet_r_t_eq = descents.create(DescentToken::new(&elements::symbols::GUILLEMET_R_T_EQ));
    let symbol_parenthesis_l    = descents.create(DescentToken::new(&elements::symbols::PARENTHESIS_L));
    let symbol_parenthesis_r    = descents.create(DescentToken::new(&elements::symbols::PARENTHESIS_R));
    let symbol_brace_l          = descents.create(DescentToken::new(&elements::symbols::BRACE_L));
    let symbol_brace_r          = descents.create(DescentToken::new(&elements::symbols::BRACE_R));
    let symbol_crotchet_l       = descents.create(DescentToken::new(&elements::symbols::CROTCHET_L));
    let symbol_crotchet_r       = descents.create(DescentToken::new(&elements::symbols::CROTCHET_R));
    let symbol_dot              = descents.create(DescentToken::new(&elements::symbols::DOT));
    let symbol_dot_d            = descents.create(DescentToken::new(&elements::symbols::DOT_D));
    let symbol_dot_d_eq         = descents.create(DescentToken::new(&elements::symbols::DOT_D_EQ));
    let symbol_dot_t            = descents.create(DescentToken::new(&elements::symbols::DOT_T));
    let symbol_comma            = descents.create(DescentToken::new(&elements::symbols::COMMA));
    let symbol_tilde            = descents.create(DescentToken::new(&elements::symbols::TILDE));
    let symbol_dollar           = descents.create(DescentToken::new(&elements::symbols::DOLLAR));
    let symbol_interrogation    = descents.create(DescentToken::new(&elements::symbols::INTERROGATION));
    let symbol_colon            = descents.create(DescentToken::new(&elements::symbols::COLON));
    let symbol_semicolon        = descents.create(DescentToken::new(&elements::symbols::SEMICOLON));
    let symbol_backslash        = descents.create(DescentToken::new(&elements::symbols::BACKSLASH));
    let variable_identifier     = descents.create(DescentToken::new(&elements::variables::IDENTIFIER));
    let variable_string         = descents.create(DescentToken::new(&elements::variables::STRING));
    let variable_number         = descents.create(DescentToken::new(&elements::variables::NUMBER));

    let statements = descents.declare();

    let expression = descents.declare();

    let expression_base = descents.declare();

    let binop_base = descents.declare();

    let expression_option = descents.declare();

    let extension = ascents.declare();

    let name = descents.create(DescentElement::new(
        descents.create(DescentOption::new(variable_identifier)),
        &elements::productions::NAME
    ));

    macro_rules! create_control {
        ( $keyword:expr, $element:expr ) => {
            descents.create(DescentElement::new(
                descents.create(DescentSequence::new([$keyword, expression_option])),
                $element
            ))
        }
    }

    macro_rules! create_binop_ascent {
        ( $child:expr, $tokens:expr ) => {{
            let ascent = ascents.declare();
            ascents.define(ascent, AscentExtension::new(
                descents.create(DescentSequence::new([
                    descents.create(DescentChoice::new($tokens)),
                    $child,
                ])),
                ascents.create(AscentList::new([
                    ascent,
                    ascents.create(AscentElement::new(&elements::expressions::EXPRESSION)),
                    ascents.create(AscentElement::new(&elements::expressions::BINOP)),
                ]))
            ));

            ascent
        }}
    }

    macro_rules! create_binop {
        ( $child:expr, $tokens:expr ) => {{
            descents.create(DescentAscent::new($child, create_binop_ascent!($child, $tokens)))
        }}
    }

    macro_rules! create_assignment {
        ( $child:expr, $tokens:expr ) => {
            descents.create(DescentAscent::new($child, {
                let ascent = ascents.declare();
                ascents.define(ascent, AscentExtension::new(
                    descents.create(DescentSequence::new([
                        descents.create(DescentChoice::new($tokens)),
                        $child,
                    ])),
                    ascents.create(AscentList::new([
                        ascent,
                        ascents.create(AscentElement::new(&elements::expressions::EXPRESSION)),
                        ascents.create(AscentElement::new(&elements::expressions::ASSIGNMENT)),
                    ]))
                ));

                ascent
            }))
        }
    }

    macro_rules! create_list {
        ( $element:expr, $separator:expr ) => {{
            descents.create(DescentSequence::new([
                $element,
                descents.create(DescentZeroOrMore::new(
                    descents.create(DescentSequence::new([$separator, $element]))
                ))
            ]))
        }}
    }

    macro_rules! create_list_option {
        ( $element:expr, $separator:expr ) => {{
            descents.create(DescentOption::new(
                create_list!($element, $separator)
            ))
        }}
    }

    let expressions = descents.create(DescentElement::new(
        create_list_option!(expression, symbol_comma),
        &elements::productions::EXPRESSIONS
    ));

    let r#type = descents.create(DescentElement::new(
        descents.create(DescentOption::new(
            descents.create(DescentSequence::new([symbol_colon, binop_base])),
        )),
        &elements::productions::TYPE
    ));

    let literal = descents.create(DescentElement::new(
        descents.create(DescentChoice::new([variable_identifier, variable_string, variable_number, keyword_true, keyword_false])),
        &elements::expressions::LITERAL
    ));

    let array = descents.create(DescentElement::new(
        descents.create(DescentSequence::new([symbol_crotchet_l, expressions, symbol_crotchet_r])),
        &elements::expressions::ARRAY
    ));

    let group = descents.create(DescentElement::new(
        descents.create(DescentSequence::new([symbol_parenthesis_l, expression, symbol_parenthesis_r])),
        &elements::expressions::GROUP
    ));

    let declaration = descents.create(DescentElement::new(
        descents.create(DescentSequence::new([variable_identifier, r#type])),
        &elements::productions::DECLARATION
    ));

    let r#let = descents.create(DescentElement::new(
        descents.create(DescentSequence::new([keyword_let, declaration])),
        &elements::expressions::LET
    ));

    let control = descents.create(DescentElement::new(
        descents.create(DescentChoice::new([
            create_control!(keyword_return,   &elements::controls::RETURN),
            create_control!(keyword_break,    &elements::controls::BREAK),
            create_control!(keyword_continue, &elements::controls::CONTINUE),
        ])),
        &elements::controls::CONTROL
    ));

    let block = descents.create(DescentElement::new(
        descents.create(DescentSequence::new([symbol_brace_l, statements, expression_option, symbol_brace_r])),
        &elements::flows::BLOCK
    ));

    let r#if = descents.create(DescentElement::new(
        descents.create(DescentSequence::new([
            keyword_if,
            expression,
            block,
            descents.create(DescentOption::new(
                descents.create(DescentSequence::new([keyword_else, block])),
            )),
        ])),
        &elements::flows::IF
    ));

    let r#loop = descents.create(DescentElement::new(
        descents.create(DescentSequence::new([keyword_loop, block])),
        &elements::flows::LOOP
    ));

    let r#while = descents.create(DescentElement::new(
        descents.create(DescentSequence::new([keyword_while, expression, block])),
        &elements::flows::WHILE
    ));

    let for_in = descents.create(DescentElement::new(
        descents.create(DescentSequence::new([keyword_for, variable_identifier, keyword_in, expression, block])),
        &elements::flows::FOR_IN
    ));

    let flow = descents.create(DescentElement::new(
        descents.create(DescentChoice::new([block, r#if, r#loop, r#while, for_in])),
        &elements::flows::FLOW
    ));

    let generics = descents.create(DescentOption::new(
        descents.create(DescentSequence::new([
            symbol_guillemet_l,
            descents.create(DescentAscent::new(
                descents.create(DescentOneOrMore::new(
                    create_list!(variable_identifier, symbol_comma)
                )),
                ascents.create(AscentElement::new(&elements::productions::GENERICS))
            )),
            symbol_guillemet_r,
        ]))
    ));

    let parameters = descents.create(DescentElement::new(
        create_list_option!(declaration, symbol_comma),
        &elements::productions::PARAMETERS
    ));

    let function = descents.create(DescentElement::new(
        descents.create(DescentSequence::new([keyword_function, name, generics, symbol_parenthesis_l, parameters, symbol_parenthesis_r, r#type, block])),
        &elements::structures::FUNCTION
    ));

    let function_named = descents.create(DescentElement::new(
        descents.create(DescentSequence::new([keyword_function, variable_identifier, generics, symbol_parenthesis_l, parameters, symbol_parenthesis_r, r#type, block])),
        &elements::structures::FUNCTION
    ));

    let methods = descents.create(DescentElement::new(
        descents.create(DescentZeroOrMore::new(function_named)),
        &elements::productions::METHODS
    ));

    let class = descents.create(DescentElement::new(
        descents.create(DescentSequence::new([keyword_class, name, generics, r#type, symbol_brace_l, methods, symbol_brace_r])),
        &elements::structures::CLASS
    ));

    let class_named = descents.create(DescentElement::new(
        descents.create(DescentSequence::new([keyword_class, variable_identifier, generics, r#type, symbol_brace_l, methods, symbol_brace_r])),
        &elements::structures::CLASS
    ));

    let structure = descents.create(DescentElement::new(
        descents.create(DescentChoice::new([class_named, function_named])),
        &elements::structures::STRUCTURE
    ));

    let chain = ascents.create(AscentExtension::new(
        descents.create(DescentSequence::new([
            symbol_dot,
            variable_identifier,
        ])),
        ascents.create(AscentList::new([
            extension,
            ascents.create(AscentElement::new(&elements::expressions::EXPRESSION)),
            ascents.create(AscentElement::new(&elements::expressions::CHAIN)),
        ]))
    ));

    let sequence = ascents.create(AscentExtension::new(
        descents.create(DescentChoice::new([
            descents.create(DescentSequence::new([symbol_parenthesis_l, expressions, symbol_parenthesis_r])),
            descents.create(DescentSequence::new([symbol_crotchet_l, expressions, symbol_crotchet_r])),
            descents.create(DescentSequence::new([
                symbol_guillemet_l,
                descents.create(DescentElement::new(
                    create_list!(expression_base, symbol_comma),
                    &elements::productions::EXPRESSIONS
                )),
                symbol_guillemet_r,
            ])),
        ])),
        ascents.create(AscentList::new([
            extension,
            ascents.create(AscentElement::new(&elements::expressions::EXPRESSION)),
            ascents.create(AscentElement::new(&elements::expressions::SEQUENCE)),
        ]))
    ));

    let preop = descents.create(DescentElement::new(
        descents.create(DescentSequence::new([
            descents.create(DescentChoice::new([symbol_tilde, symbol_minus, symbol_plus, symbol_exclamation])),
            expression_base
        ])),
        &elements::expressions::PREOP
    ));

    descents.define(expression_base, DescentElement::new(
        descents.create(DescentChoice::new([class, function, flow, control, r#let, array, group, literal, preop])),
        &elements::expressions::EXPRESSION
    ));

    ascents.define(extension, AscentList::new([chain, sequence]));

    let binop_1 = create_binop!(descents.create(DescentAscent::new(
        expression_base,
        extension,
    )), [symbol_asterisk, symbol_slash, symbol_percent, symbol_asterisk_d]);

    descents.define(binop_base, DescentAlias::new(binop_1));

    let binop_2  = create_binop!(binop_1, [symbol_plus, symbol_minus]);

    let binop_3  = create_binop!(binop_2, [symbol_guillemet_l_d, symbol_guillemet_r_d, symbol_guillemet_l_t, symbol_guillemet_l_t]);

    let binop_4  = create_binop!(binop_3, [symbol_ampersand]);

    let binop_5  = create_binop!(binop_4, [symbol_caret]);

    let binop_6  = create_binop!(binop_5, [symbol_pipe]);

    let binop_7_l = create_binop_ascent!(binop_6, [symbol_guillemet_l, symbol_guillemet_l_eq]);

    let binop_7_r = create_binop_ascent!(binop_6, [symbol_guillemet_r, symbol_guillemet_r_eq]);

    let binop_7 = descents.create(DescentChoice::new([
        descents.create(DescentSequence::new([
            descents.create(DescentAscent::new(binop_6, binop_7_l)),
            descents.create(DescentPredicateNot::new(
                descents.create(DescentChoice::new([symbol_guillemet_r, symbol_guillemet_r_eq]))
            )),
        ])),
        descents.create(DescentSequence::new([
            descents.create(DescentAscent::new(binop_6, binop_7_r)),
            descents.create(DescentPredicateNot::new(
                descents.create(DescentChoice::new([symbol_guillemet_l, symbol_guillemet_l_eq]))
            )),
        ])),
    ]));

    let binop_8  = create_binop!(binop_7,  [symbol_equal_d, symbol_exclamation_eq]);

    let binop_9  = create_binop!(binop_8,  [symbol_ampersand_d]);

    let binop_10 = create_binop!(binop_9,  [symbol_pipe_d]);

    let binop_11 = create_binop!(binop_10, [symbol_dot_d, symbol_dot_d_eq]);

    let binop_12 = create_assignment!(binop_11, [symbol_equal, symbol_plus_eq, symbol_minus_eq, symbol_asterisk_eq, symbol_slash_eq,
        symbol_percent_eq, symbol_asterisk_d_eq, symbol_guillemet_l_d_eq, symbol_guillemet_r_d_eq, symbol_guillemet_l_t_eq, symbol_guillemet_r_t_eq,
        symbol_ampersand_eq, symbol_caret_eq, symbol_pipe_eq, symbol_ampersand_d_eq, symbol_pipe_d_eq
    ]);

    descents.define(expression, DescentAlias::new(binop_12));

    descents.define(expression_option, DescentOption::new(expression));

    let no_semicolon = descents.create(DescentPredicateNot::new(symbol_semicolon));

    let statement = descents.create(DescentElement::new(
        descents.create(DescentChoice::new([
            descents.create(DescentSequence::new([structure, no_semicolon])),
            descents.create(DescentSequence::new([flow, no_semicolon])),
            descents.create(DescentSequence::new([expression, symbol_semicolon])),
        ])),
        &elements::productions::STATEMENT
    ));

    descents.define(statements, DescentElement::new(
        descents.create(DescentZeroOrMore::new(statement)),
        &elements::productions::STATEMENTS
    ));

    let program = descents.create(DescentElement::new(
        statements,
        &elements::productions::PROGRAM
    ));

    (descents, ascents, program)
}
