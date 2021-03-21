use crate::elements;
use crate::parser::arena::Arena;
use crate::parser::ascent::*;
use crate::parser::descent::*;

pub fn get() -> (Arena::<dyn Descent>, Arena::<dyn Ascent>) {
    let descents = Arena::<dyn Descent>::new();
    let ascents = Arena::<dyn Ascent>::new();

    let program = descents.declare(); // 0

    let expression = descents.declare(); // 1

    let statements = descents.declare();

    let expression_base = descents.declare();

    let binop_base = descents.declare();

    let expression_option = descents.declare();

    let extension = ascents.declare();

    macro_rules! descent_alias {
        ( $descent:expr ) => { descents.define(DescentAlias::new($descent)) }
    }

    macro_rules! descent_ascent {
        ( $ascent:expr ) => { descents.define(DescentAscent::new($ascent)) }
    }

    macro_rules! descent_sequence {
        [ $($descents:expr),+ $(,)? ] => { descents.define(DescentSequence::new(Box::from([$($descents),+]))) }
    }

    macro_rules! descent_choice {
        [ $($descents:expr),+ $(,)? ] => { descents.define(DescentChoice::new(Box::from([$($descents),+]))) }
    }

    macro_rules! descent_zero_or_more {
        ( $descent:expr ) => { descents.define(DescentZeroOrMore::new($descent)) }
    }

    macro_rules! descent_option {
        ( $descent:expr ) => { descents.define(DescentOption::new($descent)) }
    }

    macro_rules! descent_predicate_not {
        ( $descent:expr ) => { descents.define(DescentPredicateNot::new($descent)) }
    }

    macro_rules! descent_element {
        ( $descent:expr, $element:expr ) => { descents.define(DescentElement::new($descent, $element)) }
    }

    macro_rules! descent_token {
        ( $element:expr ) => { descents.define(DescentToken::new($element)) }
    }

    macro_rules! ascent_descent {
        ( $descent:expr ) => { ascents.define(AscentDescent::new($descent)) }
    }

    macro_rules! ascent_sequence {
        [ $($ascents:expr),+ $(,)? ] => { ascents.define(AscentSequence::new(Box::from([$($ascents),+]))) }
    }

    macro_rules! ascent_choice {
        [ $($ascents:expr),+ $(,)? ] => { ascents.define(AscentChoice::new(Box::from([$($ascents),+]))) }
    }

    macro_rules! ascent_option {
        ( $ascent:expr ) => { ascents.define(AscentOption::new($ascent)) }
    }
    macro_rules! ascent_element {
        ( $element:expr ) => { ascents.define(AscentElement::new($element)) }
    }

    let keyword_as              = descent_token!(&elements::keywords::AS);
    let keyword_catch           = descent_token!(&elements::keywords::CATCH);
    let keyword_class           = descent_token!(&elements::keywords::CLASS);
    let keyword_default         = descent_token!(&elements::keywords::DEFAULT);
    let keyword_do              = descent_token!(&elements::keywords::DO);
    let keyword_else            = descent_token!(&elements::keywords::ELSE);
    let keyword_export          = descent_token!(&elements::keywords::EXPORT);
    let keyword_false           = descent_token!(&elements::keywords::FALSE);
    let keyword_finally         = descent_token!(&elements::keywords::FINALLY);
    let keyword_for             = descent_token!(&elements::keywords::FOR);
    let keyword_from            = descent_token!(&elements::keywords::FROM);
    let keyword_function        = descent_token!(&elements::keywords::FUNCTION);
    let keyword_if              = descent_token!(&elements::keywords::IF);
    let keyword_import          = descent_token!(&elements::keywords::IMPORT);
    let keyword_in              = descent_token!(&elements::keywords::IN);
    let keyword_let             = descent_token!(&elements::keywords::LET);
    let keyword_loop            = descent_token!(&elements::keywords::LOOP);
    let keyword_private         = descent_token!(&elements::keywords::PRIVATE);
    let keyword_protected       = descent_token!(&elements::keywords::PROTECTED);
    let keyword_public          = descent_token!(&elements::keywords::PUBLIC);
    let keyword_continue        = descent_token!(&elements::keywords::CONTINUE);
    let keyword_break           = descent_token!(&elements::keywords::BREAK);
    let keyword_return          = descent_token!(&elements::keywords::RETURN);
    let keyword_static          = descent_token!(&elements::keywords::STATIC);
    let keyword_then            = descent_token!(&elements::keywords::THEN);
    let keyword_throw           = descent_token!(&elements::keywords::THROW);
    let keyword_true            = descent_token!(&elements::keywords::TRUE);
    let keyword_try             = descent_token!(&elements::keywords::TRY);
    let keyword_while           = descent_token!(&elements::keywords::WHILE);
    let symbol_plus             = descent_token!(&elements::symbols::PLUS);
    let symbol_plus_eq          = descent_token!(&elements::symbols::PLUS_EQ);
    let symbol_minus            = descent_token!(&elements::symbols::MINUS);
    let symbol_minus_eq         = descent_token!(&elements::symbols::MINUS_EQ);
    let symbol_asterisk         = descent_token!(&elements::symbols::ASTERISK);
    let symbol_asterisk_eq      = descent_token!(&elements::symbols::ASTERISK_EQ);
    let symbol_asterisk_d       = descent_token!(&elements::symbols::ASTERISK_D);
    let symbol_asterisk_d_eq    = descent_token!(&elements::symbols::ASTERISK_D_EQ);
    let symbol_slash            = descent_token!(&elements::symbols::SLASH);
    let symbol_slash_eq         = descent_token!(&elements::symbols::SLASH_EQ);
    let symbol_percent          = descent_token!(&elements::symbols::PERCENT);
    let symbol_percent_eq       = descent_token!(&elements::symbols::PERCENT_EQ);
    let symbol_caret            = descent_token!(&elements::symbols::CARET);
    let symbol_caret_eq         = descent_token!(&elements::symbols::CARET_EQ);
    let symbol_exclamation      = descent_token!(&elements::symbols::EXCLAMATION);
    let symbol_exclamation_eq   = descent_token!(&elements::symbols::EXCLAMATION_EQ);
    let symbol_equal            = descent_token!(&elements::symbols::EQUAL);
    let symbol_equal_d          = descent_token!(&elements::symbols::EQUAL_D);
    let symbol_arrow            = descent_token!(&elements::symbols::ARROW);
    let symbol_pipe             = descent_token!(&elements::symbols::PIPE);
    let symbol_pipe_eq          = descent_token!(&elements::symbols::PIPE_EQ);
    let symbol_pipe_d           = descent_token!(&elements::symbols::PIPE_D);
    let symbol_pipe_d_eq        = descent_token!(&elements::symbols::PIPE_D_EQ);
    let symbol_ampersand        = descent_token!(&elements::symbols::AMPERSAND);
    let symbol_ampersand_eq     = descent_token!(&elements::symbols::AMPERSAND_EQ);
    let symbol_ampersand_d      = descent_token!(&elements::symbols::AMPERSAND_D);
    let symbol_ampersand_d_eq   = descent_token!(&elements::symbols::AMPERSAND_D_EQ);
    let symbol_guillemet_l      = descent_token!(&elements::symbols::GUILLEMET_L);
    let symbol_guillemet_r      = descent_token!(&elements::symbols::GUILLEMET_R);
    let symbol_guillemet_l_eq   = descent_token!(&elements::symbols::GUILLEMET_L_EQ);
    let symbol_guillemet_r_eq   = descent_token!(&elements::symbols::GUILLEMET_R_EQ);
    let symbol_guillemet_l_d    = descent_token!(&elements::symbols::GUILLEMET_L_D);
    let symbol_guillemet_r_d    = descent_token!(&elements::symbols::GUILLEMET_R_D);
    let symbol_guillemet_l_d_eq = descent_token!(&elements::symbols::GUILLEMET_L_D_EQ);
    let symbol_guillemet_r_d_eq = descent_token!(&elements::symbols::GUILLEMET_R_D_EQ);
    let symbol_guillemet_l_t    = descent_token!(&elements::symbols::GUILLEMET_L_T);
    let symbol_guillemet_r_t    = descent_token!(&elements::symbols::GUILLEMET_R_T);
    let symbol_guillemet_l_t_eq = descent_token!(&elements::symbols::GUILLEMET_L_T_EQ);
    let symbol_guillemet_r_t_eq = descent_token!(&elements::symbols::GUILLEMET_R_T_EQ);
    let symbol_parenthesis_l    = descent_token!(&elements::symbols::PARENTHESIS_L);
    let symbol_parenthesis_r    = descent_token!(&elements::symbols::PARENTHESIS_R);
    let symbol_brace_l          = descent_token!(&elements::symbols::BRACE_L);
    let symbol_brace_r          = descent_token!(&elements::symbols::BRACE_R);
    let symbol_crotchet_l       = descent_token!(&elements::symbols::CROTCHET_L);
    let symbol_crotchet_r       = descent_token!(&elements::symbols::CROTCHET_R);
    let symbol_dot              = descent_token!(&elements::symbols::DOT);
    let symbol_dot_d            = descent_token!(&elements::symbols::DOT_D);
    let symbol_dot_d_eq         = descent_token!(&elements::symbols::DOT_D_EQ);
    let symbol_dot_t            = descent_token!(&elements::symbols::DOT_T);
    let symbol_comma            = descent_token!(&elements::symbols::COMMA);
    let symbol_tilde            = descent_token!(&elements::symbols::TILDE);
    let symbol_dollar           = descent_token!(&elements::symbols::DOLLAR);
    let symbol_interrogation    = descent_token!(&elements::symbols::INTERROGATION);
    let symbol_colon            = descent_token!(&elements::symbols::COLON);
    let symbol_semicolon        = descent_token!(&elements::symbols::SEMICOLON);
    let symbol_backslash        = descent_token!(&elements::symbols::BACKSLASH);
    let variable_integer        = descent_token!(&elements::variables::INTEGER);
    let variable_float          = descent_token!(&elements::variables::FLOAT);
    let variable_string         = descent_token!(&elements::variables::STRING);
    let variable_identifier     = descent_token!(&elements::variables::IDENTIFIER);

    macro_rules! macro_control {
        ( $keyword:expr, $element:expr ) => {
            descent_element!(
                descent_sequence![$keyword, expression_option],
                $element
            )
        }
    }

    macro_rules! macro_binop_1 {
        ( $child:expr, $tokens:expr ) => {{
            let ascent = ascents.declare();
            ascents.swap(ascent, ascent_option!(
                ascent_sequence![
                    ascent_descent!(
                        descent_sequence![
                            descent_choice![$tokens],
                            $child,
                        ]
                    ),
                    ascent_sequence![
                        ascent_element!(&elements::expressions::BINOP),
                        ascent_element!(&elements::expressions::EXPRESSION),
                        ascent,
                    ],
                ]
            ));

            ascent
        }}
    }

    macro_rules! macro_binop_2 {
        ( $child:expr, $tokens:expr ) => {{
            descent_ascent!(
                ascent_sequence![
                    ascent_descent!($child),
                    macro_binop_1!($child, $tokens),
                ]
            )
        }}
    }

    macro_rules! macro_assignment {
        ( $child:expr, $tokens:expr ) => {{
            let ascent = ascents.declare();
            ascents.swap(ascent, ascent_option!(
                ascent_sequence![
                    ascent_descent!(
                        descent_sequence![
                            $tokens,
                            $child,
                        ]
                    ),
                    ascent_sequence![
                        ascent_element!(&elements::expressions::ASSIGNMENT),
                        ascent_element!(&elements::expressions::EXPRESSION),
                        ascent,
                    ],
                ]
            ));

            descent_ascent!(
                ascent_sequence![
                    ascent_descent!($child),
                    ascent,
                ]
            )
        }}
    }

    macro_rules! macro_list {
        ( $element:expr, $separator:expr ) => {{
            descent_sequence![
                $element,
                descent_zero_or_more!(
                    descent_sequence![$separator, $element]
                )
            ]
        }}
    }

    let name = descent_element!(
        descent_option!(variable_identifier),
        &elements::productions::NAME
    );

    let expressions = descent_element!(
        descent_option!(
            macro_list!(expression, symbol_comma)
        ),
        &elements::productions::EXPRESSIONS
    );

    let r#type = descent_element!(
        descent_option!(
            descent_sequence![symbol_colon, binop_base]
        ),
        &elements::productions::TYPE
    );

    let literal = descent_element!(
        descent_choice![keyword_true, keyword_false, variable_integer, variable_float, variable_string, variable_identifier],
        &elements::expressions::LITERAL
    );

    let array = descent_element!(
        descent_sequence![symbol_crotchet_l, expressions, symbol_crotchet_r],
        &elements::expressions::ARRAY
    );

    let group = descent_element!(
        descent_sequence![symbol_parenthesis_l, expression, symbol_parenthesis_r],
        &elements::expressions::GROUP
    );

    let declaration = descent_element!(
        descent_sequence![variable_identifier, r#type],
        &elements::productions::DECLARATION
    );

    let r#let = descent_element!(
        descent_sequence![keyword_let, declaration],
        &elements::expressions::LET
    );

    let control = descent_element!(
        descent_choice![
            macro_control!(keyword_return,   &elements::controls::RETURN),
            macro_control!(keyword_break,    &elements::controls::BREAK),
            macro_control!(keyword_continue, &elements::controls::CONTINUE),
        ],
        &elements::controls::CONTROL
    );

    let block = descent_element!(
        descent_sequence![symbol_brace_l, statements, expression_option, symbol_brace_r],
        &elements::flows::BLOCK
    );

    let r#if = descent_element!(
        descent_sequence![
            keyword_if,
            expression,
            block,
            descent_option!(
                descent_sequence![keyword_else, block]
            ),
        ],
        &elements::flows::IF
    );

    let r#loop = descent_element!(
        descent_sequence![keyword_loop, block],
        &elements::flows::LOOP
    );

    let r#while = descent_element!(
        descent_sequence![keyword_while, expression, block],
        &elements::flows::WHILE
    );

    let for_in = descent_element!(
        descent_sequence![keyword_for, variable_identifier, keyword_in, expression, block],
        &elements::flows::FOR_IN
    );

    let flow = descent_element!(
        descent_choice![block, r#if, r#loop, r#while, for_in],
        &elements::flows::FLOW
    );

    let generics = descent_option!(
        descent_element!(
            descent_sequence![
                symbol_guillemet_l,
                descent_element!(
                    macro_list!(variable_identifier, symbol_comma),
                    &elements::productions::GENERICS_LIST
                ),
                symbol_guillemet_r,
            ],
            &elements::productions::GENERICS
        )
    );

    let rest = descent_element!(
        descent_option!(
            descent_sequence![
                symbol_dot_t,
                declaration,
            ]
        ),
        &elements::productions::REST
    );

    let parameters_end = ascent_sequence![
        ascent_element!(&elements::productions::PARAMETERS_LIST),
        ascent_descent!(rest),
    ];

    let parameters_more = ascents.declare();
    ascents.swap(parameters_more, ascent_choice![
        ascent_sequence![
            ascent_descent!(declaration),
            ascent_choice![
                ascent_sequence![
                    ascent_descent!(symbol_comma),
                    parameters_more,
                ],
                parameters_end,
            ]
        ],
        parameters_end,
    ]);

    let parameters = descent_element!(
        descent_sequence![
            symbol_parenthesis_l,
            descent_ascent!(parameters_more),
            symbol_parenthesis_r,
        ],
        &elements::productions::PARAMETERS
    );

    let function = descent_element!(
        descent_sequence![keyword_function, name, generics, parameters, r#type, block],
        &elements::structures::FUNCTION
    );

    let function_named = descent_element!(
        descent_sequence![keyword_function, variable_identifier, generics, parameters, r#type, block],
        &elements::structures::FUNCTION
    );

    let methods = descent_element!(
        descent_zero_or_more!(function_named),
        &elements::productions::METHODS
    );

    let class = descent_element!(
        descent_sequence![keyword_class, name, generics, r#type, symbol_brace_l, methods, symbol_brace_r],
        &elements::structures::CLASS
    );

    let class_named = descent_element!(
        descent_sequence![keyword_class, variable_identifier, generics, r#type, symbol_brace_l, methods, symbol_brace_r],
        &elements::structures::CLASS
    );

    let structure = descent_element!(
        descent_choice![class_named, function_named],
        &elements::structures::STRUCTURE
    );

    let preop = descent_element!(
        descent_sequence![
            descent_choice![symbol_tilde, symbol_minus, symbol_plus, symbol_exclamation],
            expression_base
        ],
        &elements::expressions::PREOP
    );

    let expression_core = descent_element!(
        descent_choice![class, function, flow, control, r#let, array, group, literal, preop],
        &elements::expressions::EXPRESSION
    );

    let chain = ascent_sequence![
        ascent_descent!(
            descent_sequence![
                symbol_dot,
                variable_identifier,
            ]
        ),
        ascent_sequence![
            ascent_element!(&elements::expressions::CHAIN),
            ascent_element!(&elements::expressions::EXPRESSION),
            extension,
        ],
    ];

    let sequence = ascent_sequence![
        ascent_descent!(
            descent_choice![
                descent_sequence![symbol_parenthesis_l, expressions, symbol_parenthesis_r],
                descent_sequence![symbol_crotchet_l, expressions, symbol_crotchet_r],
                descent_sequence![
                    symbol_guillemet_l,
                    descent_element!(
                        macro_list!(expression_base, symbol_comma),
                        &elements::productions::EXPRESSIONS
                    ),
                    symbol_guillemet_r,
                ],
            ]
        ),
        ascent_sequence![
            ascent_element!(&elements::expressions::SEQUENCE),
            ascent_element!(&elements::expressions::EXPRESSION),
            extension,
        ],
    ];

    ascents.swap(extension, ascent_option!(
        ascent_choice![chain, sequence]
    ));

    descents.swap(expression_base, descent_ascent!(
        ascent_sequence![
            ascent_descent!(expression_core),
            extension,
        ]
    ));

    let binop_1 = macro_binop_2!(expression_base, descent_choice![symbol_asterisk, symbol_slash, symbol_percent, symbol_asterisk_d]);

    descents.swap(binop_base, descent_alias!(binop_1));

    let binop_2  = macro_binop_2!(binop_1, descent_choice![symbol_plus, symbol_minus]);

    let binop_3  = macro_binop_2!(binop_2, descent_choice![symbol_guillemet_l_d, symbol_guillemet_r_d, symbol_guillemet_l_t, symbol_guillemet_l_t]);

    let binop_4  = macro_binop_2!(binop_3, descent_choice![symbol_ampersand]);

    let binop_5  = macro_binop_2!(binop_4, descent_choice![symbol_caret]);

    let binop_6  = macro_binop_2!(binop_5, descent_choice![symbol_pipe]);

    let binop_7_l = macro_binop_1!(binop_6, descent_choice![symbol_guillemet_l, symbol_guillemet_l_eq]);

    let binop_7_r = macro_binop_1!(binop_6, descent_choice![symbol_guillemet_r, symbol_guillemet_r_eq]);

    let binop_7 = descent_choice![
        descent_sequence![
            descent_ascent!(
                ascent_sequence![
                    ascent_descent!(binop_6),
                    binop_7_l,
                ]
            ),
            descent_predicate_not!(
                descent_choice![symbol_guillemet_r, symbol_guillemet_r_eq]
            ),
        ],
        descent_sequence![
            descent_ascent!(
                ascent_sequence![
                    ascent_descent!(binop_6),
                    binop_7_r,
                ]
            ),
            descent_predicate_not!(
                descent_choice![symbol_guillemet_l, symbol_guillemet_l_eq]
            ),
        ],
    ];

    let binop_8  = macro_binop_2!(binop_7,  descent_choice![symbol_equal_d, symbol_exclamation_eq]);

    let binop_9  = macro_binop_2!(binop_8,  descent_choice![symbol_ampersand_d]);

    let binop_10 = macro_binop_2!(binop_9,  descent_choice![symbol_pipe_d]);

    let binop_11 = macro_binop_2!(binop_10, descent_choice![symbol_dot_d, symbol_dot_d_eq]);

    let binop_12 = macro_assignment!(binop_11, descent_choice![symbol_equal, symbol_plus_eq, symbol_minus_eq, symbol_asterisk_eq, symbol_slash_eq,
        symbol_percent_eq, symbol_asterisk_d_eq, symbol_guillemet_l_d_eq, symbol_guillemet_r_d_eq, symbol_guillemet_l_t_eq, symbol_guillemet_r_t_eq,
        symbol_ampersand_eq, symbol_caret_eq, symbol_pipe_eq, symbol_ampersand_d_eq, symbol_pipe_d_eq
    ]);

    descents.swap(expression, descent_alias!(binop_12));

    descents.swap(expression_option, descent_option!(expression));

    let no_semicolon = descent_predicate_not!(symbol_semicolon);

    let statement = descent_element!(
        descent_choice![
            descent_sequence![structure, no_semicolon],
            descent_sequence![flow, no_semicolon],
            descent_sequence![expression, symbol_semicolon],
        ],
        &elements::productions::STATEMENT
    );

    descents.swap(statements, descent_element!(
        descent_zero_or_more!(statement),
        &elements::productions::STATEMENTS
    ));

    descents.swap(program, descent_element!(
        statements,
        &elements::productions::PROGRAM
    ));

    (descents, ascents)
}
