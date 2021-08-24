use crate::parser::elements;
use crate::parser::lexer::node::Node;

pub const ROOT: Node = Node::new_null(&|character| {
    Some(match character {
        'b' => &KEYWORD_B,
        'c' => &KEYWORD_C,
        'e' => &KEYWORD_E,
        'f' => &KEYWORD_F,
        'i' => &KEYWORD_I,
        'l' => &KEYWORD_L,
        'r' => &KEYWORD_R,
        't' => &KEYWORD_T,
        'w' => &KEYWORD_W,
        '+' => &SYMBOL_PLUS,
        '-' => &SYMBOL_MINUS,
        '*' => &SYMBOL_ASTERISK,
        '/' => &SYMBOL_SLASH,
        '%' => &SYMBOL_PERCENT,
        '^' =>  &SYMBOL_CARET,
        '!' =>  &SYMBOL_EXCLAMATION,
        '=' =>  &SYMBOL_EQUAL,
        '|' =>  &SYMBOL_PIPE,
        '&' =>  &SYMBOL_AMPERSAND,
        '<' =>  &SYMBOL_GUILLEMET_L,
        '>' =>  &SYMBOL_GUILLEMET_R,
        '(' =>  &SYMBOL_PARENTHESIS_L,
        ')' =>  &SYMBOL_PARENTHESIS_R,
        '{' =>  &SYMBOL_BRACE_L,
        '}' =>  &SYMBOL_BRACE_R,
        '[' =>  &SYMBOL_CROTCHET_L,
        ']' =>  &SYMBOL_CROTCHET_R,
        '.' =>  &SYMBOL_DOT,
        ',' =>  &SYMBOL_COMMA,
        '~' =>  &SYMBOL_TILDE,
        '$' =>  &SYMBOL_DOLLAR,
        '?' =>  &SYMBOL_INTERROGATION,
        ':' =>  &SYMBOL_COLON,
        ';' =>  &SYMBOL_SEMICOLON,
        '\\' =>  &SYMBOL_BACKSLASH,
        '"' => &STRING_CONTENT,
        '0' ..= '9' => &NUMBER_BASE,
        ' ' | '\t' => &WHITESPACE,
        '\r' | '\n' => &ENDLINE,
        _ => match character {
            'a' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
            _ => return None,
        },
    })
});

