use crate::code::Code;
use crate::element::Element;
use crate::node::{ Content, Node };

pub fn tokens(code: &Code, nodes: &[Node]) {
    for node in nodes {
        match &node.content {
            Content::Token(_, _) => println!("{} {:?}", node.element.name, code.node_str(node)),
            Content::Production(children) => tokens(code, children),
        }
    }
}

pub fn tree(tree: &Node) {
    node(tree, "", "");
}

fn node(tree: &Node, prefix: &str, infix: &str) {
    element(&prefix, tree.element);
    if let Content::Production(children) = &tree.content {
        for i in 0 .. children.len() {
            let (next_prefix, next_suffix) = if i == children.len() - 1 {
                (format!("{}{}", infix, "└─"), format!("{}{}", infix, "  "))
            } else {
                (format!("{}{}", infix, "├─"), format!("{}{}", infix, "│ "))
            };

            node(&children[i], &next_prefix, &next_suffix);
        }
    }
}

fn element(prefix: &str, element: &Element) {
    println!("{}{}", prefix, element.name);
}
