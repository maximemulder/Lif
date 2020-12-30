use crate::runtime::ReturnReference;
use crate::runtime::data::Tag;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::value::GcValue;

pub trait Callable<'a>: GcTrace {
    fn execute(&self, engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a>;
    fn duplicate<'c>(&'c self) -> Box<dyn Callable<'a> + 'c>;
    fn get_tag(&self) -> Tag;
}
