use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::value::GcValue;

pub fn to_string<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let mut string = String::from("[");
    let elements = arguments[0].data_array().clone();
    for element in elements.iter() {
        string.push_str(&element.read()?.call_to_string(engine)?);
        string.push_str(", ");
    }

    if !elements.is_empty() {
        string.truncate(string.len() - 2);
    }

    string.push(']');
    Ok(engine.new_string(string))
}

pub fn copy<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_array(arguments[0].data_array().clone()))
}

pub fn append<'a>(engine: &mut Engine<'a>, mut arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let reference = engine.new_reference(arguments[1]);
    arguments[0].data_array_mut().push(reference);
    Ok(engine.undefined())
}

pub fn prepend<'a>(engine: &mut Engine<'a>, mut arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let reference = engine.new_reference(arguments[1]);
    arguments[0].data_array_mut().insert(0, reference);
    Ok(engine.undefined())
}

pub fn insert<'a>(engine: &mut Engine<'a>, mut arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let reference = engine.new_reference(arguments[2]);
    let index = *arguments[1].data_integer() as usize;
    arguments[0].data_array_mut().insert(index, reference);
    Ok(engine.undefined())
}

pub fn remove<'a>(engine: &mut Engine<'a>, mut arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let index = *arguments[1].data_integer() as usize;
    arguments[0].data_array_mut().remove(index);
    Ok(engine.undefined())
}

pub fn id<'a>(_: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(arguments[0].data_array()[*arguments[1].data_array()[0].read()?.data_integer() as usize])
}
