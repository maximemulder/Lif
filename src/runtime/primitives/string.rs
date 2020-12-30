use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::value::GcValue;

pub fn to_string<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_constant(arguments[0]))
}

pub fn eq<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_boolean(if arguments[1].isa(engine.primitives.string) {
        arguments[0].data_string() == arguments[1].data_string()
    } else {
        false
    }))
}

pub fn add<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let right = arguments[1].call_to_string(engine)?;
    Ok(engine.new_string(format!("{}{}", arguments[0].data_string(), right)))
}
