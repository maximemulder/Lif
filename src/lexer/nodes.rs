use crate::lexer::node::Node;
use crate::elements;

pub const ROOT: Node = Node::new_null(&|character| {
	return Some(match character {
		'a' => &KEYWORD_A,
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
			'a' ..= 'z' | 'A' ..= 'Z' | '_'  => &IDENTIFIER,
			_ => return None,
		},
	});
});

const KEYWORD_A: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 's' {
		&KEYWORD_AS
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_AS: Node = Node::new_final(&elements::KEYWORD_AS);

const KEYWORD_C: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(match character {
		'a' => &KEYWORD_CA,
		'l' => &KEYWORD_CL,
		_ => &IDENTIFIER,
	});
});

const KEYWORD_CA: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 't' {
		&KEYWORD_CAT
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_CAT: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'c' {
		&KEYWORD_CATC
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_CATC: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'h' {
		&KEYWORD_CATCH
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_CATCH: Node = Node::new_final(&elements::KEYWORD_CATCH);

const KEYWORD_CL: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'a' {
		&KEYWORD_CLA
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_CLA: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 's' {
		&KEYWORD_CLAS
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_CLAS: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 's' {
		&KEYWORD_CLASS
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_CLASS: Node = Node::new_final(&elements::KEYWORD_CLASS);

const KEYWORD_D: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(match character {
		'e' => &KEYWORD_DE,
		'o' => &KEYWORD_DO,
		_ => &IDENTIFIER
	});
});

const KEYWORD_DE: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'f' {
		&KEYWORD_DEF
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_DEF: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'a' {
		&KEYWORD_DEFA
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_DEFA: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'u' {
		&KEYWORD_DEFAU
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_DEFAU: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'l' {
		&KEYWORD_DEFAUL
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_DEFAUL: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 't' {
		&KEYWORD_DEFAULT
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_DEFAULT: Node = Node::new_final(&elements::KEYWORD_DEFAULT);

const KEYWORD_DO: Node = Node::new_final(&elements::KEYWORD_DO);

const KEYWORD_E: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(match character {
		'l' => &KEYWORD_EL,
		'x' => &KEYWORD_EX,
		_ => &IDENTIFIER,
	});
});

const KEYWORD_EL: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 's' {
		&KEYWORD_ELS
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_ELS: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'e' {
		&KEYWORD_ELSE
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_ELSE: Node = Node::new_final(&elements::KEYWORD_ELSE);

const KEYWORD_EX: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'p' {
		&KEYWORD_EXP
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_EXP: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'o' {
		&KEYWORD_EXPO
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_EXPO: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'r' {
		&KEYWORD_EXPOR
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_EXPOR: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 't' {
		&KEYWORD_EXPORT
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_EXPORT: Node = Node::new_final(&elements::KEYWORD_EXPORT);

const KEYWORD_F: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(match character {
		'i' => &KEYWORD_FI,
		'o' => &KEYWORD_FO,
		'r' => &KEYWORD_FR,
		'u' => &KEYWORD_FU,
		_ => &IDENTIFIER,
	});
});

const KEYWORD_FI: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'n' {
		&KEYWORD_FIN
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_FIN: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'a' {
		&KEYWORD_FINA
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_FINA: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'l' {
		&KEYWORD_FINAL
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_FINAL: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'l' {
		&KEYWORD_FINALL
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_FINALL: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'y' {
		&KEYWORD_FINALLY
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_FINALLY: Node = Node::new_final(&elements::KEYWORD_FINALLY);

const KEYWORD_FO: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'r' {
		&KEYWORD_FOR
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_FOR: Node = Node::new_final(&elements::KEYWORD_FOR);

const KEYWORD_FR: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'o' {
		&KEYWORD_FRO
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_FRO: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'm' {
		&KEYWORD_FROM
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_FROM: Node = Node::new_final(&elements::KEYWORD_FROM);

const KEYWORD_FU: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'n' {
		&KEYWORD_FUN
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_FUN: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'c' {
		&KEYWORD_FUNC
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_FUNC: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 't' {
		&KEYWORD_FUNCT
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_FUNCT: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'i' {
		&KEYWORD_FUNCTI
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_FUNCTI: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'o' {
		&KEYWORD_FUNCTIO
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_FUNCTIO: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'n' {
		&KEYWORD_FUNCTION
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_FUNCTION: Node = Node::new_final(&elements::KEYWORD_FUNCTION);

const KEYWORD_I: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(match character {
		'f' => &KEYWORD_IF,
		'm' => &KEYWORD_IM,
		'n' => &KEYWORD_IN,
		_ => &IDENTIFIER,
	});
});

const KEYWORD_IF: Node = Node::new_final(&elements::KEYWORD_IF);

const KEYWORD_IM: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'p' {
		&KEYWORD_IMP
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_IMP: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'o' {
		&KEYWORD_IMPO
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_IMPO: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'r' {
		&KEYWORD_IMPOR
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_IMPOR: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 't' {
		&KEYWORD_IMPORT
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_IMPORT: Node = Node::new_final(&elements::KEYWORD_IMPORT);

const KEYWORD_IN: Node = Node::new_final(&elements::KEYWORD_IN);

const KEYWORD_L: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(match character {
		'e' => &KEYWORD_LE,
		'o' => &KEYWORD_LO,
		_ => &IDENTIFIER,
	});
});

const KEYWORD_LE: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 't' {
		&KEYWORD_LET
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_LET: Node = Node::new_final(&elements::KEYWORD_LET);

const KEYWORD_LO: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'o' {
		&KEYWORD_LOO
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_LOO: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'p' {
		&KEYWORD_LOOP
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_LOOP: Node = Node::new_final(&elements::KEYWORD_LOOP);

const KEYWORD_P: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(match character {
		'r' => &KEYWORD_PR,
		'u' => &KEYWORD_PU,
		_ => &IDENTIFIER,
	});
});

const KEYWORD_PR: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(match character {
		'i' => &KEYWORD_PRI,
		'o' => &KEYWORD_PRO,
		_ => &IDENTIFIER,
	});
});

const KEYWORD_PRI: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'v' {
		&KEYWORD_PRIV
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_PRIV: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'a' {
		&KEYWORD_PRIVA
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_PRIVA: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 't' {
		&KEYWORD_PRIVAT
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_PRIVAT: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'e' {
		&KEYWORD_PRIVATE
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_PRIVATE: Node = Node::new_final(&elements::KEYWORD_PRIVATE);

const KEYWORD_PRO: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 't' {
		&KEYWORD_PROT
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_PROT: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'e' {
		&KEYWORD_PROTE
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_PROTE: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'c' {
		&KEYWORD_PROTEC
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_PROTEC: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 't' {
		&KEYWORD_PROTECT
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_PROTECT: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'e' {
		&KEYWORD_PROTECTE
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_PROTECTE: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'd' {
		&KEYWORD_PROTECTED
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_PROTECTED: Node = Node::new_final(&elements::KEYWORD_PROTECTED);

const KEYWORD_PU: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'b' {
		&KEYWORD_PUB
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_PUB: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'l' {
		&KEYWORD_PUBL
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_PUBL: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'i' {
		&KEYWORD_PUBLI
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_PUBLI: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'c' {
		&KEYWORD_PUBLIC
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_PUBLIC: Node = Node::new_final(&elements::KEYWORD_PUBLIC);

const KEYWORD_R: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'e' {
		&KEYWORD_RE
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_RE: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 't' {
		&KEYWORD_RET
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_RET: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'u' {
		&KEYWORD_RETU
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_RETU: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'r' {
		&KEYWORD_RETUR
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_RETUR: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'n' {
		&KEYWORD_RETURN
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_RETURN: Node = Node::new_final(&elements::KEYWORD_RETURN);

const KEYWORD_S: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 't' {
		&KEYWORD_ST
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_ST: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'a' {
		&KEYWORD_STA
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_STA: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 't' {
		&KEYWORD_STAT
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_STAT: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'i' {
		&KEYWORD_STATI
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_STATI: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'c' {
		&KEYWORD_STATIC
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_STATIC: Node = Node::new_final(&elements::KEYWORD_STATIC);

const KEYWORD_T: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(match character {
		'h' => &KEYWORD_TH,
		'r' => &KEYWORD_TR,
		_ => &IDENTIFIER,
	});
});

const KEYWORD_TH: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(match character {
		'e' => &KEYWORD_THE,
		'r' => &KEYWORD_THR,
		_ => &IDENTIFIER,
	});
});

const KEYWORD_THE: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'n' {
		&KEYWORD_THEN
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_THEN: Node = Node::new_final(&elements::KEYWORD_THEN);

const KEYWORD_THR: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'o' {
		&KEYWORD_THRO
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_THRO: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'w' {
		&KEYWORD_THROW
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_THROW: Node = Node::new_final(&elements::KEYWORD_THROW);

const KEYWORD_TR: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'y' {
		&KEYWORD_TRY
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_TRY: Node = Node::new_final(&elements::KEYWORD_TRY);

const KEYWORD_W: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'h' {
		&KEYWORD_WH
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_WH: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'i' {
		&KEYWORD_WHI
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_WHI: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'l' {
		&KEYWORD_WHIL
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_WHIL: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return Some(if character == 'e' {
		&KEYWORD_WHILE
	} else {
		&IDENTIFIER
	});
});

const KEYWORD_WHILE: Node = Node::new_final(&elements::KEYWORD_WHILE);

const SYMBOL_PLUS: Node = Node::new(&elements::SYMBOL_PLUS, &|character| {
	return if character == '=' {
		Some(&SYMBOL_PLUS_EQ)
	} else {
		None
	};
});

const SYMBOL_PLUS_EQ: Node = Node::new_final(&elements::SYMBOL_PLUS_EQ);

const SYMBOL_MINUS: Node = Node::new(&elements::SYMBOL_MINUS, &|character| {
	return if character == '=' {
		Some(&SYMBOL_MINUS_EQ)
	} else {
		None
	};
});

const SYMBOL_MINUS_EQ: Node = Node::new_final(&elements::SYMBOL_MINUS_EQ);

const SYMBOL_ASTERISK: Node = Node::new(&elements::SYMBOL_ASTERISK, &|character| {
	return match character {
		'=' => Some(&SYMBOL_ASTERISK_EQ),
		'*' => Some(&SYMBOL_ASTERISK_D),
		_ => None,
	};
});

const SYMBOL_ASTERISK_EQ: Node = Node::new_final(&elements::SYMBOL_ASTERISK_EQ);

const SYMBOL_ASTERISK_D: Node = Node::new(&elements::SYMBOL_ASTERISK_D, &|character| {
	return if character == '=' {
		Some(&SYMBOL_ASTERISK_D_EQ)
	} else {
		None
	};
});

const SYMBOL_ASTERISK_D_EQ: Node = Node::new_final(&elements::SYMBOL_ASTERISK_D_EQ);

const SYMBOL_SLASH: Node = Node::new(&elements::SYMBOL_SLASH, &|character| {
	return if character == '=' {
		Some(&SYMBOL_SLASH_EQ)
	} else {
		None
	};
});

const SYMBOL_SLASH_EQ: Node = Node::new_final(&elements::SYMBOL_SLASH_EQ);

const SYMBOL_PERCENT: Node = Node::new(&elements::SYMBOL_PERCENT, &|character| {
	return if character == '=' {
		Some(&SYMBOL_PERCENT_EQ)
	} else {
		None
	};
});

const SYMBOL_PERCENT_EQ: Node = Node::new_final(&elements::SYMBOL_PERCENT_EQ);

const SYMBOL_CARET: Node = Node::new(&elements::SYMBOL_CARET, &|character| {
	return if character == '=' {
		Some(&SYMBOL_CARET_EQ)
	} else {
		None
	};
});

const SYMBOL_CARET_EQ: Node = Node::new_final(&elements::SYMBOL_CARET_EQ);

const SYMBOL_EXCLAMATION: Node = Node::new(&elements::SYMBOL_EXCLAMATION, &|character| {
	return if character == '=' {
		Some(&SYMBOL_EXCLAMATION_EQ)
	} else {
		None
	};
});

const SYMBOL_EXCLAMATION_EQ: Node = Node::new_final(&elements::SYMBOL_EXCLAMATION_EQ);

const SYMBOL_EQUAL: Node = Node::new(&elements::SYMBOL_EQUAL, &|character| {
	return match character {
		'=' => Some(&SYMBOL_EQUAL_D),
		'>' => Some(&SYMBOL_ARROW),
		_ => None,
	};
});

const SYMBOL_EQUAL_D: Node = Node::new_final(&elements::SYMBOL_EQUAL_D);

const SYMBOL_ARROW: Node = Node::new_final(&elements::SYMBOL_ARROW);

const SYMBOL_PIPE: Node = Node::new(&elements::SYMBOL_PIPE, &|character| {
	return match character {
		'=' => Some(&SYMBOL_PIPE_EQ),
		'|' => Some(&SYMBOL_PIPE_D),
		_ => None,
	};
});

const SYMBOL_PIPE_EQ: Node = Node::new_final(&elements::SYMBOL_PIPE_EQ);

const SYMBOL_PIPE_D: Node = Node::new(&elements::SYMBOL_PIPE_D, &|character| {
	return if character == '=' {
		Some(&SYMBOL_PIPE_D_EQ)
	} else {
		None
	};
});

const SYMBOL_PIPE_D_EQ: Node = Node::new_final(&elements::SYMBOL_PIPE_D_EQ);

const SYMBOL_AMPERSAND: Node = Node::new(&elements::SYMBOL_AMPERSAND, &|character| {
	return match character {
		'=' => Some(&SYMBOL_AMPERSAND_EQ),
		'&' => Some(&SYMBOL_AMPERSAND_D),
		_ => None,
	};
});

const SYMBOL_AMPERSAND_EQ: Node = Node::new_final(&elements::SYMBOL_AMPERSAND_EQ);

const SYMBOL_AMPERSAND_D: Node = Node::new(&elements::SYMBOL_AMPERSAND_D, &|character| {
	return if character == '=' {
		Some(&SYMBOL_AMPERSAND_D_EQ)
	} else {
		None
	};
});

const SYMBOL_AMPERSAND_D_EQ: Node = Node::new_final(&elements::SYMBOL_AMPERSAND_D_EQ);

const SYMBOL_GUILLEMET_L: Node = Node::new(&elements::SYMBOL_GUILLEMET_L, &|character| {
	return match character {
		'=' => Some(&SYMBOL_GUILLEMET_L_EQ),
		'<' => Some(&SYMBOL_GUILLEMET_L_D),
		_ => None,
	};
});

const SYMBOL_GUILLEMET_L_EQ: Node = Node::new_final(&elements::SYMBOL_GUILLEMET_L_EQ);

const SYMBOL_GUILLEMET_L_D: Node = Node::new(&elements::SYMBOL_GUILLEMET_L_D, &|character| {
	return match character {
		'=' => Some(&SYMBOL_GUILLEMET_L_D_EQ),
		'<' => Some(&SYMBOL_GUILLEMET_L_T),
		_ => None,
	};
});

const SYMBOL_GUILLEMET_L_D_EQ: Node = Node::new_final(&elements::SYMBOL_GUILLEMET_L_D_EQ);

const SYMBOL_GUILLEMET_L_T: Node = Node::new(&elements::SYMBOL_GUILLEMET_L_T, &|character| {
	return if character == '=' {
		Some(&SYMBOL_GUILLEMET_L_T_EQ)
	} else {
		None
	};
});

const SYMBOL_GUILLEMET_L_T_EQ: Node = Node::new_final(&elements::SYMBOL_GUILLEMET_L_T_EQ);

const SYMBOL_GUILLEMET_R: Node = Node::new(&elements::SYMBOL_GUILLEMET_R, &|character| {
	return match character {
		'=' => Some(&SYMBOL_GUILLEMET_R_EQ),
		'>' => Some(&SYMBOL_GUILLEMET_R_D),
		_ => None,
	};
});

const SYMBOL_GUILLEMET_R_EQ: Node = Node::new_final(&elements::SYMBOL_GUILLEMET_R_EQ);

const SYMBOL_GUILLEMET_R_D: Node = Node::new(&elements::SYMBOL_GUILLEMET_R_D, &|character| {
	return match character {
		'=' => Some(&SYMBOL_GUILLEMET_R_D_EQ),
		'>' => Some(&SYMBOL_GUILLEMET_R_T),
		_ => None,
	};
});

const SYMBOL_GUILLEMET_R_D_EQ: Node = Node::new_final(&elements::SYMBOL_GUILLEMET_R_D_EQ);

const SYMBOL_GUILLEMET_R_T: Node = Node::new(&elements::SYMBOL_GUILLEMET_R_T, &|character| {
	return if character == '=' {
		Some(&SYMBOL_GUILLEMET_R_T_EQ)
	} else {
		None
	};
});

const SYMBOL_GUILLEMET_R_T_EQ: Node = Node::new_final(&elements::SYMBOL_GUILLEMET_R_T_EQ);

const SYMBOL_PARENTHESIS_L: Node = Node::new_final(&elements::SYMBOL_PARENTHESIS_L);

const SYMBOL_PARENTHESIS_R: Node = Node::new_final(&elements::SYMBOL_PARENTHESIS_R);

const SYMBOL_BRACE_L: Node = Node::new_final(&elements::SYMBOL_BRACE_L);

const SYMBOL_BRACE_R: Node = Node::new_final(&elements::SYMBOL_BRACE_R);

const SYMBOL_CROTCHET_L: Node = Node::new_final(&elements::SYMBOL_CROTCHET_L);

const SYMBOL_CROTCHET_R: Node = Node::new_final(&elements::SYMBOL_CROTCHET_R);

const SYMBOL_DOT: Node = Node::new(&elements::SYMBOL_DOT, &|character| {
	return if character == '.' {
		Some(&SYMBOL_DOT_D)
	} else {
		None
	};
});

const SYMBOL_DOT_D: Node = Node::new(&elements::SYMBOL_DOT_D, &|character| {

	return match character {
		'=' => Some(&SYMBOL_DOT_D_EQ),
		'.' => Some(&SYMBOL_DOT_T),
		_ => None,
	};
});

const SYMBOL_DOT_D_EQ: Node = Node::new_final(&elements::SYMBOL_DOT_D_EQ);

const SYMBOL_DOT_T: Node = Node::new_final(&elements::SYMBOL_DOT_T);

const SYMBOL_COMMA: Node = Node::new_final(&elements::SYMBOL_COMMA);

const SYMBOL_TILDE: Node = Node::new_final(&elements::SYMBOL_TILDE);

const SYMBOL_DOLLAR: Node = Node::new_final(&elements::SYMBOL_DOLLAR);

const SYMBOL_INTERROGATION: Node = Node::new_final(&elements::SYMBOL_INTERROGATION);

const SYMBOL_COLON: Node = Node::new_final(&elements::SYMBOL_COLON);

const SYMBOL_SEMICOLON: Node = Node::new_final(&elements::SYMBOL_SEMICOLON);

const SYMBOL_BACKSLASH: Node = Node::new_final(&elements::SYMBOL_BACKSLASH);

const STRING_CONTENT: Node = Node::new_null(&|character| {
	return Some(if character == '"' {
		&STRING
	} else {
		&STRING_CONTENT
	});
});

const STRING: Node = Node::new_final(&elements::STRING);

const NUMBER: Node = Node::new(&elements::NUMBER, &|character| {
	return match character {
		'0' ..= '9' => Some(&NUMBER),
		_ => None,
	};
});

const IDENTIFIER: Node = Node::new(&elements::IDENTIFIER, &|character| {
	return match character {
		'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => Some(&IDENTIFIER),
		_ => None,
	};
});

const WHITESPACE: Node = Node::new(&elements::WHITESPACE, &|character| {
	return match character {
		' ' | '\t' => Some(&WHITESPACE),
		_ => None,
	};
});

const ENDLINE: Node = Node::new(&elements::ENDLINE, &|character| {
	return match character {
		'\r' | '\n' => Some(&ENDLINE),
		_ => None,
	};
});
