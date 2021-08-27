use crate::memory::Ref;
use crate::parser::{ Element, CNode };

pub fn tokens(nodes: &[CNode]) {
    for node in nodes {
        let children = node.children();
        if children.is_empty() {
            println!("{} {:?}", node.element.name, Ref::as_ref(&node.text()));
        } else {
            tokens(children);
        }
    }
}

pub fn tree(tree: &CNode) {
    node(tree, "", "");
}

fn node(tree: &CNode, prefix: &str, infix: &str) {
    element(&prefix, tree.element);
    let children = tree.children();
    for (i, child) in children.iter().enumerate() {
        let (next_prefix, next_suffix) = if i != children.len() - 1 {
            (format!("{}{}", infix, "├─"), format!("{}{}", infix, "│ "))
        } else {
            (format!("{}{}", infix, "└─"), format!("{}{}", infix, "  "))
        };

        node(child, &next_prefix, &next_suffix);
    }
}

fn element(prefix: &str, element: &Element) {
    println!("{}{}", prefix, element.name);
}
