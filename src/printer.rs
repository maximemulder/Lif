use crate::element::Element;
use crate::token::Token;
use crate::tree::{ Child, Tree };

pub fn tokens(tokens: &Vec<Token>) {
    for token in tokens {
        println!("{} {:?}", token.element.name, token.string);
    }
}

pub fn tree(tree: &Tree) {
	node(0, tree);
}

fn node(mut shift: usize, tree: &Tree) {
	element(shift, tree.element);
	shift += 1;
	for child in tree.children.iter() {
        match child {
            Child::Token(token) => element(shift, token.element),
            Child::Tree(tree) => node(shift, tree),
		};
    }
}

fn element(shift: usize, element: &Element) {
    println!("{}{}", " ".repeat(shift * 4), element.name);
}
