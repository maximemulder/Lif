use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTraceable;
use crate::runtime::reference::GcReference;

pub trait Callable<'a, 'b>: GcTraceable {
	fn execute(&self, engine: &mut Engine<'a, 'b>, arguments: Vec<GcReference<'a, 'b>>) -> ReturnReference<'a, 'b>;
	fn duplicate<'c>(&'c self) -> Box<dyn Callable<'a, 'b> + 'c>;
}
