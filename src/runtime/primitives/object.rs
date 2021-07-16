use crate::runtime::data::{ Class, Object };
use crate::runtime::engine::Engine;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::{ Return, ReturnReference };
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    let Primitives { object, string, .. } = engine.primitives;
    engine.set_constant_value("Object", object);
    engine.primitive_method(object, "to_string", [], None, Some(string), &to_string);
    engine.primitive_method(object, "__cn__", [("property", string)], None, None, &chain);
}

fn to_string<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let mut string = String::from("{");
    string.push_str(&arguments[0].get_ref::<Object>(engine).attributes().iter()
        .map(|(name, attribute)| Ok(format!("{}: {}", &name, &attribute.read()?.call_sstr(engine)?)))
        .collect::<Return<Box<[String]>>>()?
        .join(", ")
    );

    string.push('}');
    Ok(engine.new_string(string))
}

fn chain<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let mut this = arguments[0];
    let name = arguments[1].get_ref::<String>(engine);
    if let Some(method) = this.class.get_ref::<Class>(engine).get_method(engine, name) {
        return Ok(engine.new_method(method, this));
    }

    let member = engine.new_variable(None, engine.primitives.any);
    let object = this.get_mut::<Object>(engine);
    Ok(if let Some(member) = object.get_attribute(name) {
        member
    } else {
        object.set_attribute(name, member);
        member
    })
}
