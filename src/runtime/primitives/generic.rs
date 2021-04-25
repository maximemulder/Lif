use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::utilities::{ Arguments, ReturnReference };
use crate::runtime::utilities::builder;
use crate::runtime::utilities::parameters;

pub fn populate(engine: &mut Engine) {
    let Primitives { array_any, generic, .. } = engine.primitives;
    engine.add_constant_value("Generic", generic);
    builder::method(engine, generic, "to_string", [generic],            &to_string);
    builder::method(engine, generic, "__gn__",    [generic, array_any], &gn);
}

fn to_string<'a>(engine: &mut Engine<'a>, _: Arguments<'a>) -> ReturnReference<'a> {
    Ok(engine.new_string("GENERIC".to_string()))
}

fn gn<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let parameters = parameters::unpack(arguments[1])?;
    let generic = arguments[0];
    generic.clone().data_generic_mut().call(engine, generic, parameters)
}
