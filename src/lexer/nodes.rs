use crate::lexer::node::Node;
use crate::elements;

pub const ROOT: Node = Node::new_null(&|character| {
    Some(match character {
        'a' => &KEYWORD_A,
        'b' => &KEYWORD_B,
        'c' => &KEYWORD_C,
        'd' => &KEYWORD_D,
        'e' => &KEYWORD_E,
        'f' => &KEYWORD_F,
        'i' => &KEYWORD_I,
        'l' => &KEYWORD_L,
        'p' => &KEYWORD_P,
        'r' => &KEYWORD_R,
        's' => &KEYWORD_S,
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
        '0' ..= '9' => &NUMBER,
        ' ' | '\t' => &WHITESPACE,
        '\r' | '\n' => &ENDLINE,
        _ => match character {
            'a' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
            _ => return None,
        },
    })
});

const KEYWORD_A: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        's' => &KEYWORD_AS,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_AS: Node = Node::new_final(&elements::keywords::AS);

const KEYWORD_B: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'r' => &KEYWORD_BR,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_BR: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_BRE,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_BRE: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'a' => &KEYWORD_BREA,
        'b' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_BREA: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'k' => &KEYWORD_BREAK,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_BREAK: Node = Node::new_final(&elements::keywords::BREAK);

const KEYWORD_C: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'a' => &KEYWORD_CA,
        'l' => &KEYWORD_CL,
        'o' => &KEYWORD_CO,
        'b' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CA: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        't' => &KEYWORD_CAT,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CAT: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'c' => &KEYWORD_CATC,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CATC: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'h' => &KEYWORD_CATCH,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CATCH: Node = Node::new_final(&elements::keywords::CATCH);

const KEYWORD_CL: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'a' =>  &KEYWORD_CLA,
        'b' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CLA: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        's' => &KEYWORD_CLAS,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CLAS: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        's' => &KEYWORD_CLASS,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CLASS: Node = Node::new_final(&elements::keywords::CLASS);

const KEYWORD_CO: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'n' => &KEYWORD_CON,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CON: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        't' => &KEYWORD_CONT,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CONT: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'i' => &KEYWORD_CONTI,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CONTI: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'n' => &KEYWORD_CONTIN,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CONTIN: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'u' => &KEYWORD_CONTINU,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CONTINU: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_CONTINUE,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_CONTINUE: Node = Node::new_final(&elements::keywords::CONTINUE);

const KEYWORD_D: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_DE,
        'o' => &KEYWORD_DO,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_DE: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'f' => &KEYWORD_DEF,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_DEF: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'a' => &KEYWORD_DEFA,
        'b' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_DEFA: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'u' => &KEYWORD_DEFAU,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_DEFAU: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'l' => &KEYWORD_DEFAUL,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_DEFAUL: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        't' => &KEYWORD_DEFAULT,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_DEFAULT: Node = Node::new_final(&elements::keywords::DEFAULT);

const KEYWORD_DO: Node = Node::new_final(&elements::keywords::DO);

const KEYWORD_E: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'l' => &KEYWORD_EL,
        'x' => &KEYWORD_EX,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_EL: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        's' => &KEYWORD_ELS,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_ELS: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_ELSE,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_ELSE: Node = Node::new_final(&elements::keywords::ELSE);

const KEYWORD_EX: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'p' => &KEYWORD_EXP,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_EXP: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'o' => &KEYWORD_EXPO,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_EXPO: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'r' => &KEYWORD_EXPOR,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_EXPOR: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        't' => &KEYWORD_EXPORT,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_EXPORT: Node = Node::new_final(&elements::keywords::EXPORT);

const KEYWORD_F: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'a' => &KEYWORD_FA,
        'i' => &KEYWORD_FI,
        'o' => &KEYWORD_FO,
        'r' => &KEYWORD_FR,
        'u' => &KEYWORD_FU,
        'b' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FA: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'l' => &KEYWORD_FAL,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FAL: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        's' => &KEYWORD_FALS,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FALS: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_FALSE,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FALSE: Node = Node::new_final(&elements::keywords::FALSE);

