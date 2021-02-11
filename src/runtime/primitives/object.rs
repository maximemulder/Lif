use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::utilities::ReturnReference;
use crate::runtime::utilities::builder;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { object, string, .. } = engine.primitives;
    engine.add_constant_value("Object", object);
    builder::method(engine, object, "to_string", [object],         &to_string);
    builder::method(engine, object, "__cn__",    [object, string], &cn);
}

fn to_string<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let mut string = String::from("{");
    let attributes = &arguments[0].data_object().attributes.clone();
    for (name, attribute) in attributes {
        string.push_str(&name);
        string.push_str(": ");
        string.push_str(&attribute.read()?.call_to_string(engine)?);
        string.push_str(", ");
    }

    if !attributes.is_empty() {
        string.truncate(string.len() - 2);
    }

    string.push('}');
    Ok(engine.new_string(string))
}

fn cn<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let mut this = arguments[0];
    let name = arguments[1].data_string().clone();
    if let Some(method) = this.class.data_class().get_method(&name) {
        return Ok(engine.new_method(method, this));
    }

    let member = engine.undefined();
    let object = this.data_object_mut();
    Ok(if let Some(&member) = object.attributes.get(&name) {
        member
    } else {
        object.attributes.insert(name, member);
        member
    })
}
