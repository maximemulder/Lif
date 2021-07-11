use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { any, boolean, string, .. } = engine.primitives;
    engine.set_constant_value("Boolean", boolean);
    engine.primitive_method(boolean, "__sstr__", [], None, Some(string), &sstr);
    engine.primitive_method(boolean, "__eq__", [("other", any)], None, Some(boolean), &eq);
    engine.primitive_method(boolean, "__not__", [], None, Some(boolean), &not);
}

fn sstr<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_string(arguments[0].data_boolean().to_string()))
}

fn eq<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_boolean(if arguments[1].isa(engine.primitives.boolean) {
        arguments[0].data_boolean() == arguments[1].data_boolean()
    } else {
        false
    }))
}

fn not<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_boolean(!arguments[0].data_boolean()))
}
