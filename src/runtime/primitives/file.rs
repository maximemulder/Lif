use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::utilities::Arguments;
use crate::runtime::utilities::builder;

use std::fs;

pub fn populate(engine: &mut Engine) {
    let Primitives { any, file, string, .. } = engine.primitives;
    engine.set_constant_value("File", file);
    builder::r#static(engine, file, "read",  [string],      &read);
    builder::r#static(engine, file, "write", [string, any], &write);
}

fn read<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let string = fs::read_to_string(arguments[0].data_string()).unwrap();
    Ok(engine.new_string(string))
}

fn write<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    fs::write(arguments[0].data_string(), arguments[1].call_to_string(engine)?).unwrap();
    Ok(engine.undefined())
}
