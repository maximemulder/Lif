use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::value::GcValue;

pub fn to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_constant(arguments[0]))
}

pub fn eq<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_boolean(if arguments[1].isa(engine.primitives.string) {
        arguments[0].data_string() == arguments[1].data_string()
    } else {
        false
    }))
}

pub fn add<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let right = arguments[1].call_to_string(engine)?;
    Ok(engine.new_string(format!("{}{}", arguments[0].data_string(), right)))
}
