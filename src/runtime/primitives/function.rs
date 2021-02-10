use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { array, function, function_code, function_primitive, .. } = engine.primitives;
    engine.add_constant_value("Function", function);
    engine.add_method_primitive(function,           "to_string", [function],                  &to_string);
    engine.add_method_primitive(function_code,      "__cl__",    [function_code, array],      &cl_code);
    engine.add_method_primitive(function_primitive, "__cl__",    [function_primitive, array], &cl_primitive);
}

fn to_string<'a>(engine: &mut Engine<'a>, _: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_string("FUNCTION".to_string()))
}

fn cl_code<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let mut array = Vec::new();
    for argument in arguments[1].data_array().iter() {
        array.push(argument.read()?);
    }

    arguments[0].data_function().call(engine, array)
}

fn cl_primitive<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let mut array = Vec::new();
    for argument in arguments[1].data_array().iter() {
        array.push(argument.read()?);
    }

    arguments[0].data_function_primitive().call(engine, array)
}
