use crate::parser::{ Code, Element, SNode, SNodeContent };

pub fn tokens(code: &Code, nodes: &[SNode]) {
    for node in nodes {
        match &node.content {
            SNodeContent::Token(_, _) => println!("{} {:?}", node.element.name, code.node_str(node)),
            SNodeContent::Production(children) => tokens(code, children),
        }
    }
}

pub fn tree(tree: &SNode) {
    node(tree, "", "");
}

fn node(tree: &SNode, prefix: &str, infix: &str) {
    element(&prefix, tree.element);
    if let SNodeContent::Production(children) = &tree.content {
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
