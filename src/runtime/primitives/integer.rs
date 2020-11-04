use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::value::GcValue;

use std::mem::size_of;

pub fn to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_string(arguments[0].data_integer().to_string()))
}

pub fn eq<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_boolean(if arguments[1].isa(engine.primitives.integer) {
        *arguments[0].data_integer() == *arguments[1].data_integer()
    } else {
        false
    }))
}

pub fn lt<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_boolean(*arguments[0].data_integer() < *arguments[1].data_integer()))
}

pub fn pos<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(*arguments[0].data_integer()))
}

pub fn neg<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(-arguments[0].data_integer()))
}

pub fn add<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(*arguments[0].data_integer() + *arguments[1].data_integer()))
}

pub fn sub<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(*arguments[0].data_integer() - *arguments[1].data_integer()))
}

pub fn mul<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(*arguments[0].data_integer() * *arguments[1].data_integer()))
}

pub fn div<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(*arguments[0].data_integer() / *arguments[1].data_integer()))
}

pub fn rem<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(*arguments[0].data_integer() % *arguments[1].data_integer()))
}

pub fn bnot<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(!arguments[0].data_integer()))
}

pub fn band<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(arguments[0].data_integer() & arguments[1].data_integer()))
}

pub fn bor<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(arguments[0].data_integer() | arguments[1].data_integer()))
}

pub fn bxor<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(arguments[0].data_integer() ^ arguments[1].data_integer()))
}

pub fn bls<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(arguments[0].data_integer() << arguments[1].data_integer()))
}

pub fn brs<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(arguments[0].data_integer() >> arguments[1].data_integer()))
}

pub fn bcls<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let x = *arguments[0].data_integer();
    let y = *arguments[1].data_integer();
    Ok(engine.new_integer((x << y) | (x >> (-y & size_of::<usize>() as isize))))
}

pub fn bcrs<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let x = *arguments[0].data_integer();
    let y = *arguments[1].data_integer();
    Ok(engine.new_integer((x >> y) | (x << (-y & size_of::<usize>() as isize))))
}
