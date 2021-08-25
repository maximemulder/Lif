use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ ReturnFlow, ReturnJump, ReturnReference };
use crate::walker::ANode;
use crate::walker::nodes::AGenerics;

pub trait WExecutable {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a>;
}

pub trait WStatement {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnJump<'a>;
}

pub trait WDefinition {
    fn name(&self) -> &str;
    fn generics(&self) -> &ANode<AGenerics>;
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a>;
}

pub trait WExpression {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a>;
}

pub trait WStructure {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a>;
}

pub trait WLiteral {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a>;
}
