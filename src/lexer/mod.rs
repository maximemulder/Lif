mod node;
mod nodes;

use crate::code::Code;
use crate::elements::ignores::{ WHITESPACE, ENDLINE, COMMENT_LINE, COMMENT_BLOCK };
use crate::element::Element;
use crate::node::Node;

use nodes::ROOT;

pub fn lex<'a>(code: &'a Code) -> Vec<Node<'a>> {
    let mut tokens = Vec::new();
    let mut shift = 0;
    while let Some((element, length)) = automaton(&code.text[shift ..]) {
        if element != &WHITESPACE && element != &ENDLINE && element != &COMMENT_LINE && element != &COMMENT_BLOCK {
            tokens.push(Node::new_token(code, element, (shift, shift + length)));
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
