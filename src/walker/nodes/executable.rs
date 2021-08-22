use crate::runtime::engine::Engine;
use crate::runtime::r#return::ReturnReference;

pub trait AExecutableTrait {
	fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a>;
}
