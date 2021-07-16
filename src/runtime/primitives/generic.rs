use crate::runtime::data::Generic;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::utilities::parameters;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { array_any, generic, .. } = engine.primitives;
    engine.set_constant_value("Generic", generic);
    engine.primitive_method(generic, "__gn__", [("arguments", array_any)], None, None, &apply);
}

fn apply<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let generic = arguments[0];
    let mut values = parameters::unpack(engine, arguments[1])?;
    generic.clone().get_mut::<Generic>(engine).call(engine, generic, &mut values)
}
