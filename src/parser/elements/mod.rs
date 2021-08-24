macro_rules! declare_node {
    ( $name:ident ) => {
        pub const $name: Element = Element::new(stringify!($name));
    }
}

pub mod definitions;
pub mod expressions;
pub mod ignores;
pub mod keywords;
pub mod literals;
pub mod productions;
pub mod structures;
pub mod symbols;
