use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::utilities::Arguments;
use crate::runtime::utilities::builder;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    engine.set_constant_value("Option", engine.primitives.nullable);
}

pub fn create<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let class = arguments[0];
    class.cast(engine.primitives.class)?;
    let nullable = engine.new_class_value(None, Some(engine.primitives.any));
    builder::r#static(engine, nullable, "new",     [class],    &new);
    builder::r#static(engine, nullable, "null",    [],         &null);
    builder::method(engine, nullable, "to_string", [nullable], &to_string);
    builder::method(engine, nullable, "get",       [nullable], &get);
    Ok(engine.new_constant(nullable))
}

fn get_type<'a>(engine: &mut Engine<'a>) -> GcValue<'a> {
    engine.scope().parent().unwrap().source().unwrap().data_class().constructor.unwrap().arguments[0]
}

fn to_string<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let mut string = String::new();
    if let Some(value) = arguments[0].data_nullable().option {
        string.push_str("OPTION(");
        string.push_str(&value.call_to_string(engine)?);
        string.push(')');
    } else {
        string.push_str("NULL");
    }

    Ok(engine.new_string(string))
}

fn new<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let class = get_type(engine);
    let generic = engine.primitives.nullable;
    let nullable = generic.clone().data_generic_mut().call(engine, generic, Box::new([class]))?.read()?;
    Ok(engine.new_nullable(nullable, Some(arguments[0])))
}

fn null<'a>(engine: &mut Engine<'a>, _: Arguments<'a>) -> ReturnReference<'a> {
    let class = get_type(engine);
    let generic = engine.primitives.nullable;
    let nullable = generic.clone().data_generic_mut().call(engine, generic, Box::new([class]))?.read()?;
    Ok(engine.new_nullable(nullable, None))
}

fn get<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    return if let Some(value) = arguments[0].data_nullable().option {
        Ok(engine.new_constant(value))
    } else {
        Err(Error::new_nullable())
    }
}
