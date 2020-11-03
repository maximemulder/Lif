use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::value::GcValue;

pub fn to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_string(arguments[0].data_integer().to_string()))
}

pub fn cmp<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_boolean(*arguments[0].data_integer() == *arguments[1].data_integer()))
}

pub fn lt<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_boolean(*arguments[0].data_integer() < *arguments[1].data_integer()))
}

pub fn pos<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(*arguments[0].data_integer()))
}

pub fn neg<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(-arguments[0].data_integer()))
}

pub fn add<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(*arguments[0].data_integer() + *arguments[1].data_integer()))
}

pub fn sub<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(*arguments[0].data_integer() - *arguments[1].data_integer()))
}

pub fn mul<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(*arguments[0].data_integer() * *arguments[1].data_integer()))
}

pub fn div<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(*arguments[0].data_integer() / *arguments[1].data_integer()))
}

pub fn rem<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(*arguments[0].data_integer() % *arguments[1].data_integer()))
}
