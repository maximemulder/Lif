use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { array, function, string, .. } = engine.primitives;
    engine.add_constant_value("Function", function);
    engine.add_method_primitive(function, "to_string", [function],         &to_string);
    engine.add_method_primitive(function, "__cl__",    [function, array],  &cl);
    engine.add_method_primitive(function, "__cn__",    [function, string], &cn);
}

fn to_string<'a>(engine: &mut Engine<'a>, _: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_string("FUNCTION".to_string()))
}

fn cn<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let this = arguments[0];
    let name = arguments[1].data_string();
    if name == "__cl__" {
        return Ok(engine.new_constant(this))
    }

    Ok(engine.new_method(this.get_method(&name)?, this))
}

fn cl<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let mut array = Vec::new();
    for argument in arguments[1].data_array().iter() {
        array.push(argument.read()?);
    }

    arguments[0].data_callable().duplicate().execute(engine, array)
}
