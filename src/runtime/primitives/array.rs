use crate::runtime::engine::Engine;
use crate::runtime::utilities::{ Arguments, ReturnReference, ReturnValue };
use crate::runtime::utilities::builder;

pub fn populate(engine: &mut Engine) {
    engine.add_constant_value("Array", engine.primitives.array);
}

pub fn create<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let class = arguments[0];
    class.cast(engine.primitives.class)?;
    let array = engine.new_class_value(None, engine.primitives.any);
    builder::method(engine, array, "to_string", [array],               &to_string);
    builder::method(engine, array, "insert",    [array, engine.primitives.integer, class], &insert);
    builder::method(engine, array, "remove",    [array, engine.primitives.integer],        &remove);
    builder::method(engine, array, "__id__",    [array, array],        &id);
    builder::method_rest(engine, array, "prepend", [array, class], &prepend);
    builder::method_rest(engine, array, "append",  [array, class], &append);
    Ok(engine.new_constant(array))
}

fn get_type<'a>(engine: &mut Engine<'a>) -> ReturnValue<'a> {
    engine.get_variable("__type__")?.read()
}

fn to_string<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let mut string = String::from("[");
    let elements = arguments[0].data_array().elements.clone();
    for element in elements.iter() {
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
    for index in 1 .. arguments.len() {
        let r#type = get_type(engine)?;
        let reference = engine.new_variable(Some(arguments[index]), r#type);
        arguments[0].data_array_mut().elements.push(reference);
    }

    Ok(engine.undefined())
}

fn prepend<'a>(engine: &mut Engine<'a>, mut arguments: Arguments<'a>) -> ReturnReference<'a> {
    for index in 1 .. arguments.len() {
        let r#type = get_type(engine)?;
        let reference = engine.new_variable(Some(arguments[index]), r#type);
        arguments[0].data_array_mut().elements.insert(index - 1, reference);
    }

    Ok(engine.undefined())
}

fn insert<'a>(engine: &mut Engine<'a>, mut arguments: Arguments<'a>) -> ReturnReference<'a> {
    let index = *arguments[1].data_integer() as usize;
    let r#type = get_type(engine)?;
    let reference = engine.new_variable(Some(arguments[2]), r#type);
    arguments[0].data_array_mut().elements.insert(index, reference);
    Ok(engine.undefined())
}

fn remove<'a>(engine: &mut Engine<'a>, mut arguments: Arguments<'a>) -> ReturnReference<'a> {
    let index = *arguments[1].data_integer() as usize;
    arguments[0].data_array_mut().elements.remove(index);
    Ok(engine.undefined())
}

fn id<'a>(_: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    Ok(arguments[0].data_array().elements[*arguments[1].data_array().elements[0].read()?.data_integer() as usize])
}
