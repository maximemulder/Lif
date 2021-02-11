use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::utilities::ReturnReference;
use crate::runtime::utilities::builder;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    engine.add_constant_value("Option", engine.primitives.nullable);
}

pub fn create<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let class = arguments[0];
    class.cast(engine.primitives.class)?;
    let nullable = engine.new_class_value(None, engine.primitives.any);
    builder::r#static(engine, nullable, "new",       [class],    &new);
    builder::r#static(engine, nullable, "null",      [],    &null);
    builder::method(engine, nullable, "to_string", [nullable], &to_string);
    builder::method(engine, nullable, "get",       [nullable], &get);
    Ok(engine.new_constant(nullable))
}

fn to_string<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let mut string = String::new();
    if let Some(value) = arguments[0].data_nullable().option {
        string.push_str("OPTION(");
        string.push_str(&value.call_to_string(engine)?);
        string.push_str(")");
    } else {
        string.push_str("NULL");
    }

    Ok(engine.new_string(string))
}

fn new<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let class = engine.get_variable("__type__")?.read()?;
    let mut generic = engine.primitives.nullable;
    let nullable = generic.data_generic_primitive_mut().call(engine, vec![class])?.read()?;
    Ok(engine.new_nullable(nullable, Some(arguments[0])))
}

fn null<'a>(engine: &mut Engine<'a>, _: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let class = engine.get_variable("__type__")?.read()?;
    let mut generic = engine.primitives.nullable;
    let nullable = generic.data_generic_primitive_mut().call(engine, vec![class])?.read()?;
    Ok(engine.new_nullable(nullable, None))
}

fn get<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    return if let Some(value) = arguments[0].data_nullable().option {
        Ok(engine.new_constant(value))
    } else {
        Err(Error::new_nullable())
    }
}
