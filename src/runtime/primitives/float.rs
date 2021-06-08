use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::utilities::Arguments;
use crate::runtime::utilities::builder;

pub fn populate(engine: &mut Engine) {
    let Primitives { any, float, .. } = engine.primitives;
    engine.set_constant_value("Float", float);
    builder::method(engine, float, "to_string", [float],          &to_string);
    builder::method(engine, float, "__eq__",    [float, any],     &eq);
    builder::method(engine, float, "__lt__",    [float, float], &lt);
    builder::method(engine, float, "__pos__",   [float],        &pos);
    builder::method(engine, float, "__neg__",   [float],        &neg);
    builder::method(engine, float, "__add__",   [float, float], &add);
    builder::method(engine, float, "__sub__",   [float, float], &sub);
    builder::method(engine, float, "__mul__",   [float, float], &mul);
    builder::method(engine, float, "__div__",   [float, float], &div);
    builder::method(engine, float, "__rem__",   [float, float], &rem);
}

fn to_string<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    Ok(engine.new_string(arguments[0].data_float().to_string()))
}

fn eq<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    Ok(engine.new_boolean(if arguments[1].isa(engine.primitives.integer) {
        *arguments[0].data_float() == *arguments[1].data_float()
    } else {
        false
    }))
}

fn lt<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    Ok(engine.new_boolean(*arguments[0].data_float() < *arguments[1].data_float()))
}

fn pos<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    Ok(engine.new_float(*arguments[0].data_float()))
}

fn neg<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    Ok(engine.new_float(-arguments[0].data_float()))
}

fn add<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    Ok(engine.new_float(*arguments[0].data_float() + *arguments[1].data_float()))
}

fn sub<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    Ok(engine.new_float(*arguments[0].data_float() - *arguments[1].data_float()))
}

fn mul<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    Ok(engine.new_float(*arguments[0].data_float() * *arguments[1].data_float()))
}

fn div<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    Ok(engine.new_float(*arguments[0].data_float() / *arguments[1].data_float()))
}

fn rem<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    Ok(engine.new_float(*arguments[0].data_float() % *arguments[1].data_float()))
}
