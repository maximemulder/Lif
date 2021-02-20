use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::utilities::{ Arguments, ReturnReference };
use crate::runtime::utilities::builder;

pub fn populate(engine: &mut Engine) {
    let Primitives { any, array, integer, .. } = engine.primitives;
    engine.add_constant_value("Array", array);
    builder::method(engine, array, "to_string", [array],               &to_string);
    builder::method(engine, array, "append",    [array, any],          &append);
    builder::method(engine, array, "prepend",   [array, any],          &prepend);
    builder::method(engine, array, "insert",    [array, integer, any], &insert);
    builder::method(engine, array, "remove",    [array, integer],      &remove);
    builder::method(engine, array, "__id__",    [array, array],        &id);
}

fn to_string<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let mut string = String::from("[");
    let elements = arguments[0].data_array().clone();
    for element in elements.slice() {
        string.push_str(&element.read()?.call_to_string(engine)?);
        string.push_str(", ");
    }

    if string.len() != 1 {
        string.truncate(string.len() - 2);
    }

    string.push(']');
    Ok(engine.new_string(string))
}

fn append<'a>(engine: &mut Engine<'a>, mut arguments: Arguments<'a>) -> ReturnReference<'a> {
    arguments[0].data_array_mut().push(engine, arguments[1]);
    Ok(engine.undefined())
}

fn prepend<'a>(engine: &mut Engine<'a>, mut arguments: Arguments<'a>) -> ReturnReference<'a> {
    arguments[0].data_array_mut().insert(engine, 0, arguments[1]);
    Ok(engine.undefined())
}

fn insert<'a>(engine: &mut Engine<'a>, mut arguments: Arguments<'a>) -> ReturnReference<'a> {
    let index = *arguments[1].data_integer() as usize;
    arguments[0].data_array_mut().insert(engine, index, arguments[2]);
    Ok(engine.undefined())
}

fn remove<'a>(engine: &mut Engine<'a>, mut arguments: Arguments<'a>) -> ReturnReference<'a> {
    let index = *arguments[1].data_integer() as usize;
    arguments[0].data_array_mut().remove(index);
    Ok(engine.undefined())
}

fn id<'a>(_: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    Ok(arguments[0].data_array().get(*arguments[1].data_array().get(0).read()?.data_integer() as usize))
}
