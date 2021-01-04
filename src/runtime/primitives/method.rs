use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::value::GcValue;

pub fn to_string<'a>(engine: &mut Engine<'a>, _: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_string("METHOD".to_string()))
}

pub fn gn<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let method = arguments[0].data_method();
    let mut elements = Vec::new();
    for argument in arguments[1].data_array().iter() {
        elements.push(*argument);
    }

    let array = engine.new_array_value(elements);
    let function = method.function.call_method(engine, "__gn__", vec![array])?.read()?;
    Ok(engine.new_method(function, method.this))
}

pub fn cl<'a>(engine: &mut Engine<'a>, mut arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let this = arguments[0].data_method().this;
    arguments[1].data_array_mut().insert(0, engine.new_constant(this));
    let method = arguments[0].data_method();
    method.function.call_method(engine, "__cl__", vec![arguments[1]])
}
