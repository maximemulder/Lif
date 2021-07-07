use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::utilities::Arguments;
use crate::runtime::utilities::builder;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    engine.set_constant_value("Array", engine.primitives.array);
}

pub fn create<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let Primitives { any, integer, .. } = engine.primitives;
    let class = arguments[0];
    class.cast(engine.primitives.class)?;
    let array = engine.new_class_value(None, Some(any));
    let array_any = if class == any {
        array
    } else {
        engine.primitives.array_any
    };

    builder::static_rest(engine, array, "__init__", [], &new);

    builder::method(engine, array, "to_string", [array],                 &to_string);
    builder::method(engine, array, "insert",    [array, integer, class], &insert);
    builder::method(engine, array, "remove",    [array, integer],        &remove);
    builder::method(engine, array, "__cl__",    [array, array_any],      &id);

    builder::method_rest(engine, array, "prepend", [array, class], &prepend);
    builder::method_rest(engine, array, "append",  [array, class], &append);

    Ok(engine.new_constant(array))
}

fn get_type<'a>(engine: &mut Engine<'a>) -> GcValue<'a> {
    engine.scope().parent().unwrap().source().unwrap().data_class().constructor.unwrap().arguments[0]
}

fn new<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let r#type = get_type(engine);
    let elements = arguments.into_iter()
    .copied()
    .map(|argument| engine.new_variable(Some(argument), r#type))
    .collect();
    Ok(engine.new_array(engine.scope().parent().unwrap().source().unwrap(), elements))
}

fn to_string<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let mut string = String::from("[");
    let elements = arguments[0].data_array().elements();
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
        let r#type = get_type(engine);
        let reference = engine.new_variable(Some(arguments[index]), r#type);
        arguments[0].data_array_mut().append(reference);
    }

    Ok(engine.undefined())
}

fn prepend<'a>(engine: &mut Engine<'a>, mut arguments: Arguments<'a>) -> ReturnReference<'a> {
    for index in 1 .. arguments.len() {
        let r#type = get_type(engine);
        let reference = engine.new_variable(Some(arguments[index]), r#type);
        arguments[0].data_array_mut().insert(index - 1, reference);
    }

    Ok(engine.undefined())
}

fn insert<'a>(engine: &mut Engine<'a>, mut arguments: Arguments<'a>) -> ReturnReference<'a> {
    let index = *arguments[1].data_integer() as usize;
    let r#type = get_type(engine);
    let reference = engine.new_variable(Some(arguments[2]), r#type);
    arguments[0].data_array_mut().insert(index, reference);
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
