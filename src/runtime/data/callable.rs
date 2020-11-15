use crate::runtime::ReturnReference;
use crate::runtime::data::Tag;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::value::GcValue;

pub trait Callable<'a, 'b>: GcTrace {
    fn execute(&self, engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b>;
    fn duplicate<'c>(&'c self) -> Box<dyn Callable<'a, 'b> + 'c>;
    fn get_tag(&self) -> Tag;
}
