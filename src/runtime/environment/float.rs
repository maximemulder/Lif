use crate::runtime::engine::Engine;
use crate::runtime::environment::Environment;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::value::Value;

pub fn populate(engine: &mut Engine) {
    let Environment { any, boolean, float, string, .. } = engine.environment;
    engine.populate_class("Float", float);
    engine.primitive_method(float, "__sstr__", [], None, Some(string), &sstr);
    engine.primitive_method(float, "__eq__", [("other", any)], None, Some(boolean), &eq);
    engine.primitive_method(float, "__lt__", [("other", float)], None, Some(boolean), &lt);
    engine.primitive_method(float, "__pos__", [], None, Some(float), &pos);
    engine.primitive_method(float, "__neg__", [], None, Some(float), &neg);
    engine.primitive_method(float, "__add__", [("other", float)], None, Some(float), &add);
    engine.primitive_method(float, "__sub__", [("other", float)], None, Some(float), &sub);
    engine.primitive_method(float, "__mul__", [("other", float)], None, Some(float), &mul);
    engine.primitive_method(float, "__div__", [("other", float)], None, Some(float), &div);
    engine.primitive_method(float, "__rem__", [("other", float)], None, Some(float), &rem);
}

fn sstr<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_string(arguments[0].get::<f64>(engine).to_string()))
}

fn eq<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_boolean(if arguments[1].isa(engine.environment.float) {
        arguments[0].get::<f64>(engine) == arguments[1].get(engine)
    } else {
        false
    }))
}

fn lt<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_boolean(arguments[0].get::<f64>(engine) < arguments[1].get::<f64>(engine)))
}

fn pos<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_float(arguments[0].get::<f64>(engine)))
}

fn neg<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_float(-arguments[0].get::<f64>(engine)))
}

fn add<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_float(arguments[0].get::<f64>(engine) + arguments[1].get::<f64>(engine)))
}

fn sub<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_float(arguments[0].get::<f64>(engine) - arguments[1].get::<f64>(engine)))
}

fn mul<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_float(arguments[0].get::<f64>(engine) * arguments[1].get::<f64>(engine)))
}

fn div<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_float(arguments[0].get::<f64>(engine) / arguments[1].get::<f64>(engine)))
}

fn rem<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_float(arguments[0].get::<f64>(engine) % arguments[1].get::<f64>(engine)))
}
