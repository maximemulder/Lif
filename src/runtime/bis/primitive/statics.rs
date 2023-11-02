use crate::runtime::bis::Value;
use crate::runtime::bis::data::GcClass;
use crate::runtime::bis::engine::Engine;
use crate::runtime::bis::env::Env;
use crate::runtime::bis::flow::ResValue;
use crate::runtime::bis::primitive::functions::PrimFunction;

pub fn get_list_statics<'a>(env: &Env<'a>, class: GcClass<'a>) -> [PrimFunction<'a>; 1] {
    [
        PrimFunction::new_rest("__init__", [], ("values", env.any), class, list_init),
    ]
}

fn list_init<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_list(args))
}
