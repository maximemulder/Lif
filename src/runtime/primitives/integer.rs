use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::value::GcValue;

use std::mem::size_of;

pub fn to_string<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_string(arguments[0].data_integer().to_string()))
}

pub fn eq<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_boolean(if arguments[1].isa(engine.primitives.integer) {
        *arguments[0].data_integer() == *arguments[1].data_integer()
    } else {
        false
    }))
}

pub fn lt<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_boolean(*arguments[0].data_integer() < *arguments[1].data_integer()))
}

pub fn pos<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(*arguments[0].data_integer()))
}

pub fn neg<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(-arguments[0].data_integer()))
}

pub fn add<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(*arguments[0].data_integer() + *arguments[1].data_integer()))
}

pub fn sub<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(*arguments[0].data_integer() - *arguments[1].data_integer()))
}

pub fn mul<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(*arguments[0].data_integer() * *arguments[1].data_integer()))
}

pub fn div<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(*arguments[0].data_integer() / *arguments[1].data_integer()))
}

pub fn rem<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(*arguments[0].data_integer() % *arguments[1].data_integer()))
}

pub fn bnot<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(!arguments[0].data_integer()))
}

pub fn band<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].data_integer() & arguments[1].data_integer()))
}

pub fn bor<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].data_integer() | arguments[1].data_integer()))
}

pub fn bxor<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].data_integer() ^ arguments[1].data_integer()))
}

pub fn bls<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].data_integer() << arguments[1].data_integer()))
}

pub fn brs<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_integer(arguments[0].data_integer() >> arguments[1].data_integer()))
}

pub fn bcls<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let x = *arguments[0].data_integer();
    let y = *arguments[1].data_integer();
    Ok(engine.new_integer((x << y) | (x >> (-y & size_of::<usize>() as isize))))
}

pub fn bcrs<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let x = *arguments[0].data_integer();
    let y = *arguments[1].data_integer();
    Ok(engine.new_integer((x >> y) | (x << (-y & size_of::<usize>() as isize))))
}
