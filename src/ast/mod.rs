use crate::memory::Ref;
use crate::parser::Code;

pub mod build;
pub mod nodes;

#[derive(Clone, Copy)]
pub struct Pos {
    pub source: Ref<Code>,
    pub start: usize,
    pub length: usize,
}
