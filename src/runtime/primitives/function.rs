use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::value::GcValue;


pub fn to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, _: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_string("FUNCTION".to_string()))
}

pub fn call<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let mut array = Vec::new();
    for argument in arguments[1].data_array().iter() {
        array.push(argument.read()?);
    }

    arguments[0].data_callable().duplicate().execute(engine, array)
}
