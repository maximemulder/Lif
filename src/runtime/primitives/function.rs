use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::value::GcValue;


pub fn to_string<'a>(engine: &mut Engine<'a>, _: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_string("FUNCTION".to_string()))
}

pub fn cl<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let mut array = Vec::new();
    for argument in arguments[1].data_array().iter() {
        array.push(argument.read()?);
    }

    arguments[0].data_callable().duplicate().execute(engine, array)
}
