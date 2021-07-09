use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { any, string, .. } = engine.primitives;
    engine.set_constant_value("String", string);
    engine.primitive_method(string, "to_string", [], None, Some(string), &to_string);
    engine.primitive_method(string, "__eq__", [("other", any)], None, Some(string), &eq);
    engine.primitive_method(string, "__add__", [("other", any)], None, Some(string), &add);
}

fn to_string<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_constant(arguments[0]))
}

fn eq<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_boolean(if arguments[1].isa(engine.primitives.string) {
        arguments[0].data_string() == arguments[1].data_string()
    } else {
        false
    }))
}

fn add<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let right = arguments[1].call_to_string(engine)?;
    Ok(engine.new_string(format!("{}{}", arguments[0].data_string(), right)))
}
