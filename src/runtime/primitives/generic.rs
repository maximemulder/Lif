use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::value::GcValue;

pub fn to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, _: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_string("GENERIC".to_string()))
}

pub fn apply<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    engine.push_scope();
    let value = arguments[0];
    let generic = value.data_generic();
    for (parameter, argument) in generic.generics.iter().zip(arguments[1].data_array()) {
        let reference = engine.new_reference(argument.read()?);
        engine.add_variable(parameter, reference);
    }

    let reference = generic.node.execute(engine)?;
    engine.pop_scope();
    Ok(reference)
}
