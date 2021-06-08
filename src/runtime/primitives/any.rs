use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::utilities::Arguments;
use crate::runtime::utilities::builder;

pub fn populate(engine: &mut Engine) {
    let Primitives { any, string, .. } = engine.primitives;
    engine.set_constant_value("Any", any);
    builder::method(engine, any, "__cn__", [any, string], &cn);
    builder::method(engine, any, "__eq__", [any, any],    &eq);
    builder::method(engine, any, "__ne__", [any, any],    &ne);
    builder::method(engine, any, "__gt__", [any, any],    &gt);
    builder::method(engine, any, "__le__", [any, any],    &le);
    builder::method(engine, any, "__ge__", [any, any],    &ge);
}

fn cn<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let this = arguments[0];
    let name = arguments[1].data_string();
    Ok(engine.new_method(this.get_method(&name)?, this))
}

fn eq<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    Ok(engine.new_boolean(arguments[0] == arguments[1]))
}

fn ne<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let reference = arguments[0].call_method_self(engine, "__eq__", arguments)?;
    Ok(engine.new_boolean(!reference.read()?.data_boolean()))
}

fn gt<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let left  = arguments[0].call_method_self(engine, "__lt__", arguments.clone())?;
    let right = arguments[0].call_method_self(engine, "__eq__", arguments.clone())?;
    Ok(engine.new_boolean(!left.read()?.data_boolean() && !right.read()?.data_boolean()))
}

fn ge<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let reference = arguments[0].call_method_self(engine, "__lt__", arguments)?;
    Ok(engine.new_boolean(!reference.read()?.data_boolean()))
}

fn le<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let left  = arguments[0].call_method_self(engine, "__lt__", arguments.clone())?;
    let right = arguments[0].call_method_self(engine, "__eq__", arguments.clone())?;
    Ok(engine.new_boolean(*left.read()?.data_boolean() || *right.read()?.data_boolean()))
}
