use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { class, string, .. } = engine.primitives;
    engine.add_constant_value("Class", class);
    engine.add_method_primitive(class, "to_string", [class],         &to_string);
    engine.add_method_primitive(class, "__cn__",    [class, string], &cn);
    engine.add_method_primitive(class, "__id__",    [class],         &id);
}

fn to_string<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let mut string = String::new();
    string += "Class";
    if let Some(name) = &arguments[0].data_class().tag.get_name() {
        string += "(";
        string += name;
        string += ")";
    }

    Ok(engine.new_string(string))
}

fn cn<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let mut this = arguments[0];
    let name = arguments[1].data_string().clone();
    if let Some(method) = this.get_method(&name) {
        println!("{}", name);
        return Ok(engine.new_method(method, this));
    }

    let member = engine.undefined();
    let class = this.data_class_mut();
    Ok(if let Some(&member) = class.statics.get(&name) {
        member
    } else {
        class.statics.insert(name, member);
        member
    })
}

fn id<'a>(engine: &mut Engine<'a>, _: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_constant(engine.primitives.array))
}
