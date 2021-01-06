use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { any, boolean, .. } = engine.primitives;
    engine.add_constant_value("Boolean", boolean);
    engine.add_method_primitive(boolean, "to_string", [boolean],      &to_string);
    engine.add_method_primitive(boolean, "__eq__",    [boolean, any], &eq);
    engine.add_method_primitive(boolean, "__not__",   [boolean],      &not);
}

fn to_string<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_string(arguments[0].data_boolean().to_string()))
}

fn eq<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_boolean(if arguments[1].isa(engine.primitives.boolean) {
        arguments[0].data_boolean() == arguments[1].data_boolean()
    } else {
        false
    }))
}

fn not<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_boolean(!arguments[0].data_boolean()))
}
