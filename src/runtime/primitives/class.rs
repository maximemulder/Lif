use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::utilities::Arguments;
use crate::runtime::utilities::builder;

pub fn populate(engine: &mut Engine) {
    let Primitives { class, string, .. } = engine.primitives;
    engine.set_constant_value("Class", class);
    builder::method(engine, class, "to_string", [class],         &to_string);
    builder::method(engine, class, "__cn__",    [class, string], &cn);
    builder::method(engine, class, "__id__",    [class],         &id);
}

fn to_string<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let mut string = String::new();
    string += "Class";
    if let Some(name) = arguments[0].data_class().tag().get_name() {
        string += "(";
        string += name;
        string += ")";
    }

    Ok(engine.new_string(string))
}

fn cn<'a>(engine: &mut Engine<'a>, arguments: Arguments<'a>) -> ReturnReference<'a> {
    let mut this = arguments[0];
    let name = arguments[1].data_string();
    if let Some(method) = this.class.data_class().get_method(name) {
        return Ok(engine.new_method(method, this));
    }

    let member = engine.undefined();
    let class = this.data_class_mut();
    Ok(if let Some(member) = class.get_static(name) {
        member
    } else {
        class.set_static(name, member);
        member
    })
}

fn id<'a>(engine: &mut Engine<'a>, _: Arguments<'a>) -> ReturnReference<'a> {
    Ok(engine.new_constant(engine.primitives.array))
}
