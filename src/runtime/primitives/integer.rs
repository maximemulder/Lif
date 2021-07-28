use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::value::Value;

use std::mem::size_of;

pub fn populate(engine: &mut Engine) {
    let Primitives { any, boolean, integer, string, .. } = engine.primitives;
    engine.populate_class("Integer", integer);
    engine.primitive_method(integer, "__sstr__", [], None, Some(string), &sstr);
    engine.primitive_method(integer, "__eq__", [("other", any)], None, Some(boolean), &eq);
    engine.primitive_method(integer, "__lt__", [("other", integer)], None, Some(boolean), &lt);
    engine.primitive_method(integer, "__pos__", [], None, Some(integer), &pos);
    engine.primitive_method(integer, "__neg__", [], None, Some(integer), &neg);
    engine.primitive_method(integer, "__add__", [("other", integer)], None, Some(integer), &add);
    engine.primitive_method(integer, "__sub__", [("other", integer)], None, Some(integer), &sub);
    engine.primitive_method(integer, "__mul__", [("other", integer)], None, Some(integer), &mul);
    engine.primitive_method(integer, "__div__", [("other", integer)], None, Some(integer), &div);
    engine.primitive_method(integer, "__rem__", [("other", integer)], None, Some(integer), &rem);
    engine.primitive_method(integer, "__bnot__", [], None, Some(integer), &bnot);
    engine.primitive_method(integer, "__band__", [("other", integer)], None, Some(integer), &band);
    engine.primitive_method(integer, "__bor__", [("other", integer)], None, Some(integer), &bor);
    engine.primitive_method(integer, "__bxor__", [("other", integer)], None, Some(integer), &bxor);
    engine.primitive_method(integer, "__bls__", [("other", integer)], None, Some(integer), &bls);
    engine.primitive_method(integer, "__brs__", [("other", integer)], None, Some(integer), &brs);
    engine.primitive_method(integer, "__bcls__", [("other", integer)], None, Some(integer), &bcls);
    engine.primitive_method(integer, "__bcrs__", [("other", integer)], None, Some(integer), &bcrs);
}

fn sstr<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_string(arguments[0].get::<isize>(engine).to_string()))
}

fn eq<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_boolean(if arguments[1].isa(engine.primitives.integer) {
        arguments[0].get::<isize>(engine) == arguments[1].get(engine)
    } else {
        false
    }))
}

fn lt<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_boolean(arguments[0].get::<isize>(engine) < arguments[1].get::<isize>(engine)))
}

fn pos<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].get::<isize>(engine)))
}

fn neg<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_integer(-arguments[0].get::<isize>(engine)))
}

fn add<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].get::<isize>(engine) + arguments[1].get::<isize>(engine)))
}

fn sub<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].get::<isize>(engine) - arguments[1].get::<isize>(engine)))
}

fn mul<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].get::<isize>(engine) * arguments[1].get::<isize>(engine)))
}

fn div<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].get::<isize>(engine) / arguments[1].get::<isize>(engine)))
}

fn rem<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].get::<isize>(engine) % arguments[1].get::<isize>(engine)))
}

fn bnot<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_integer(!arguments[0].get::<isize>(engine)))
}

fn band<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].get::<isize>(engine) & arguments[1].get::<isize>(engine)))
}

fn bor<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].get::<isize>(engine) | arguments[1].get::<isize>(engine)))
}

fn bxor<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].get::<isize>(engine) ^ arguments[1].get::<isize>(engine)))
}

fn bls<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].get::<isize>(engine) << arguments[1].get::<isize>(engine)))
}

fn brs<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].get::<isize>(engine) >> arguments[1].get::<isize>(engine)))
}

fn bcls<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let x = arguments[0].get::<isize>(engine);
    let y = arguments[1].get::<isize>(engine);
    Ok(engine.new_integer((x << y) | (x >> (-y & size_of::<usize>() as isize))))
}

fn bcrs<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let x = arguments[0].get::<isize>(engine);
    let y = arguments[1].get::<isize>(engine);
    Ok(engine.new_integer((x >> y) | (x << (-y & size_of::<usize>() as isize))))
}
