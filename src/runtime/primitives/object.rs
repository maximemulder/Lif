use crate::runtime::data::Object;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::{ Return, ReturnReference };
use crate::runtime::value::Value;

pub fn populate(engine: &mut Engine) {
    let Primitives { object, string, .. } = engine.primitives;
    engine.populate_class("Object", object);
    engine.primitive_method(object, "to_string", [], None, Some(string), &to_string);
    engine.primitive_method(object, "__cn__", [("property", string)], None, None, &chain);
}

fn to_string<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let mut string = String::from("{");
    string.push_str(&arguments[0].get_gc::<Object>(engine).attributes().iter()
        .map(|(name, attribute)| Ok(format!("{}: {}", &name, &attribute.read()?.call_sstr(engine)?)))
        .collect::<Return<Box<[String]>>>()?
        .join(", ")
    );

    string.push('}');
    Ok(engine.new_string(string))
}

fn chain<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let this = arguments[0];
    let name = &arguments[1].get_gc::<String>(engine);
    if let Some(method) = this.class.get_method(name) {
        return Ok(engine.new_method(method, this));
    }

    let member = engine.new_variable(None, engine.primitives.any);
    let mut object = this.get_gc::<Object>(engine);
    Ok(if let Some(member) = object.get_attribute(name) {
        member
    } else {
        object.set_attribute(name, member);
        member
    })
}
