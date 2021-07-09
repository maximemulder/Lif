use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { any, array_any, method, string, .. } = engine.primitives;
    engine.set_constant_value("Method", method);
    engine.primitive_method(method, "to_string", [], None, Some(string), &to_string);
    engine.primitive_method(method, "__gn__", [("arguments", array_any)], None, Some(any), &apply);
    engine.primitive_method(method, "__cl__", [("arguments", array_any)], None, None, &call);
}

fn to_string<'a>(engine: &mut Engine<'a>, _: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_string("METHOD".to_string()))
}

fn apply<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let method = arguments[0].data_method();
    let function = method.function.call_method(engine, "__gn__", &mut [arguments[1]])?.read()?;
    Ok(engine.new_method(function, method.this))
}

fn call<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let this = arguments[0].data_method().this;
    let reference = engine.new_reference(this);
    arguments[1].data_array_mut().insert(0, reference);
    let method = arguments[0].data_method();
    method.function.call_method(engine, "__cl__", &mut [arguments[1]])
}
