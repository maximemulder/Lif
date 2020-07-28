use crate::element::Element;

pub const STRUCTURE: Element = Element::new("STRUCTURE");
pub const BLOCK:     Element = Element::new("BLOCK");
pub const IF:        Element = Element::new("IF");
pub const IF_BODY:   Element = Element::new("IF_BODY");
pub const IF_ELSE:   Element = Element::new("IF_ELSE");
pub const LOOP:      Element = Element::new("LOOP");
pub const LOOP_BODY: Element = Element::new("LOOP_BODY");
pub const WHILE:     Element = Element::new("WHILE");
pub const DO_WHILE:  Element = Element::new("DO_WHILE");
pub const FOR_IN:    Element = Element::new("FOR_IN");
pub const CONTINUE:  Element = Element::new("CONTINUE");
pub const BREAK:     Element = Element::new("BREAK");
pub const RETURN:    Element = Element::new("RETURN");