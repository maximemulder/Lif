use crate::runtime::engine::Engine;
use crate::runtime::environment::Environment;
use crate::runtime::primitives::Generic;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::utilities::parameters;
use crate::runtime::value::Value;

pub fn populate(engine: &mut Engine) {
    let Environment { array_any, generic, .. } = engine.environment;
    engine.populate_class("Generic", generic);
    engine.primitive_method(generic, "__gn__", [("arguments", array_any)], None, None, &apply);
}

fn apply<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let generic = arguments[0].get_gc::<Generic>(engine);
    let mut values = parameters::unpack(engine, arguments[1])?;
    generic.clone().call(engine, generic, &mut values)
}
