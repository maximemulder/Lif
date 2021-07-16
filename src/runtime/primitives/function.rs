use crate::runtime::data::Function;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::utilities::parameters;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { array_any, function, .. } = engine.primitives;
    engine.set_constant_value("Function", function);
    engine.primitive_method(function, "__cl__", [("arguments", array_any)], None, None, &call);
}

fn call<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    arguments[0].get_ref::<Function>(engine).call(engine, &mut parameters::unpack(engine, arguments[1])?)
}
