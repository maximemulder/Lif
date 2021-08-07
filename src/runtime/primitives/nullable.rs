use crate::runtime::data::{ Class, Nullable };
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::gc::GcRef;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::value::Value;

pub fn populate(engine: &mut Engine) {
    engine.populate_generic("Option", engine.primitives.nullable);
}

pub fn create<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let Primitives { any, string, .. } = engine.primitives;
    let r#type = arguments[0].get_cast_class(engine)?;
    let nullable = engine.primitive_class("Option", Some(any), true);
    engine.primitive_static(nullable, "some", [("value", r#type)], None, Some(nullable), &some);
    engine.primitive_static(nullable, "none", [], None, Some(nullable), &none);
    engine.primitive_method(nullable, "__fstr__", [], None, Some(string), &fstr);
    engine.primitive_method(nullable, "get", [], None, Some(r#type), &get);
    Ok(engine.new_constant(Value::primitive_gc(engine, nullable)))
}

fn get_type<'a>(engine: &Engine<'a>) -> GcRef<Class<'a>> {
    engine.scope().parent().unwrap().source().unwrap().get_gc::<Class>(engine).constructor().unwrap().arguments[0].get_gc::<Class>(engine)
}

fn fstr<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let this = arguments[0];
    let mut string = Value::primitive_gc(engine, this.class).call_sstr(engine)?;
    if let Some(value) = this.get_gn::<Nullable>(engine).option {
        string.push_str(".some(");
        string.push_str(&value.call_sstr(engine)?);
        string.push_str(")");
    } else {
        string.push_str(".none");
    }

    Ok(engine.new_string(string))
}

fn some<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let class = Value::primitive_gc(engine, get_type(engine));
    let generic = engine.primitives.nullable;
    let nullable = generic.clone().call(engine, generic, &mut [class])?.read()?.get_gc::<Class>(engine);
    Ok(engine.new_nullable(nullable, Some(arguments[0])))
}

fn none<'a>(engine: &mut Engine<'a>, _: &mut [Value<'a>]) -> ReturnReference<'a> {
    let class = Value::primitive_gc(engine, get_type(engine));
    let generic = engine.primitives.nullable;
    let nullable = generic.clone().call(engine, generic, &mut [class])?.read()?.get_gc::<Class>(engine);
    Ok(engine.new_nullable(nullable, None))
}

fn get<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    return if let Some(value) = arguments[0].get_gn::<Nullable>(engine).option {
        Ok(engine.new_constant(value))
    } else {
        Err(error_nullable())
    }
}

fn error_nullable() -> Error {
    Error::new_runtime("Cannot get the content of a null value.")
}
