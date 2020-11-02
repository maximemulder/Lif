use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::value::GcValue;

pub fn to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, _: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_string("METHOD".to_string()))
}

pub fn apply<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let method = arguments[0].data_method();
    let mut elements = Vec::new();
    for argument in arguments[1].data_array().iter() {
        elements.push(*argument);
    }

    let array = engine.new_array_value(elements);
    let function = method.function.call_method(engine, "<>", vec![array])?.read()?;
    Ok(engine.new_method(function, method.this))
}

pub fn call<'a, 'b>(engine: &mut Engine<'a, 'b>, mut arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let this = arguments[0].data_method().this;
    arguments[1].data_array_mut().insert(0, engine.new_constant(this));
    let method = arguments[0].data_method();
    method.function.call_method(engine, "()", vec![arguments[1]])
}
