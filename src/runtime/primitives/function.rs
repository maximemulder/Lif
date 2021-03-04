use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::utilities::{ Arguments, ReturnReference };
use crate::runtime::utilities::builder;
use crate::runtime::utilities::parameters;

pub fn populate(engine: &mut Engine) {
    let Primitives { array_any, function, function_code, function_primitive, .. } = engine.primitives;
    engine.add_constant_value("Function", function);
    builder::method(engine, function,           "to_string", [function],                      &to_string);
    builder::method(engine, function_code,      "__cl__",    [function_code, array_any],      &cl_code);
    builder::method(engine, function_primitive, "__cl__",    [function_primitive, array_any], &cl_primitive);
}

fn to_string<'a>(engine: &mut Engine<'a>, _: Arguments<'a>) -> ReturnReference<'a> {
    Ok(engine.new_string("FUNCTION".to_string()))
}

fn cl_code<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    arguments[0].data_function().call(engine, parameters::unpack(arguments[1])?)
}

fn cl_primitive<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    arguments[0].data_function_primitive().call(engine, parameters::unpack(arguments[1])?)
}
