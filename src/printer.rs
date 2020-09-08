use crate::element::Element;
use crate::node::{ Type, Node };

pub fn tokens(text: &str, nodes: &Vec<Node>) {
    for node in nodes {
        match &node.r#type {
            Type::Token(left, right) => println!("{} {:?}", node.element.name, &text[*left..*right]),
            Type::Production(children) => tokens(text, children),
        }
    }
}

pub fn tree(tree: &Node) {
	node(tree, String::from(""), String::from(""));
}

fn node(tree: &Node, prefix: String, infix: String) {
    element(&prefix, tree.element);
    if let Type::Production(children) = &tree.r#type {
        for i in 0..children.len() {
            let (next_prefix, next_suffix) = if i == children.len() - 1 {
                (format!("{}{}", infix, "└─"), format!("{}{}", infix, "  "))
            } else {
                (format!("{}{}", infix, "├─"), format!("{}{}", infix, "│ "))
            };

            node(&children[i], next_prefix, next_suffix);
        }
    }
}

fn element(prefix: &String, element: &Element) {
    println!("{}{}", prefix, element.name);
}
