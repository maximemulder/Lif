use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { array, function_primitive, function_standard, .. } = engine.primitives;
    engine.add_method_primitive(function_primitive, "to_string", [function_primitive],        &to_string);
    engine.add_method_primitive(function_primitive, "__cl__",    [function_primitive, array], &cl_primitive);
    engine.add_method_primitive(function_standard, "to_string",  [function_standard],         &to_string);
    engine.add_method_primitive(function_standard, "__cl__",     [function_standard, array],  &cl_standard);
}

fn to_string<'a>(engine: &mut Engine<'a>, _: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_string("FUNCTION".to_string()))
}

fn cl_primitive<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let mut array = Vec::new();
    for argument in arguments[1].data_array().iter() {
        array.push(argument.read()?);
    }

    arguments[0].data_function_primitive().call(engine, array)
}

fn cl_standard<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let mut array = Vec::new();
    for argument in arguments[1].data_array().iter() {
        array.push(argument.read()?);
    }

    arguments[0].data_function().call(engine, array)
}
