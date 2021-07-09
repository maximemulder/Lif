use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { any, boolean, float, string, .. } = engine.primitives;
    engine.set_constant_value("Float", float);
    engine.primitive_method(float, "to_string", [], None, Some(string), &to_string);
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

fn to_string<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_string(arguments[0].data_float().to_string()))
}

fn eq<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_boolean(if arguments[1].isa(engine.primitives.integer) {
        *arguments[0].data_float() == *arguments[1].data_float()
    } else {
        false
    }))
}

fn lt<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_boolean(*arguments[0].data_float() < *arguments[1].data_float()))
}

fn pos<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_float(*arguments[0].data_float()))
}

fn neg<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_float(-arguments[0].data_float()))
}

fn add<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_float(*arguments[0].data_float() + *arguments[1].data_float()))
}

fn sub<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_float(*arguments[0].data_float() - *arguments[1].data_float()))
}

fn mul<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_float(*arguments[0].data_float() * *arguments[1].data_float()))
}

fn div<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_float(*arguments[0].data_float() / *arguments[1].data_float()))
}

fn rem<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_float(*arguments[0].data_float() % *arguments[1].data_float()))
}
