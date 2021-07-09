use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::utilities::parameters;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { array_any, function, string, .. } = engine.primitives;
    engine.set_constant_value("Function", function);
    engine.primitive_method(function, "to_string", [], None, Some(string), &to_string);
    engine.primitive_method(function, "__cl__", [("arguments", array_any)], None, None, &call);
}

fn to_string<'a>(engine: &mut Engine<'a>, _: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_string("FUNCTION".to_string()))
}

fn call<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    arguments[0].data_function().call(engine, &mut parameters::unpack(arguments[1])?)
}
