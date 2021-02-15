use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::utilities::{ Arguments, ReturnReference };
use crate::runtime::utilities::builder;
use crate::runtime::utilities::parameters;

pub fn populate(engine: &mut Engine) {
    let Primitives { array, generic, generic_code, generic_primitive, .. } = engine.primitives;
    engine.add_constant_value("Generic", generic);
    builder::method(engine, generic,           "to_string", [generic],                  &to_string);
    builder::method(engine, generic_code,      "__gn__",    [generic_code, array],      &gn_code);
    builder::method(engine, generic_primitive, "__gn__",    [generic_primitive, array], &gn_primitive);
}

fn to_string<'a>(engine: &mut Engine<'a>, _: Arguments<'a>) -> ReturnReference<'a> {
    Ok(engine.new_string("GENERIC".to_string()))
}

fn gn_code<'a>(engine: &mut Engine<'a>, mut arguments: Arguments<'a>) -> ReturnReference<'a> {
    let parameters = parameters::unpack(arguments[1])?;
    arguments[0].data_generic_mut().call(engine, parameters)
}

fn gn_primitive<'a>(engine: &mut Engine<'a>, mut arguments: Arguments<'a>) -> ReturnReference<'a> {
    let parameters = parameters::unpack(arguments[1])?;
    arguments[0].data_generic_primitive_mut().call(engine, parameters)
}
