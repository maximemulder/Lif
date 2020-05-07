use crate::element::Element;
use crate::node::{ Content, Node };

pub fn tokens(nodes: &Vec<Node>) {
    for node in nodes {
        match &node.content {
            Content::Token(string) => println!("{} {:?}", node.element.name, string),
            Content::Production(children) => tokens(children),
        }
    }
}

pub fn tree(tree: &Node) {
	node(tree, String::from(""), String::from(""));
}

fn node(tree: &Node, prefix: String, infix: String) {
    element(&prefix, tree.element);
    if let Content::Production(children) = &tree.content {
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
