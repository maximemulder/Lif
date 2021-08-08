use crate::runtime::engine::Engine;
use crate::runtime::environment::Environment;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::value::Value;

use std::ops::Deref;

pub fn populate(engine: &mut Engine) {
    let Environment { any, string, .. } = engine.environment;
    engine.populate_class("String", string);
    engine.primitive_method(string, "__sstr__", [], None, Some(string), &sstr);
    engine.primitive_method(string, "__eq__", [("other", any)], None, Some(string), &eq);
    engine.primitive_method(string, "__add__", [("other", any)], None, Some(string), &add);
}

fn sstr<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_constant(arguments[0]))
}

fn eq<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_boolean(if arguments[1].isa(engine.environment.string) {
        arguments[0].get_gc::<String>(engine) == arguments[1].get_gc::<String>(engine)
    } else {
        false
    }))
}

fn add<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let right = arguments[1].call_fstr(engine)?;
    Ok(engine.new_string(format!("{}{}", arguments[0].get_gc::<String>(engine).deref(), right)))
}
