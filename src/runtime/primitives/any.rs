use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { any, boolean, method, string, .. } = engine.primitives;
    engine.set_constant_value("Any", any);
    engine.primitive_method(any, "__cn__", [("property", string)], None, Some(method), &chain);
    engine.primitive_method(any, "__eq__", [("other", any)], None, Some(boolean), &eq);
    engine.primitive_method(any, "__ne__", [("other", any)], None, Some(boolean), &ne);
    engine.primitive_method(any, "__le__", [("other", any)], None, Some(boolean), &le);
    engine.primitive_method(any, "__gt__", [("other", any)], None, Some(boolean), &gt);
    engine.primitive_method(any, "__ge__", [("other", any)], None, Some(boolean), &ge);
}

fn chain<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let this = arguments[0];
    let name = arguments[1].data_string();
    Ok(engine.new_method(this.get_method(&name)?, this))
}

fn eq<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_boolean(arguments[0] == arguments[1]))
}

fn ne<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let reference = arguments[0].call_method_self(engine, "__eq__", arguments)?;
    Ok(engine.new_boolean(!reference.read()?.data_boolean()))
}

fn le<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let left  = arguments[0].call_method_self(engine, "__lt__", arguments)?;
    let right = arguments[0].call_method_self(engine, "__eq__", arguments)?;
    Ok(engine.new_boolean(*left.read()?.data_boolean() || *right.read()?.data_boolean()))
}

fn gt<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let left  = arguments[0].call_method_self(engine, "__lt__", arguments)?;
    let right = arguments[0].call_method_self(engine, "__eq__", arguments)?;
    Ok(engine.new_boolean(!left.read()?.data_boolean() && !right.read()?.data_boolean()))
}

fn ge<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let reference = arguments[0].call_method_self(engine, "__lt__", arguments)?;
    Ok(engine.new_boolean(!reference.read()?.data_boolean()))
}
