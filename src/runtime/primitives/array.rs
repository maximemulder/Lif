use crate::runtime::data::{ Array, Class };
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcRef;
use crate::runtime::primitives::Primitives;
use crate::runtime::r#return::{ Return, ReturnReference };
use crate::runtime::value::Value;

pub fn populate(engine: &mut Engine) {
    engine.populate_generic("Array", engine.primitives.array);
}

pub fn create<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let Primitives { any, integer, string, .. } = engine.primitives;
    let r#type = arguments[0].get_cast_class(engine)?;
    let array = engine.primitive_class("Array", Some(any));
    let array_any = if r#type == any {
        array
    } else {
        engine.primitives.array_any
    };

    engine.primitive_static(array, "__init__", [], Some(("elements", array)), None, &init);
    engine.primitive_method(array, "__fstr__", [], None, Some(string), &fstr);
    engine.primitive_method(array, "append", [], Some(("elements", array)), None, &append);
    engine.primitive_method(array, "prepend", [], Some(("elements", array)), None, &prepend);
    engine.primitive_method(array, "insert", [("index", integer), ("element", r#type)], None, None, &insert);
    engine.primitive_method(array, "remove", [("index", integer)], None, None, &remove);
    engine.primitive_method(array, "__cl__", [("arguments", array_any)], None, Some(r#type), &access);
    Ok(engine.new_constant(Value::primitive_gc(engine, array)))
}

fn get_type<'a>(engine: &Engine<'a>) -> GcRef<Class<'a>> {
    engine.scope().parent().unwrap().source().unwrap().get_gc::<Class>(engine).constructor().unwrap().arguments[0].get_gc::<Class>(engine)
}

fn init<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let r#type = get_type(engine);
    let elements = arguments.iter()
        .copied()
        .map(|argument| engine.new_variable(Some(argument), r#type))
        .collect();

    Ok(engine.new_array(engine.scope().parent().unwrap().source().unwrap().get_gc::<Class>(engine), elements))
}

fn fstr<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let this = arguments[0];
    let mut string = Value::primitive_gc(engine, this.class).call_sstr(engine)?;
    string.push_str("(");
    string.push_str(&this.get_gn::<Array>(engine).elements().iter()
        .map(|element| element.read()?.call_sstr(engine))
        .collect::<Return<Box<[_]>>>()?
        .join(", ")
    );

    string.push_str(")");
    Ok(engine.new_string(string))
}

fn append<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    for index in 1 .. arguments.len() {
        let r#type = get_type(engine);
        let reference = engine.new_variable(Some(arguments[index]), r#type);
        arguments[0].get_gn::<Array>(engine).append(reference);
    }

    Ok(engine.undefined())
}

fn prepend<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    for index in 1 .. arguments.len() {
        let r#type = get_type(engine);
        let reference = engine.new_variable(Some(arguments[index]), r#type);
        arguments[0].get_gn::<Array>(engine).insert(index - 1, reference);
    }

    Ok(engine.undefined())
}

fn insert<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let index = arguments[1].get::<isize>(engine) as usize;
    let r#type = get_type(engine);
    let reference = engine.new_variable(Some(arguments[2]), r#type);
    arguments[0].get_gn::<Array>(engine).insert(index, reference);
    Ok(engine.undefined())
}

fn remove<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    let index = arguments[1].get::<isize>(engine) as usize;
    arguments[0].get_gn::<Array>(engine).remove(index);
    Ok(engine.undefined())
}

fn access<'a>(engine: &mut Engine<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
    Ok(arguments[0].get_gn::<Array>(engine).get(arguments[1].get_gn::<Array>(engine).get(0).read()?.get::<isize>(engine) as usize))
}
