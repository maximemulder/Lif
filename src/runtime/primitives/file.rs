use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::value::Value;

use std::fs;
use std::ops::Deref;

pub fn populate(engine: &mut Engine) {
    let Primitives { any, file, string, .. } = engine.primitives;
    engine.populate_class("File", file);
    engine.primitive_static(file, "read", [("file", string)], None, Some(string), &read);
    engine.primitive_static(file, "write", [("file", string), ("content", any)], None, None, &write);
}

fn read<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let string = fs::read_to_string(arguments[0].get_gc::<String>(engine).deref()).unwrap();
    Ok(engine.new_string(string))
}

fn write<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    fs::write(arguments[0].get_gc::<String>(engine).deref(), arguments[1].call_fstr(engine)?).unwrap();
    Ok(engine.undefined())
}
