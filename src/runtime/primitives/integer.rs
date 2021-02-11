use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::utilities::ReturnReference;
use crate::runtime::utilities::builder;
use crate::runtime::value::GcValue;

use std::mem::size_of;

pub fn populate(engine: &mut Engine) {
    let Primitives { any, integer, .. } = engine.primitives;
    engine.add_constant_value("Integer", integer);
    builder::method(engine, integer, "to_string", [integer],          &to_string);
    builder::method(engine, integer, "__eq__",    [integer, any],     &eq);
    builder::method(engine, integer, "__lt__",    [integer, integer], &lt);
    builder::method(engine, integer, "__pos__",   [integer],          &pos);
    builder::method(engine, integer, "__neg__",   [integer],          &neg);
    builder::method(engine, integer, "__add__",   [integer, integer], &add);
    builder::method(engine, integer, "__sub__",   [integer, integer], &sub);
    builder::method(engine, integer, "__mul__",   [integer, integer], &mul);
    builder::method(engine, integer, "__div__",   [integer, integer], &div);
    builder::method(engine, integer, "__rem__",   [integer, integer], &rem);
    builder::method(engine, integer, "__bnot__",  [integer],          &bnot);
    builder::method(engine, integer, "__band__",  [integer, integer], &band);
    builder::method(engine, integer, "__bor__",   [integer, integer], &bor);
    builder::method(engine, integer, "__bxor__",  [integer, integer], &bxor);
    builder::method(engine, integer, "__bls__",   [integer, integer], &bls);
    builder::method(engine, integer, "__brs__",   [integer, integer], &brs);
    builder::method(engine, integer, "__bcls__",  [integer, integer], &bcls);
    builder::method(engine, integer, "__bcrs__",  [integer, integer], &bcrs);
}

fn to_string<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_string(arguments[0].data_integer().to_string()))
}

fn eq<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_boolean(if arguments[1].isa(engine.primitives.integer) {
        *arguments[0].data_integer() == *arguments[1].data_integer()
    } else {
        false
    }))
}

fn lt<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_boolean(*arguments[0].data_integer() < *arguments[1].data_integer()))
}

fn pos<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(*arguments[0].data_integer()))
}

fn neg<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(-arguments[0].data_integer()))
}

fn add<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(*arguments[0].data_integer() + *arguments[1].data_integer()))
}

fn sub<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(*arguments[0].data_integer() - *arguments[1].data_integer()))
}

fn mul<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(*arguments[0].data_integer() * *arguments[1].data_integer()))
}

fn div<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(*arguments[0].data_integer() / *arguments[1].data_integer()))
}

fn rem<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(*arguments[0].data_integer() % *arguments[1].data_integer()))
}

fn bnot<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(!arguments[0].data_integer()))
}

fn band<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].data_integer() & arguments[1].data_integer()))
}

fn bor<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].data_integer() | arguments[1].data_integer()))
}

fn bxor<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].data_integer() ^ arguments[1].data_integer()))
}

fn bls<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].data_integer() << arguments[1].data_integer()))
}

fn brs<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].data_integer() >> arguments[1].data_integer()))
}

fn bcls<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let x = *arguments[0].data_integer();
    let y = *arguments[1].data_integer();
    Ok(engine.new_integer((x << y) | (x >> (-y & size_of::<usize>() as isize))))
}

fn bcrs<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let x = *arguments[0].data_integer();
    let y = *arguments[1].data_integer();
    Ok(engine.new_integer((x >> y) | (x << (-y & size_of::<usize>() as isize))))
}
