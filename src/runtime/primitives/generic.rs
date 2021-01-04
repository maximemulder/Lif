use crate::memory::Ref;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::value::GcValue;

pub fn to_string<'a>(engine: &mut Engine<'a>, _: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_string("GENERIC".to_string()))
}

pub fn gn<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    engine.push_scope();
    let value = arguments[0];
    let generic = value.data_generic();
    for (parameter, argument) in generic.generics.iter().zip(arguments[1].data_array()) {
        let reference = engine.new_reference(argument.read()?);
        engine.add_variable(parameter, reference);
    }

    let reference = engine.execute(Ref::as_ref(&generic.node))?;
    engine.pop_scope();
    Ok(reference)
}
