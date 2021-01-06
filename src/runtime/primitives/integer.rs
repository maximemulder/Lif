use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::value::GcValue;

use std::mem::size_of;

pub fn populate(engine: &mut Engine) {
    let Primitives { any, integer, .. } = engine.primitives;
    engine.add_constant_value("Integer", integer);
    engine.add_method_primitive(integer, "to_string", [integer],          &to_string);
    engine.add_method_primitive(integer, "__eq__",    [integer, any],     &eq);
    engine.add_method_primitive(integer, "__lt__",    [integer, integer], &lt);
    engine.add_method_primitive(integer, "__pos__",   [integer],          &pos);
    engine.add_method_primitive(integer, "__neg__",   [integer],          &neg);
    engine.add_method_primitive(integer, "__add__",   [integer, integer], &add);
    engine.add_method_primitive(integer, "__sub__",   [integer, integer], &sub);
    engine.add_method_primitive(integer, "__mul__",   [integer, integer], &mul);
    engine.add_method_primitive(integer, "__div__",   [integer, integer], &div);
    engine.add_method_primitive(integer, "__rem__",   [integer, integer], &rem);
    engine.add_method_primitive(integer, "__bnot__",  [integer],          &bnot);
    engine.add_method_primitive(integer, "__band__",  [integer, integer], &band);
    engine.add_method_primitive(integer, "__bor__",   [integer, integer], &bor);
    engine.add_method_primitive(integer, "__bxor__",  [integer, integer], &bxor);
    engine.add_method_primitive(integer, "__bls__",   [integer, integer], &bls);
    engine.add_method_primitive(integer, "__brs__",   [integer, integer], &brs);
    engine.add_method_primitive(integer, "__bcls__",  [integer, integer], &bcls);
    engine.add_method_primitive(integer, "__bcrs__",  [integer, integer], &bcrs);
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
