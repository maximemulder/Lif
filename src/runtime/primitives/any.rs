use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::value::GcValue;

pub fn chain<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let this = arguments[0];
    let name = arguments[1].data_string().clone();
    if let Some(method) = this.get_method(&name) {
        return Ok(engine.new_method(method, this));
    }

    Err(Error::new_undefined_method(&name, this))
}

pub fn comparison<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_boolean(arguments[0] == arguments[1]))
}

pub fn difference<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let reference = arguments[0].call_method_self(engine, "==", arguments)?;
    Ok(engine.new_boolean(!reference.read()?.data_boolean()))
}

pub fn greater<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let left  = arguments[0].call_method_self(engine, "<", arguments.clone())?;
    let right = arguments[0].call_method_self(engine, "==", arguments.clone())?;
    Ok(engine.new_boolean(!left.read()?.data_boolean() && !right.read()?.data_boolean()))
}

pub fn greater_equal<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let reference = arguments[0].call_method_self(engine, "<", arguments)?;
    Ok(engine.new_boolean(!reference.read()?.data_boolean()))
}

pub fn lesser_equal<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let left  = arguments[0].call_method_self(engine, "<", arguments.clone())?;
    let right = arguments[0].call_method_self(engine, "==", arguments.clone())?;
    Ok(engine.new_boolean(*left.read()?.data_boolean() || *right.read()?.data_boolean()))
}