const KEYWORD_FI: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'n' => &KEYWORD_FIN,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FIN: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'a' => &KEYWORD_FINA,
        'b' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FINA: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'l' => &KEYWORD_FINAL,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FINAL: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'l' => &KEYWORD_FINALL,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FINALL: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'y' => &KEYWORD_FINALLY,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FINALLY: Node = Node::new_final(&elements::keywords::FINALLY);

const KEYWORD_FO: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'r' => &KEYWORD_FOR,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FOR: Node = Node::new_final(&elements::keywords::FOR);

const KEYWORD_FR: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'o' => &KEYWORD_FRO,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FRO: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'm' => &KEYWORD_FROM,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FROM: Node = Node::new_final(&elements::keywords::FROM);

const KEYWORD_FU: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'n' => &KEYWORD_FUN,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FUN: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'c' => &KEYWORD_FUNC,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FUNC: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        't' => &KEYWORD_FUNCT,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FUNCT: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'i' => &KEYWORD_FUNCTI,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FUNCTI: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'o' => &KEYWORD_FUNCTIO,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FUNCTIO: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'n' => &KEYWORD_FUNCTION,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_FUNCTION: Node = Node::new_final(&elements::keywords::FUNCTION);

const KEYWORD_I: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'f' => &KEYWORD_IF,
        'm' => &KEYWORD_IM,
        'n' => &KEYWORD_IN,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_IF: Node = Node::new_final(&elements::keywords::IF);

const KEYWORD_IM: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'p' => &KEYWORD_IMP,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_IMP: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'o' => &KEYWORD_IMPO,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_IMPO: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'r' => &KEYWORD_IMPOR,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_IMPOR: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        't' => &KEYWORD_IMPORT,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_IMPORT: Node = Node::new_final(&elements::keywords::IMPORT);

const KEYWORD_IN: Node = Node::new_final(&elements::keywords::IN);

const KEYWORD_L: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_LE,
        'o' => &KEYWORD_LO,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_LE: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        't' => &KEYWORD_LET,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_LET: Node = Node::new_final(&elements::keywords::LET);

const KEYWORD_LO: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'o' => &KEYWORD_LOO,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_LOO: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'p' => &KEYWORD_LOOP,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_LOOP: Node = Node::new_final(&elements::keywords::LOOP);

const KEYWORD_P: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'r' => &KEYWORD_PR,
        'u' => &KEYWORD_PU,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_PR: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'i' => &KEYWORD_PRI,
        'o' => &KEYWORD_PRO,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_PRI: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'v' => &KEYWORD_PRIV,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_PRIV: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'a' => &KEYWORD_PRIVA,
        'b' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_PRIVA: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        't' => &KEYWORD_PRIVAT,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_PRIVAT: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_PRIVATE,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_PRIVATE: Node = Node::new_final(&elements::keywords::PRIVATE);

const KEYWORD_PRO: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        't' => &KEYWORD_PROT,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_PROT: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_PROTE,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_PROTE: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'c' => &KEYWORD_PROTEC,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_PROTEC: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        't' => &KEYWORD_PROTECT,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_PROTECT: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_PROTECTE,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_PROTECTE: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'd' => &KEYWORD_PROTECTED,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_PROTECTED: Node = Node::new_final(&elements::keywords::PROTECTED);

const KEYWORD_PU: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'b' => &KEYWORD_PUB,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_PUB: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'l' => &KEYWORD_PUBL,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_PUBL: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'i' => &KEYWORD_PUBLI,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_PUBLI: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'c' => &KEYWORD_PUBLIC,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_PUBLIC: Node = Node::new_final(&elements::keywords::PUBLIC);

