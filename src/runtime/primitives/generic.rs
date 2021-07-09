use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::utilities::parameters;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { array_any, generic, string, .. } = engine.primitives;
    engine.set_constant_value("Generic", generic);
    engine.primitive_method(generic, "to_string", [], None, Some(string), &to_string);
    engine.primitive_method(generic, "__gn__", [("arguments", array_any)], None, None, &apply);
}

fn to_string<'a>(engine: &mut Engine<'a>, _: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_string("GENERIC".to_string()))
}

fn apply<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let generic = arguments[0];
    let mut values = parameters::unpack(arguments[1])?;
    generic.clone().data_generic_mut().call(engine, generic, &mut values)
}
