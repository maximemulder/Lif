use crate::runtime::engine::Engine;
use crate::runtime::environment::Environment;
use crate::runtime::error::Error;
use crate::runtime::primitives::{ Class, Function };
use crate::runtime::r#return::{ Return, ReturnReference };
use crate::runtime::utilities::parameters;
use crate::runtime::value::Value;

pub fn populate(engine: &mut Engine) {
    let Environment { any, array_any, class, string, .. } = engine.environment;
    engine.populate_class("Class", class);
    engine.primitive_method(class, "__sstr__", [], None, Some(string), &sstr);
    engine.primitive_method(class, "__cn__", [("property", string)], None, Some(any), &chain);
    engine.primitive_method(class, "__cl__", [("arguments", array_any)], None, Some(any), &call);
}

fn sstr<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let this = arguments[0].get_gc::<Class>(engine);
    let mut string = this.tag().to_string();
    if let Some(constructor) = this.constructor() {
        string.push_str("[");
        string.push_str(&constructor.arguments.iter()
            .map(|argument| argument.call_sstr(engine))
            .collect::<Return<Box<[_]>>>()?
            .join(", ")
        );

        string.push_str("]");
    }

    Ok(engine.new_string(string))
}

fn chain<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let this = arguments[0];
    let name = &arguments[1].get_gc::<String>(engine);
    if let Some(method) = this.class.get_method(name) {
        return Ok(engine.new_method(method, this));
    }

    let member = engine.new_variable(None, engine.environment.any);
    let mut class = this.get_gc::<Class>(engine);
    Ok(if let Some(member) = class.get_static(name) {
        member
    } else {
        class.set_static(name, member);
        member
    })
}

fn call<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let class = arguments[0];
    Ok(if let Some(member) = class.get_gc::<Class>(engine).get_static("__init__") {
        member.read()?.get_gc::<Function>(engine).call(engine, &mut parameters::unpack(engine, arguments[1])?)?
    } else {
        return Err(error_constructor(engine, class))
    })
}

fn error_constructor<'a>(engine: &Engine<'a>, class: Value<'a>) -> Error {
    Error::new_runtime(&format!("Class {} has no default constructor.", class.get_gc::<Class>(engine).tag()))
}
