#![allow(unused_variables)]

use crate::memory::Ref;
use crate::parser::{ Code, Parse, SNode };
use crate::parser::arena::{ Arena, ArenaRef };
use crate::parser::elements;
use crate::parser::ascent::*;
use crate::parser::descent::*;
use crate::parser::lexer::lex;

pub struct Grammar {
    pub descents: Arena<dyn Descent>,
    pub ascents: Arena<dyn Ascent>,
    pub program: ArenaRef<dyn Descent>,
    pub expression: ArenaRef<dyn Descent>,
}

impl Grammar {
    pub fn new(descents: Arena::<dyn Descent>, ascents: Arena::<dyn Ascent>, program: ArenaRef<dyn Descent>, expression: ArenaRef<dyn Descent>) -> Self {
        Self {
            descents,
            ascents,
            program,
            expression,
        }
    }

    pub fn parse(&self, production: ArenaRef<dyn Descent>, code: Ref<Code>) -> Option<SNode> {
        let tokens = lex(code);
        let mut parse = Parse::new(self, code, &tokens);
        parse.parse(production)
    }
}

pub fn get() -> Grammar {
    let descents = Arena::<dyn Descent>::new();
    let ascents = Arena::<dyn Ascent>::new();

    let program = descents.declare();

    let expression = descents.declare();

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

    let keyword_break           = descent_token!(&elements::keywords::BREAK);
    let keyword_class           = descent_token!(&elements::keywords::CLASS);
    let keyword_continue        = descent_token!(&elements::keywords::CONTINUE);
    let keyword_else            = descent_token!(&elements::keywords::ELSE);
    let keyword_false           = descent_token!(&elements::keywords::FALSE);
    let keyword_for             = descent_token!(&elements::keywords::FOR);
    let keyword_function        = descent_token!(&elements::keywords::FUNCTION);
    let keyword_if              = descent_token!(&elements::keywords::IF);
    let keyword_in              = descent_token!(&elements::keywords::IN);
    let keyword_let             = descent_token!(&elements::keywords::LET);
    let keyword_loop            = descent_token!(&elements::keywords::LOOP);
    let keyword_return          = descent_token!(&elements::keywords::RETURN);
    let keyword_true            = descent_token!(&elements::keywords::TRUE);
    let keyword_while           = descent_token!(&elements::keywords::WHILE);
    let literal_integer        = descent_token!(&elements::literals::INTEGER);
    let literal_float          = descent_token!(&elements::literals::FLOAT);
    let literal_string         = descent_token!(&elements::literals::STRING);
    let literal_identifier     = descent_token!(&elements::literals::IDENTIFIER);
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

    macro_rules! macro_binop {
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
                        ascent_element!(&elements::productions::EXPRESSION),
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
                        ascent_element!(&elements::productions::EXPRESSION),
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
        descent_choice![keyword_true, keyword_false, literal_integer, literal_float, literal_string, literal_identifier],
        &elements::expressions::LITERAL
    );

    let declaration = descent_element!(
        descent_sequence![literal_identifier, r#type],
        &elements::productions::DECLARATION
    );

    let r#let = descent_element!(
        descent_sequence![keyword_let, declaration],
        &elements::expressions::LET
    );

    let jump = descent_element!(
        descent_sequence![
            descent_choice![keyword_continue, keyword_break, keyword_return],
            expression_option,
        ],
        &elements::expressions::JUMP
    );

    let block = descent_element!(
        descent_sequence![symbol_brace_l, statements, expression_option, symbol_brace_r],
        &elements::structures::BLOCK
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
        &elements::structures::IF
    );

    let r#loop = descent_element!(
        descent_sequence![keyword_loop, block],
        &elements::structures::LOOP
    );

    let r#while = descent_element!(
        descent_sequence![keyword_while, expression, block],
        &elements::structures::WHILE
    );

    let r#for = descent_element!(
        descent_sequence![keyword_for, literal_identifier, keyword_in, expression, block],
        &elements::structures::FOR
    );

    let structure = descent_element!(
        descent_choice![block, r#if, r#loop, r#while, r#for],
        &elements::productions::STRUCTURE
    );

    let generics = descent_element!(
        descent_option!(
            descent_sequence![
                symbol_crotchet_l,
                descent_element!(
                    macro_list!(literal_identifier, symbol_comma),
                    &elements::productions::GENERICS_LIST
                ),
                symbol_crotchet_r,
            ]
        ),
        &elements::productions::GENERICS
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
        descent_sequence![keyword_function, literal_identifier, generics, parameters, r#type, block],
        &elements::definitions::FUNCTION
    );

    let methods = descent_element!(
        descent_zero_or_more!(function),
        &elements::productions::METHODS
    );

    let class = descent_element!(
        descent_sequence![keyword_class, literal_identifier, generics, r#type, symbol_brace_l, methods, symbol_brace_r],
        &elements::definitions::CLASS
    );

    let definition = descent_element!(
        descent_choice![class, function],
        &elements::productions::DEFINITION
    );

    let preop = descent_element!(
        descent_sequence![
            descent_choice![symbol_tilde, symbol_minus, symbol_plus, symbol_exclamation],
            expression_base
        ],
        &elements::expressions::PREOP
    );

    let expression_core = descent_element!(
        descent_choice![structure, jump, r#let, literal, preop],
        &elements::productions::EXPRESSION
    );

    let chain = ascent_sequence![
        ascent_descent!(
            descent_sequence![
                symbol_dot,
                literal_identifier,
            ]
        ),
        ascent_sequence![
            ascent_element!(&elements::expressions::CHAIN),
            ascent_element!(&elements::productions::EXPRESSION),
            extension,
        ],
    ];

    let sequence = ascent_sequence![
        ascent_descent!(
            descent_choice![
                descent_sequence![symbol_parenthesis_l, expressions, symbol_parenthesis_r],
                descent_sequence![symbol_crotchet_l, expressions, symbol_crotchet_r],
            ]
        ),
        ascent_sequence![
            ascent_element!(&elements::expressions::SEQUENCE),
            ascent_element!(&elements::productions::EXPRESSION),
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

    let binop_1 = macro_binop!(expression_base, descent_choice![symbol_asterisk, symbol_slash, symbol_percent, symbol_asterisk_d]);

    descents.swap(binop_base, descent_alias!(binop_1));

    let binop_2  = macro_binop!(binop_1, descent_choice![symbol_plus, symbol_minus]);

    let binop_3  = macro_binop!(binop_2, descent_choice![symbol_guillemet_l_d, symbol_guillemet_r_d, symbol_guillemet_l_t, symbol_guillemet_r_t]);

    let binop_4  = macro_binop!(binop_3, descent_choice![symbol_ampersand]);

    let binop_5  = macro_binop!(binop_4, descent_choice![symbol_caret]);

    let binop_6  = macro_binop!(binop_5, descent_choice![symbol_pipe]);

    let binop_7  = macro_binop!(binop_6,  descent_choice![symbol_guillemet_l, symbol_guillemet_l_eq, symbol_guillemet_r, symbol_guillemet_r_eq]);

    let binop_8  = macro_binop!(binop_7,  descent_choice![symbol_equal_d, symbol_exclamation_eq]);

    let binop_9  = macro_binop!(binop_8,  descent_choice![symbol_ampersand_d]);

    let binop_10 = macro_binop!(binop_9,  descent_choice![symbol_pipe_d]);

    let binop_11 = macro_binop!(binop_10, descent_choice![symbol_dot_d, symbol_dot_d_eq]);

    let binop_12 = macro_assignment!(binop_11, descent_choice![symbol_equal, symbol_plus_eq, symbol_minus_eq, symbol_asterisk_eq, symbol_slash_eq,
        symbol_percent_eq, symbol_asterisk_d_eq, symbol_guillemet_l_d_eq, symbol_guillemet_r_d_eq, symbol_guillemet_l_t_eq, symbol_guillemet_r_t_eq,
        symbol_ampersand_eq, symbol_caret_eq, symbol_pipe_eq, symbol_ampersand_d_eq, symbol_pipe_d_eq
    ]);

    descents.swap(expression, descent_alias!(binop_12));

    descents.swap(expression_option, descent_option!(expression));

    let no_semicolon = descent_predicate_not!(symbol_semicolon);

    let statement = descent_element!(
        descent_choice![
            descent_sequence![definition, no_semicolon],
            descent_sequence![structure, no_semicolon],
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

    Grammar::new(descents, ascents, program, expression)
}
