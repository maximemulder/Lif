use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ ReturnFlow, ReturnJump, ReturnReference };

pub trait WExecutable {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a>;
}

pub trait WStatement {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnJump<'a>;
}

pub trait WDefinition {
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
