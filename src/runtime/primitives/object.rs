use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::value::GcValue;

pub fn to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
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

pub fn cn<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let mut this = arguments[0];
    let name = arguments[1].data_string().clone();
    if let Some(method) = this.get_method(&name) {
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