const KEYWORD_R: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_RE,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_RE: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        't' => &KEYWORD_RET,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_RET: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'u' => &KEYWORD_RETU,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_RETU: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'r' => &KEYWORD_RETUR,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_RETUR: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'n' => &KEYWORD_RETURN,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_RETURN: Node = Node::new_final(&elements::keywords::RETURN);

const KEYWORD_S: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        't' => &KEYWORD_ST,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_ST: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'a' => &KEYWORD_STA,
        'b' ..= 'z' | 'A' ..= 'Z' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_STA: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        't' => &KEYWORD_STAT,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_STAT: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'i' => &KEYWORD_STATI,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_STATI: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'c' => &KEYWORD_STATIC,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_STATIC: Node = Node::new_final(&elements::keywords::STATIC);

const KEYWORD_T: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'h' => &KEYWORD_TH,
        'r' => &KEYWORD_TR,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_TH: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_THE,
        'r' => &KEYWORD_THR,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_THE: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'n' => &KEYWORD_THEN,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_THEN: Node = Node::new_final(&elements::keywords::THEN);

const KEYWORD_THR: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'o' => &KEYWORD_THRO,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_THRO: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'w' => &KEYWORD_THROW,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_THROW: Node = Node::new_final(&elements::keywords::THROW);

const KEYWORD_TR: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'u' => &KEYWORD_TRU,
        'y' => &KEYWORD_TRY,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_TRU: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_TRUE,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_TRUE: Node = Node::new_final(&elements::keywords::TRUE);

const KEYWORD_TRY: Node = Node::new_final(&elements::keywords::TRY);

const KEYWORD_W: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'h' => &KEYWORD_WH,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_WH: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'i' => &KEYWORD_WHI,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_WHI: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'l' => &KEYWORD_WHIL,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_WHIL: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    Some(match character {
        'e' => &KEYWORD_WHILE,
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => &IDENTIFIER,
        _ => return None,
    })
});

const KEYWORD_WHILE: Node = Node::new_final(&elements::keywords::WHILE);

const SYMBOL_PLUS: Node = Node::new(&elements::symbols::PLUS, &|character| {
    if character == '=' {
        Some(&SYMBOL_PLUS_EQ)
    } else {
        None
    }
});

const SYMBOL_PLUS_EQ: Node = Node::new_final(&elements::symbols::PLUS_EQ);

const SYMBOL_MINUS: Node = Node::new(&elements::symbols::MINUS, &|character| {
    if character == '=' {
        Some(&SYMBOL_MINUS_EQ)
    } else {
        None
    }
});

const SYMBOL_MINUS_EQ: Node = Node::new_final(&elements::symbols::MINUS_EQ);

const SYMBOL_ASTERISK: Node = Node::new(&elements::symbols::ASTERISK, &|character| {
    match character {
        '=' => Some(&SYMBOL_ASTERISK_EQ),
        '*' => Some(&SYMBOL_ASTERISK_D),
        _ => None,
    }
});

const SYMBOL_ASTERISK_EQ: Node = Node::new_final(&elements::symbols::ASTERISK_EQ);

const SYMBOL_ASTERISK_D: Node = Node::new(&elements::symbols::ASTERISK_D, &|character| {
    if character == '=' {
        Some(&SYMBOL_ASTERISK_D_EQ)
    } else {
        None
    }
});

const SYMBOL_ASTERISK_D_EQ: Node = Node::new_final(&elements::symbols::ASTERISK_D_EQ);

const SYMBOL_SLASH: Node = Node::new(&elements::symbols::SLASH, &|character| {
    if character == '=' {
        Some(&SYMBOL_SLASH_EQ)
    } else {
        None
    }
});

const SYMBOL_SLASH_EQ: Node = Node::new_final(&elements::symbols::SLASH_EQ);

