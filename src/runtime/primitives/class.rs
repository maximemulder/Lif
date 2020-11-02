use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::value::GcValue;

pub fn to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let mut string = String::new();
    string += "Class";
    if let Some(name) = &arguments[0].data_class().tag.get_name() {
        string += "(";
        string += name;
        string += ")";
    }

    Ok(engine.new_string(string))
}

pub fn chain<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let mut this = arguments[0];
    let name = arguments[1].data_string().clone();
    if let Some(method) = this.get_method(&name) {
        return Ok(engine.new_method(method, this));
    }

    let member = engine.undefined();
    let class = this.data_class_mut();
    Ok(if let Some(&member) = class.statics.get(&name) {
        member
    } else {
        class.statics.insert(name.clone(), member);
        member
    })
}

pub fn access<'a, 'b>(engine: &mut Engine<'a, 'b>, _: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_constant(engine.primitives.array))
}
