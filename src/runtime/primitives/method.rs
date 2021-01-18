use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { array, method, .. } = engine.primitives;
    engine.add_constant_value("Method", method);
    engine.add_method_primitive(method, "to_string", [method],        &to_string);
    engine.add_method_primitive(method, "__gn__",    [method, array], &gn);
    engine.add_method_primitive(method, "__cl__",    [method, array], &cl);
}

fn to_string<'a>(engine: &mut Engine<'a>, _: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_string("METHOD".to_string()))
}

fn gn<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let method = arguments[0].data_method();
    let function = method.function.call_method(engine, "__gn__", vec![arguments[1]])?.read()?;
    Ok(engine.new_method(function, method.this))
}

fn cl<'a>(engine: &mut Engine<'a>, mut arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let this = arguments[0].data_method().this;
    arguments[1].data_array_mut().insert(0, engine.new_constant(this));
    let method = arguments[0].data_method();
    method.function.call_method(engine, "__cl__", vec![arguments[1]])
}
