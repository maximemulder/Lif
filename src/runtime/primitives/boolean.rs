use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::utilities::Arguments;
use crate::runtime::utilities::builder;

pub fn populate(engine: &mut Engine) {
    let Primitives { any, boolean, .. } = engine.primitives;
    engine.set_constant_value("Boolean", boolean);
    builder::method(engine, boolean, "to_string", [boolean],      &to_string);
    builder::method(engine, boolean, "__eq__",    [boolean, any], &eq);
    builder::method(engine, boolean, "__not__",   [boolean],      &not);
}

fn to_string<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    Ok(engine.new_string(arguments[0].data_boolean().to_string()))
}

fn eq<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    Ok(engine.new_boolean(if arguments[1].isa(engine.primitives.boolean) {
        arguments[0].data_boolean() == arguments[1].data_boolean()
    } else {
        false
    }))
}

fn not<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    Ok(engine.new_boolean(!arguments[0].data_boolean()))
}
