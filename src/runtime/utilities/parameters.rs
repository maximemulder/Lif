use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::utilities::{ Arguments, Return };
use crate::runtime::value::GcValue;

pub fn length(arguments: usize, parameters: usize) -> Return<()> {
    if arguments != parameters {
        return Err(Error::new_arguments(parameters, arguments));
    }

    Ok(())
}

pub fn pack<'a>(engine: &mut Engine<'a>, values: Arguments<'a>) -> GcValue<'a> {
    let mut references = Vec::new();
    for value in values.iter().copied() {
        references.push(engine.new_constant(value));
    }

    engine.new_array_any_value(references)
}

pub fn unpack(value: GcValue<'_>) -> Return<Arguments<'_>> {
    let mut elements = Vec::new();
    for reference in value.data_array().elements.iter() {
        elements.push(reference.read()?);
    }

    Ok(elements.into_boxed_slice())
}
