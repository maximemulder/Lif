use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::r#return::Return;
use crate::runtime::utilities::Arguments;
use crate::runtime::value::GcValue;

pub fn length(arguments: usize, parameters: usize) -> Return<()> {
    if arguments != parameters {
        return Err(error_arguments(parameters, arguments));
    }

    Ok(())
}

pub fn pack<'a>(engine: &mut Engine<'a>, values: Arguments<'a>) -> GcValue<'a> {
    let elements = values.iter()
        .copied()
        .map(|value| engine.new_constant(value))
        .collect();

    engine.new_array_any_value(elements)
}

pub fn unpack(value: GcValue<'_>) -> Return<Arguments<'_>> {
    value.data_array().elements().iter()
        .copied()
        .map(|element| element.read())
        .collect()
}

fn error_arguments(parameters: usize, arguments: usize) -> Error {
    Error::new_runtime(&format!("Provided {} arguments while the function expects {} parameters.", arguments, parameters))
}
