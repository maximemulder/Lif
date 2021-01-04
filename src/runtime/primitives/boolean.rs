use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::value::GcValue;

pub fn to_string<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_string(arguments[0].data_boolean().to_string()))
}

pub fn eq<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_boolean(if arguments[1].isa(engine.primitives.boolean) {
        arguments[0].data_boolean() == arguments[1].data_boolean()
    } else {
        false
    }))
}

pub fn not<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_boolean(!arguments[0].data_boolean()))
}
