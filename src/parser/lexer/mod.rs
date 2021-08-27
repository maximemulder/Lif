mod node;
mod nodes;

use crate::memory::Ref;
use crate::parser::{ Code, CNode, Element };
use crate::parser::elements::ignores::{ WHITESPACE, ENDLINE, COMMENT_LINE, COMMENT_BLOCK };

use nodes::ROOT;

pub fn lex(code: Ref<Code>) -> Vec<CNode> {
    let mut tokens = Vec::new();
    let mut shift = 0;
    while let Some((element, length)) = automaton(&code.text[shift ..]) {
        if element != &WHITESPACE && element != &ENDLINE && element != &COMMENT_LINE && element != &COMMENT_BLOCK {
            tokens.push(CNode::new_token(code, element, shift, shift + length));
        }

        shift += length;
    }

    tokens
}

fn automaton(string: &str) -> Option<(&'static Element, usize)> {
    let mut node = &ROOT;
    let mut counter = 0;
    for character in string.chars() {
        let next = (node.execute)(character);
        if next.is_none() {
            break;
        }

        node = next.unwrap();
        counter += 1;
    }

    Some((node.element?, counter))
}
