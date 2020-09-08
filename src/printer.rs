use crate::code::Code;
use crate::element::Element;
use crate::node::{ Type, Node };

pub fn tokens(code: &Code, nodes: &Vec<Node>) {
    for node in nodes {
        match &node.r#type {
			Type::Token(_, _) => println!("{} {:?}", node.element.name, code.node_str(node)),
            Type::Production(children) => tokens(code, children),
        }
    }
}

pub fn tree(tree: &Node) {
	node(tree, String::from(""), String::from(""));
}

fn node(tree: &Node, prefix: String, infix: String) {
    element(&prefix, tree.element);
    if let Type::Production(children) = &tree.r#type {
        for i in 0 .. children.len() {
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
