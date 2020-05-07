use crate::element::Element;
use crate::token::Token;
use crate::tree::{ Child, Tree };

pub fn tokens(tokens: &Vec<Token>) {
    for token in tokens {
        println!("{} {:?}", token.element.name, token.string);
    }
}

pub fn tree(tree: &Tree) {
	node(tree, String::from(""), String::from(""));
}

fn node(tree: &Tree, prefix: String, infix: String) {
    element(&prefix, tree.element);
    for i in 0..tree.children.len() {
        let (next_prefix, next_suffix) = if i == tree.children.len() - 1 {
            (format!("{}{}", infix, "└─"), format!("{}{}", infix, "  "))
        } else {
            (format!("{}{}", infix, "├─"), format!("{}{}", infix, "│ "))
        };

        match &tree.children[i] {
            Child::Token(token) => element(&next_prefix, token.element),
            Child::Tree(tree) => node(tree, next_prefix, next_suffix),
        };
    }
}

fn element(prefix: &String, element: &Element) {
    println!("{}{}", prefix, element.name);
}
