use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::utilities::ReturnReference;
use crate::runtime::utilities::builder;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { any, string, .. } = engine.primitives;
    engine.add_constant_value("String", string);
    builder::method(engine, string, "to_string", [string],    &to_string);
    builder::method(engine, string, "__eq__",    [string, any], &eq);
    builder::method(engine, string, "__add__",   [string, any], &add);
}

fn to_string<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_constant(arguments[0]))
}

fn eq<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_boolean(if arguments[1].isa(engine.primitives.string) {
        arguments[0].data_string() == arguments[1].data_string()
    } else {
        false
    }))
}

fn add<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let right = arguments[1].call_to_string(engine)?;
    Ok(engine.new_string(format!("{}{}", arguments[0].data_string(), right)))
}
