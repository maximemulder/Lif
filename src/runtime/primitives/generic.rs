use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::value::GcValue;
use crate::runtime::utilities::parameters;

pub fn populate(engine: &mut Engine) {
    let Primitives { array, generic, generic_code, generic_primitive, .. } = engine.primitives;
    engine.add_constant_value("Generic", generic);
    engine.add_method_primitive(generic,           "to_string", [generic],                  &to_string);
    engine.add_method_primitive(generic_code,      "__gn__",    [generic_code, array],      &gn_code);
    engine.add_method_primitive(generic_primitive, "__gn__",    [generic_primitive, array], &gn_primitive);
}

fn to_string<'a>(engine: &mut Engine<'a>, _: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_string("GENERIC".to_string()))
}

fn gn_code<'a>(engine: &mut Engine<'a>, mut arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let parameters = parameters::unpack(arguments[1])?;
    arguments[0].data_generic_mut().call(engine, parameters)
}

fn gn_primitive<'a>(engine: &mut Engine<'a>, mut arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let parameters = parameters::unpack(arguments[1])?;
    arguments[0].data_generic_primitive_mut().call(engine, parameters)
}