const SYMBOL_PERCENT: Node = Node::new(&elements::symbols::PERCENT, &|character| {
    if character == '=' {
        Some(&SYMBOL_PERCENT_EQ)
    } else {
        None
    }
});

const SYMBOL_PERCENT_EQ: Node = Node::new_final(&elements::symbols::PERCENT_EQ);

const SYMBOL_CARET: Node = Node::new(&elements::symbols::CARET, &|character| {
    if character == '=' {
        Some(&SYMBOL_CARET_EQ)
    } else {
        None
    }
});

const SYMBOL_CARET_EQ: Node = Node::new_final(&elements::symbols::CARET_EQ);

const SYMBOL_EXCLAMATION: Node = Node::new(&elements::symbols::EXCLAMATION, &|character| {
    if character == '=' {
        Some(&SYMBOL_EXCLAMATION_EQ)
    } else {
        None
    }
});

const SYMBOL_EXCLAMATION_EQ: Node = Node::new_final(&elements::symbols::EXCLAMATION_EQ);

const SYMBOL_EQUAL: Node = Node::new(&elements::symbols::EQUAL, &|character| {
    match character {
        '=' => Some(&SYMBOL_EQUAL_D),
        '>' => Some(&SYMBOL_ARROW),
        _ => None,
    }
});

const SYMBOL_EQUAL_D: Node = Node::new_final(&elements::symbols::EQUAL_D);

const SYMBOL_ARROW: Node = Node::new_final(&elements::symbols::ARROW);

const SYMBOL_PIPE: Node = Node::new(&elements::symbols::PIPE, &|character| {
    match character {
        '=' => Some(&SYMBOL_PIPE_EQ),
        '|' => Some(&SYMBOL_PIPE_D),
        _ => None,
    }
});

const SYMBOL_PIPE_EQ: Node = Node::new_final(&elements::symbols::PIPE_EQ);

const SYMBOL_PIPE_D: Node = Node::new(&elements::symbols::PIPE_D, &|character| {
    if character == '=' {
        Some(&SYMBOL_PIPE_D_EQ)
    } else {
        None
    }
});

const SYMBOL_PIPE_D_EQ: Node = Node::new_final(&elements::symbols::PIPE_D_EQ);

const SYMBOL_AMPERSAND: Node = Node::new(&elements::symbols::AMPERSAND, &|character| {
    match character {
        '=' => Some(&SYMBOL_AMPERSAND_EQ),
        '&' => Some(&SYMBOL_AMPERSAND_D),
        _ => None,
    }
});

const SYMBOL_AMPERSAND_EQ: Node = Node::new_final(&elements::symbols::AMPERSAND_EQ);

const SYMBOL_AMPERSAND_D: Node = Node::new(&elements::symbols::AMPERSAND_D, &|character| {
    if character == '=' {
        Some(&SYMBOL_AMPERSAND_D_EQ)
    } else {
        None
    }
});

const SYMBOL_AMPERSAND_D_EQ: Node = Node::new_final(&elements::symbols::AMPERSAND_D_EQ);

const SYMBOL_GUILLEMET_L: Node = Node::new(&elements::symbols::GUILLEMET_L, &|character| {
    match character {
        '=' => Some(&SYMBOL_GUILLEMET_L_EQ),
        '<' => Some(&SYMBOL_GUILLEMET_L_D),
        _ => None,
    }
});

const SYMBOL_GUILLEMET_L_EQ: Node = Node::new_final(&elements::symbols::GUILLEMET_L_EQ);

const SYMBOL_GUILLEMET_L_D: Node = Node::new(&elements::symbols::GUILLEMET_L_D, &|character| {
    match character {
        '=' => Some(&SYMBOL_GUILLEMET_L_D_EQ),
        '<' => Some(&SYMBOL_GUILLEMET_L_T),
        _ => None,
    }
});

const SYMBOL_GUILLEMET_L_D_EQ: Node = Node::new_final(&elements::symbols::GUILLEMET_L_D_EQ);

