use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::ReturnReference;
use crate::runtime::value::GcValue;

pub fn populate(engine: &mut Engine) {
    engine.set_constant_value("Option", engine.primitives.nullable);
}

pub fn create<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let Primitives { any, class, string, .. } = engine.primitives;
    let r#type = arguments[0];
    r#type.cast(class)?;
    let nullable = engine.new_class_value(Some("Option"), Some(any));
    engine.primitive_static(nullable, "some", [("value", r#type)], None, Some(nullable), &some);
    engine.primitive_static(nullable, "none", [], None, Some(nullable), &none);
    engine.primitive_method(nullable, "__fstr__", [], None, Some(string), &fstr);
    engine.primitive_method(nullable, "get", [], None, Some(r#type), &get);
    Ok(engine.new_constant(nullable))
}

fn get_type<'a>(engine: &mut Engine<'a>) -> GcValue<'a> {
    engine.scope().parent().unwrap().source().unwrap().data_class().constructor.unwrap().arguments[0]
}

fn fstr<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let this = arguments[0];
    let mut string = this.class.call_sstr(engine)?;
    if let Some(value) = this.data_nullable().option {
        string.push_str(".some(");
        string.push_str(&value.call_sstr(engine)?);
        string.push_str(")");
    } else {
        string.push_str(".none");
    }

    Ok(engine.new_string(string))
}

fn some<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let class = get_type(engine);
    let generic = engine.primitives.nullable;
    let nullable = generic.clone().data_generic_mut().call(engine, generic, &mut [class])?.read()?;
    Ok(engine.new_nullable(nullable, Some(arguments[0])))
}

fn none<'a>(engine: &mut Engine<'a>, _: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    let class = get_type(engine);
    let generic = engine.primitives.nullable;
    let nullable = generic.clone().data_generic_mut().call(engine, generic, &mut [class])?.read()?;
    Ok(engine.new_nullable(nullable, None))
}

fn get<'a>(engine: &mut Engine<'a>, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
    return if let Some(value) = arguments[0].data_nullable().option {
        Ok(engine.new_constant(value))
    } else {
        Err(error_nullable())
    }
}

fn error_nullable() -> Error {
    Error::new_runtime("Cannot get the content of a null value.")
}
