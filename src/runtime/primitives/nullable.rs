use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::primitives::Primitives;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { any, nullable, .. } = engine.primitives;
    let null = engine.new_nullable_value(None);
    engine.add_constant_value("null", null);
    engine.add_constant_primitive("some", [any], &new);
    engine.add_constant_value("Option", nullable);
    engine.add_method_primitive(nullable, "to_string", [nullable],      &to_string);
    engine.add_method_primitive(nullable, "get",       [nullable],      &get);
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
    Ok(engine.new_nullable(Some(arguments[0])))
}

fn get<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    return if let Some(value) = arguments[0].data_nullable().option {
        Ok(engine.new_constant(value))
    } else {
        Err(Error::new_nullable())
    }
}