const SYMBOL_GUILLEMET_L_T: Node = Node::new(&elements::symbols::GUILLEMET_L_T, &|character| {
    if character == '=' {
        Some(&SYMBOL_GUILLEMET_L_T_EQ)
    } else {
        None
    }
});

const SYMBOL_GUILLEMET_L_T_EQ: Node = Node::new_final(&elements::symbols::GUILLEMET_L_T_EQ);

const SYMBOL_GUILLEMET_R: Node = Node::new(&elements::symbols::GUILLEMET_R, &|character| {
    match character {
        '=' => Some(&SYMBOL_GUILLEMET_R_EQ),
        '>' => Some(&SYMBOL_GUILLEMET_R_D),
        _ => None,
    }
});

const SYMBOL_GUILLEMET_R_EQ: Node = Node::new_final(&elements::symbols::GUILLEMET_R_EQ);

const SYMBOL_GUILLEMET_R_D: Node = Node::new(&elements::symbols::GUILLEMET_R_D, &|character| {
    match character {
        '=' => Some(&SYMBOL_GUILLEMET_R_D_EQ),
        '>' => Some(&SYMBOL_GUILLEMET_R_T),
        _ => None,
    }
});

const SYMBOL_GUILLEMET_R_D_EQ: Node = Node::new_final(&elements::symbols::GUILLEMET_R_D_EQ);

const SYMBOL_GUILLEMET_R_T: Node = Node::new(&elements::symbols::GUILLEMET_R_T, &|character| {
    if character == '=' {
        Some(&SYMBOL_GUILLEMET_R_T_EQ)
    } else {
        None
    }
});

const SYMBOL_GUILLEMET_R_T_EQ: Node = Node::new_final(&elements::symbols::GUILLEMET_R_T_EQ);

const SYMBOL_PARENTHESIS_L: Node = Node::new_final(&elements::symbols::PARENTHESIS_L);

const SYMBOL_PARENTHESIS_R: Node = Node::new_final(&elements::symbols::PARENTHESIS_R);

const SYMBOL_BRACE_L: Node = Node::new_final(&elements::symbols::BRACE_L);

const SYMBOL_BRACE_R: Node = Node::new_final(&elements::symbols::BRACE_R);

const SYMBOL_CROTCHET_L: Node = Node::new_final(&elements::symbols::CROTCHET_L);

const SYMBOL_CROTCHET_R: Node = Node::new_final(&elements::symbols::CROTCHET_R);

const SYMBOL_DOT: Node = Node::new(&elements::symbols::DOT, &|character| {
    if character == '.' {
        Some(&SYMBOL_DOT_D)
    } else {
        None
    }
});

const SYMBOL_DOT_D: Node = Node::new(&elements::symbols::DOT_D, &|character| {
    match character {
        '=' => Some(&SYMBOL_DOT_D_EQ),
        '.' => Some(&SYMBOL_DOT_T),
        _ => None,
    }
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
    Some(if character == '"' {
        &STRING
    } else {
        &STRING_CONTENT
    })
});

const STRING: Node = Node::new_final(&elements::variables::STRING);

const NUMBER: Node = Node::new(&elements::variables::NUMBER, &|character| {
    match character {
        '0' ..= '9' => Some(&NUMBER),
        _ => None,
    }
});

const IDENTIFIER: Node = Node::new(&elements::variables::IDENTIFIER, &|character| {
    match character {
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => Some(&IDENTIFIER),
        _ => None,
    }
});

const WHITESPACE: Node = Node::new(&elements::ignores::WHITESPACE, &|character| {
    match character {
        ' ' | '\t' => Some(&WHITESPACE),
        _ => None,
    }
});

const ENDLINE: Node = Node::new(&elements::ignores::ENDLINE, &|character| {
    match character {
        '\r' | '\n' => Some(&ENDLINE),
        _ => None,
    }
});
