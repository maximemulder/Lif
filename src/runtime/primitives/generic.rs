use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::utilities::builder;
use crate::runtime::utilities::parameters;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { array_any, generic, .. } = engine.primitives;
    engine.set_constant_value("Generic", generic);
    builder::method(engine, generic, "to_string", [generic],            &to_string);
    builder::method(engine, generic, "__gn__",    [generic, array_any], &gn);
}

fn to_string<'a>(engine: &mut Engine<'a>, _: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_string("GENERIC".to_string()))
}

fn gn<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let generic = arguments[0];
    let mut values = parameters::unpack(arguments[1])?;
    generic.clone().data_generic_mut().call(engine, generic, &mut values)
}
