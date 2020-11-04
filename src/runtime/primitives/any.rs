use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::value::GcValue;

pub fn cn<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let this = arguments[0];
    let name = arguments[1].data_string();
    if let Some(method) = this.get_method(&name) {
        return Ok(engine.new_method(method, this));
    }

    Err(Error::new_undefined_method(&name, this))
}

pub fn eq<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_boolean(arguments[0] == arguments[1]))
}

pub fn ne<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let reference = arguments[0].call_method_self(engine, "__eq__", arguments)?;
    Ok(engine.new_boolean(!reference.read()?.data_boolean()))
}

pub fn gt<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let left  = arguments[0].call_method_self(engine, "__lt__", arguments.clone())?;
    let right = arguments[0].call_method_self(engine, "__eq__", arguments.clone())?;
    Ok(engine.new_boolean(!left.read()?.data_boolean() && !right.read()?.data_boolean()))
}

pub fn ge<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let reference = arguments[0].call_method_self(engine, "__lt__", arguments)?;
    Ok(engine.new_boolean(!reference.read()?.data_boolean()))
}

pub fn le<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let left  = arguments[0].call_method_self(engine, "__lt__", arguments.clone())?;
    let right = arguments[0].call_method_self(engine, "__eq__", arguments.clone())?;
    Ok(engine.new_boolean(*left.read()?.data_boolean() || *right.read()?.data_boolean()))
}