const KEYWORD_B: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'r' => &KEYWORD_BR,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_BR: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_BRE,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_BRE: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'a' => &KEYWORD_BREA,
        'b' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_BREA: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'k' => &KEYWORD_BREAK,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_BREAK: Node = Node::new(&elements::keywords::BREAK, &|character| {
    Some(match character {
        'a' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_C: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'l' => &KEYWORD_CL,
        'o' => &KEYWORD_CO,
        'a' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CL: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'a' =>  &KEYWORD_CLA,
        'b' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CLA: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        's' => &KEYWORD_CLAS,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CLAS: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        's' => &KEYWORD_CLASS,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CLASS: Node = Node::new(&elements::keywords::CLASS, &|character| {
    Some(match character {
        'a' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CO: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'n' => &KEYWORD_CON,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CON: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        't' => &KEYWORD_CONT,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CONT: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'i' => &KEYWORD_CONTI,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CONTI: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'n' => &KEYWORD_CONTIN,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CONTIN: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'u' => &KEYWORD_CONTINU,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CONTINU: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_CONTINUE,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CONTINUE: Node = Node::new(&elements::keywords::CONTINUE, &|character| {
    Some(match character {
        'a' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_E: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'l' => &KEYWORD_EL,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_EL: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        's' => &KEYWORD_ELS,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_ELS: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_ELSE,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_ELSE: Node = Node::new(&elements::keywords::ELSE, &|character| {
    Some(match character {
        'a' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_F: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'a' => &KEYWORD_FA,
        'o' => &KEYWORD_FO,
        'u' => &KEYWORD_FU,
        'b' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FA: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'l' => &KEYWORD_FAL,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FAL: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        's' => &KEYWORD_FALS,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FALS: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_FALSE,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FALSE: Node = Node::new(&elements::keywords::FALSE, &|character| {
    Some(match character {
        'a' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FO: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'r' => &KEYWORD_FOR,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FOR: Node = Node::new(&elements::keywords::FOR, &|character| {
    Some(match character {
        'a' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FU: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'n' => &KEYWORD_FUN,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FUN: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'c' => &KEYWORD_FUNC,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FUNC: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        't' => &KEYWORD_FUNCT,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FUNCT: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'i' => &KEYWORD_FUNCTI,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FUNCTI: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'o' => &KEYWORD_FUNCTIO,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FUNCTIO: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'n' => &KEYWORD_FUNCTION,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FUNCTION: Node = Node::new(&elements::keywords::FUNCTION, &|character| {
    Some(match character {
        'a' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_I: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'f' => &KEYWORD_IF,
        'n' => &KEYWORD_IN,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_IF: Node = Node::new(&elements::keywords::IF, &|character| {
    Some(match character {
        'a' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_IN: Node = Node::new(&elements::keywords::IN, &|character| {
    Some(match character {
        'a' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_L: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_LE,
        'o' => &KEYWORD_LO,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_LE: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        't' => &KEYWORD_LET,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_LET: Node = Node::new(&elements::keywords::LET, &|character| {
    Some(match character {
        'a' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_LO: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'o' => &KEYWORD_LOO,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_LOO: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'p' => &KEYWORD_LOOP,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_LOOP: Node = Node::new(&elements::keywords::LOOP, &|character| {
    Some(match character {
        'a' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_R: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_RE,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_RE: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        't' => &KEYWORD_RET,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_RET: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'u' => &KEYWORD_RETU,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_RETU: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'r' => &KEYWORD_RETUR,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_RETUR: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'n' => &KEYWORD_RETURN,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_RETURN: Node = Node::new(&elements::keywords::RETURN, &|character| {
    Some(match character {
        'a' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_T: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'r' => &KEYWORD_TR,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_TR: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'u' => &KEYWORD_TRU,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_TRU: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_TRUE,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_TRUE: Node = Node::new(&elements::keywords::TRUE, &|character| {
    Some(match character {
        'a' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_W: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'h' => &KEYWORD_WH,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_WH: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'i' => &KEYWORD_WHI,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_WHI: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'l' => &KEYWORD_WHIL,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_WHIL: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_WHILE,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_WHILE: Node = Node::new(&elements::keywords::WHILE, &|character| {
    Some(match character {
        'a' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const SYMBOL_PLUS: Node = Node::new(&elements::symbols::PLUS, &|character| {
    Some(match character {
        '=' => &SYMBOL_PLUS_EQ,
        _ => return None,
    })
});

const SYMBOL_PLUS_EQ: Node = Node::new_final(&elements::symbols::PLUS_EQ);

const SYMBOL_MINUS: Node = Node::new(&elements::symbols::MINUS, &|character| {
    Some(match character {
        '=' => &SYMBOL_MINUS_EQ,
        _ => return None,
    })
});

const SYMBOL_MINUS_EQ: Node = Node::new_final(&elements::symbols::MINUS_EQ);

const SYMBOL_ASTERISK: Node = Node::new(&elements::symbols::ASTERISK, &|character| {
    Some(match character {
        '=' => &SYMBOL_ASTERISK_EQ,
        '*' => &SYMBOL_ASTERISK_D,
        _ => return None,
    })
});

const SYMBOL_ASTERISK_EQ: Node = Node::new_final(&elements::symbols::ASTERISK_EQ);

const SYMBOL_ASTERISK_D: Node = Node::new(&elements::symbols::ASTERISK_D, &|character| {
    Some(match character {
        '=' => &SYMBOL_ASTERISK_D_EQ,
        _ => return None,
    })
});

const SYMBOL_ASTERISK_D_EQ: Node = Node::new_final(&elements::symbols::ASTERISK_D_EQ);

const SYMBOL_SLASH: Node = Node::new(&elements::symbols::SLASH, &|character| {
    Some(match character {
        '=' => &SYMBOL_SLASH_EQ,
        '/' => &COMMENT_LINE,
        '*' => &COMMENT_BLOCK_1,
        _ => return None,
    })
});

const SYMBOL_SLASH_EQ: Node = Node::new_final(&elements::symbols::SLASH_EQ);

const SYMBOL_PERCENT: Node = Node::new(&elements::symbols::PERCENT, &|character| {
    Some(match character {
        '=' => &SYMBOL_PERCENT_EQ,
        _ => return None,
    })
});

const SYMBOL_PERCENT_EQ: Node = Node::new_final(&elements::symbols::PERCENT_EQ);

const SYMBOL_CARET: Node = Node::new(&elements::symbols::CARET, &|character| {
    Some(match character {
        '=' => &SYMBOL_CARET_EQ,
        _ => return None,
    })
});

const SYMBOL_CARET_EQ: Node = Node::new_final(&elements::symbols::CARET_EQ);

const SYMBOL_EXCLAMATION: Node = Node::new(&elements::symbols::EXCLAMATION, &|character| {
    Some(match character {
        '=' => &SYMBOL_EXCLAMATION_EQ,
        _ => return None,
    })
});

const SYMBOL_EXCLAMATION_EQ: Node = Node::new_final(&elements::symbols::EXCLAMATION_EQ);

const SYMBOL_EQUAL: Node = Node::new(&elements::symbols::EQUAL, &|character| {
    Some(match character {
        '=' => &SYMBOL_EQUAL_D,
        '>' => &SYMBOL_ARROW,
        _ => return None,
    })
});

const SYMBOL_EQUAL_D: Node = Node::new_final(&elements::symbols::EQUAL_D);

const SYMBOL_ARROW: Node = Node::new_final(&elements::symbols::ARROW);

const SYMBOL_PIPE: Node = Node::new(&elements::symbols::PIPE, &|character| {
    Some(match character {
        '=' => &SYMBOL_PIPE_EQ,
        '|' => &SYMBOL_PIPE_D,
        _ => return None,
    })
});

const SYMBOL_PIPE_EQ: Node = Node::new_final(&elements::symbols::PIPE_EQ);

const SYMBOL_PIPE_D: Node = Node::new(&elements::symbols::PIPE_D, &|character| {
    Some(match character {
        '=' => &SYMBOL_PIPE_D_EQ,
        _ => return None,
    })
});

const SYMBOL_PIPE_D_EQ: Node = Node::new_final(&elements::symbols::PIPE_D_EQ);

const SYMBOL_AMPERSAND: Node = Node::new(&elements::symbols::AMPERSAND, &|character| {
    Some(match character {
        '=' => &SYMBOL_AMPERSAND_EQ,
        '&' => &SYMBOL_AMPERSAND_D,
        _ => return None,
    })
});

const SYMBOL_AMPERSAND_EQ: Node = Node::new_final(&elements::symbols::AMPERSAND_EQ);

const SYMBOL_AMPERSAND_D: Node = Node::new(&elements::symbols::AMPERSAND_D, &|character| {
    Some(match character {
        '=' => &SYMBOL_AMPERSAND_D_EQ,
        _ => return None,
    })
});

const SYMBOL_AMPERSAND_D_EQ: Node = Node::new_final(&elements::symbols::AMPERSAND_D_EQ);

const SYMBOL_GUILLEMET_L: Node = Node::new(&elements::symbols::GUILLEMET_L, &|character| {
    Some(match character {
        '=' => &SYMBOL_GUILLEMET_L_EQ,
        '<' => &SYMBOL_GUILLEMET_L_D,
        _ => return None,
    })
});

const SYMBOL_GUILLEMET_L_EQ: Node = Node::new_final(&elements::symbols::GUILLEMET_L_EQ);

const SYMBOL_GUILLEMET_L_D: Node = Node::new(&elements::symbols::GUILLEMET_L_D, &|character| {
    Some(match character {
        '=' => &SYMBOL_GUILLEMET_L_D_EQ,
        '<' => &SYMBOL_GUILLEMET_L_T,
        _ => return None,
    })
});

const SYMBOL_GUILLEMET_L_D_EQ: Node = Node::new_final(&elements::symbols::GUILLEMET_L_D_EQ);

const SYMBOL_GUILLEMET_L_T: Node = Node::new(&elements::symbols::GUILLEMET_L_T, &|character| {
    Some(match character {
        '=' => &SYMBOL_GUILLEMET_L_T_EQ,
        _ => return None,
    })
});

const SYMBOL_GUILLEMET_L_T_EQ: Node = Node::new_final(&elements::symbols::GUILLEMET_L_T_EQ);

const SYMBOL_GUILLEMET_R: Node = Node::new(&elements::symbols::GUILLEMET_R, &|character| {
    Some(match character {
        '=' => &SYMBOL_GUILLEMET_R_EQ,
        '>' => &SYMBOL_GUILLEMET_R_D,
        _ => return None,
    })
});

const SYMBOL_GUILLEMET_R_EQ: Node = Node::new_final(&elements::symbols::GUILLEMET_R_EQ);

const SYMBOL_GUILLEMET_R_D: Node = Node::new(&elements::symbols::GUILLEMET_R_D, &|character| {
    Some(match character {
        '=' => &SYMBOL_GUILLEMET_R_D_EQ,
        '>' => &SYMBOL_GUILLEMET_R_T,
        _ => return None,
    })
});

const SYMBOL_GUILLEMET_R_D_EQ: Node = Node::new_final(&elements::symbols::GUILLEMET_R_D_EQ);

const SYMBOL_GUILLEMET_R_T: Node = Node::new(&elements::symbols::GUILLEMET_R_T, &|character| {
    Some(match character {
        '=' => &SYMBOL_GUILLEMET_R_T_EQ,
        _ => return None,
    })
});

const SYMBOL_GUILLEMET_R_T_EQ: Node = Node::new_final(&elements::symbols::GUILLEMET_R_T_EQ);

const SYMBOL_PARENTHESIS_L: Node = Node::new_final(&elements::symbols::PARENTHESIS_L);

const SYMBOL_PARENTHESIS_R: Node = Node::new_final(&elements::symbols::PARENTHESIS_R);

const SYMBOL_BRACE_L: Node = Node::new_final(&elements::symbols::BRACE_L);

const SYMBOL_BRACE_R: Node = Node::new_final(&elements::symbols::BRACE_R);

const SYMBOL_CROTCHET_L: Node = Node::new_final(&elements::symbols::CROTCHET_L);

const SYMBOL_CROTCHET_R: Node = Node::new_final(&elements::symbols::CROTCHET_R);

const SYMBOL_DOT: Node = Node::new(&elements::symbols::DOT, &|character| {
    Some(match character {
        '.' => &SYMBOL_DOT_D,
        _ => return None,
    })
});

const SYMBOL_DOT_D: Node = Node::new(&elements::symbols::DOT_D, &|character| {
    Some(match character {
        '=' => &SYMBOL_DOT_D_EQ,
        '.' => &SYMBOL_DOT_T,
        _ => return None,
    })
});

const SYMBOL_DOT_D_EQ: Node = Node::new_final(&elements::symbols::DOT_D_EQ);

const SYMBOL_DOT_T: Node = Node::new_final(&elements::symbols::DOT_T);

const SYMBOL_COMMA: Node = Node::new_final(&elements::symbols::COMMA);

const SYMBOL_TILDE: Node = Node::new_final(&elements::symbols::TILDE);

const SYMBOL_DOLLAR: Node = Node::new_final(&elements::symbols::DOLLAR);

const SYMBOL_INTERROGATION: Node = Node::new_final(&elements::symbols::INTERROGATION);

const SYMBOL_COLON: Node = Node::new_final(&elements::symbols::COLON);

const SYMBOL_SEMICOLON: Node = Node::new_final(&elements::symbols::SEMICOLON);

const SYMBOL_BACKSLASH: Node = Node::new_final(&elements::symbols::BACKSLASH);

const STRING_CONTENT: Node = Node::new_null(&|character| {
    Some(match character {
        '"' => &STRING,
        _ => &STRING_CONTENT,
    })
});

const STRING: Node = Node::new_final(&elements::literals::STRING);

const NUMBER_BASE: Node = Node::new(&elements::literals::INTEGER, &|character| {
    Some(match character {
        'b' => &BINARY_START,
        'o' => &OCTAL_START,
        'x' => &HEXADECIMAL_START,
        '.' => &FLOAT_START,
        '_' => &DECIMAL_SEPARATOR,
        '0' ..= '9' => &DECIMAL,
        _ => return None,
    })
});

const BINARY_START: Node = Node::new_null(&|character| {
    Some(match character {
        '0' | '1' => &BINARY,
        _ => return None,
    })
});

const BINARY: Node = Node::new(&elements::literals::INTEGER, &|character| {
    Some(match character {
        '0' | '1' => &BINARY,
        '_' => &BINARY_SEPARATOR,
        _ => return None,
    })
});

const BINARY_SEPARATOR: Node = Node::new_null(&|character| {
    Some(match character {
        '0' | '1' => &BINARY,
        '_' => &BINARY_SEPARATOR,
        _ => return None,
    })
});

const OCTAL_START: Node = Node::new_null(&|character| {
    Some(match character {
        '0' ..= '7' => &OCTAL,
        _ => return None,
    })
});

const OCTAL: Node = Node::new(&elements::literals::INTEGER, &|character| {
    Some(match character {
        '0' ..= '7' => &OCTAL,
        '_' => &OCTAL_SEPARATOR,
        _ => return None,
    })
});

const OCTAL_SEPARATOR: Node = Node::new_null(&|character| {
    Some(match character {
        '0' ..= '7' => &OCTAL,
        '_' => &OCTAL_SEPARATOR,
        _ => return None,
    })
});

const HEXADECIMAL_START: Node = Node::new_null(&|character| {
    Some(match character {
        '0' ..= '9' | 'A' ..= 'F' | 'a' ..= 'f' => &HEXADECIMAL,
        _ => return None,
    })
});

const HEXADECIMAL: Node = Node::new(&elements::literals::INTEGER, &|character| {
    Some(match character {
        '0' ..= '9' | 'A' ..= 'F' | 'a' ..= 'f' => &HEXADECIMAL,
        '_' => &HEXADECIMAL_SEPARATOR,
        _ => return None,
    })
});

const HEXADECIMAL_SEPARATOR: Node = Node::new_null(&|character| {
    Some(match character {
        '0' ..= '9' | 'A' ..= 'F' | 'a' ..= 'f' => &HEXADECIMAL,
        '_' => &HEXADECIMAL_SEPARATOR,
        _ => return None,
    })
});

const DECIMAL: Node = Node::new(&elements::literals::INTEGER, &|character| {
    Some(match character {
        '0' ..= '9' => &DECIMAL,
        '.' => &FLOAT_START,
        '_' => &DECIMAL_SEPARATOR,
        _ => return None,
    })
});

const DECIMAL_SEPARATOR: Node = Node::new_null(&|character| {
    Some(match character {
        '0' ..= '9' => &DECIMAL,
        '_' => &DECIMAL_SEPARATOR,
        _ => return None,
    })
});

const FLOAT_START: Node = Node::new_null(&|character| {
    Some(match character {
        '0' ..= '9' => &FLOAT,
        _ => return None,
    })
});

const FLOAT: Node = Node::new(&elements::literals::FLOAT, &|character| {
    Some(match character {
        '0' ..= '9' => &FLOAT,
        '_' => &FLOAT_SEPARATOR,
        _ => return None,
    })
});

const FLOAT_SEPARATOR: Node = Node::new_null(&|character| {
    Some(match character {
        '0' ..= '9' => &FLOAT,
        '_' => &FLOAT_SEPARATOR,
        _ => return None,
    })
});

const IDENTIFIER: Node = Node::new(&elements::literals::IDENTIFIER, &|character| {
    Some(match character {
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const WHITESPACE: Node = Node::new(&elements::ignores::WHITESPACE, &|character| {
    Some(match character {
        ' ' | '\t' => &WHITESPACE,
        _ => return None,
    })
});

const ENDLINE: Node = Node::new(&elements::ignores::ENDLINE, &|character| {
    Some(match character {
        '\r' | '\n' => &ENDLINE,
        _ => return None,
    })
});

const COMMENT_LINE: Node = Node::new(&elements::ignores::COMMENT_LINE, &|character| {
    Some(match character {
        '\r' | '\n' => return None,
        _ => &COMMENT_LINE,
    })
});

const COMMENT_BLOCK_1: Node = Node::new_null(&|character| {
    Some(match character {
        '*' => &COMMENT_BLOCK_2,
        _ => &COMMENT_BLOCK_1,
    })
});

const COMMENT_BLOCK_2: Node = Node::new_null(&|character| {
    Some(match character {
        '/' => &COMMENT_BLOCK_3,
        _ => &COMMENT_BLOCK_1,
    })
});

const COMMENT_BLOCK_3: Node = Node::new_final(&elements::ignores::COMMENT_BLOCK);
