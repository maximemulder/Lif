use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::value::GcValue;

pub fn to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
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

pub fn copy<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_array(arguments[0].data_array().clone()))
}

pub fn append<'a, 'b>(engine: &mut Engine<'a, 'b>, mut arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let reference = engine.new_reference(arguments[1]);
    arguments[0].data_array_mut().push(reference);
    Ok(engine.undefined())
}

pub fn prepend<'a, 'b>(engine: &mut Engine<'a, 'b>, mut arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let reference = engine.new_reference(arguments[1]);
    arguments[0].data_array_mut().insert(0, reference);
    Ok(engine.undefined())
}

pub fn insert<'a, 'b>(engine: &mut Engine<'a, 'b>, mut arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let reference = engine.new_reference(arguments[2]);
    let index = *arguments[1].data_integer() as usize;
    arguments[0].data_array_mut().insert(index, reference);
    Ok(engine.undefined())
}

pub fn remove<'a, 'b>(engine: &mut Engine<'a, 'b>, mut arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let index = *arguments[1].data_integer() as usize;
    arguments[0].data_array_mut().remove(index);
    Ok(engine.undefined())
}

pub fn id<'a, 'b>(_: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(arguments[0].data_array()[*arguments[1].data_array()[0].read()?.data_integer() as usize])
}
