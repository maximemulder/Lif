use crate::runtime::data::GcClass;
use crate::runtime::engine::Engine;
use crate::runtime::env::Env;
use crate::runtime::flow::{Res, ResValue};
use crate::runtime::primitive::functions::PrimFunction;
use crate::runtime::value::Value;

use std::mem::size_of;

pub struct PrimMethod<'a> {
    pub class: GcClass<'a>,
    pub methods: Box<[PrimFunction<'a>]>,
}

impl<'a> PrimMethod<'a> {
    pub fn new<const N: usize>(
        class: GcClass<'a>,
        methods: [PrimFunction<'a>; N]
    ) -> Self {
        Self { class, methods: Box::new(methods) }
    }
}

pub fn get_methods<'a>(env: &Env<'a>) -> [PrimMethod<'a>; 9] {
    [
        PrimMethod::new(env.any, [
            PrimFunction::new("__str__", [], env.any,  any_str),
            PrimFunction::new("__cn__",  [("other", env.string)], env.any,  any_cn),
            PrimFunction::new("__ne__",  [("other", env.any)],    env.bool, any_ne),
            PrimFunction::new("__gt__",  [("other", env.any)],    env.bool, any_gt),
            PrimFunction::new("__le__",  [("other", env.any)],    env.bool, any_le),
            PrimFunction::new("__ge__",  [("other", env.any)],    env.bool, any_ge),
        ]),
        PrimMethod::new(env.bool, [
            PrimFunction::new("__str__", [],                   env.string, bool_str),
            PrimFunction::new("__eq__",  [("other", env.any)], env.bool,   bool_eq),
            PrimFunction::new("__not__", [],                   env.bool,   bool_not),
        ]),
        PrimMethod::new(env.class, [
            PrimFunction::new("__cl__",  [("arguments", env.any)], env.any, class_cl),
        ]),
        PrimMethod::new(env.float, [
            PrimFunction::new("__str__", [],                      env.string, float_str),
            PrimFunction::new("__eq__",  [("other", env.any)],    env.bool,   float_eq),
            PrimFunction::new("__lt__",  [("other", env.float)],  env.bool,   float_lt),
            PrimFunction::new("__pos__", [],                      env.float,  float_pos),
            PrimFunction::new("__neg__", [],                      env.float,  float_neg),
            PrimFunction::new("__add__", [("other", env.float)],  env.float,  float_add),
            PrimFunction::new("__sub__", [("other", env.float)],  env.float,  float_sub),
            PrimFunction::new("__mul__", [("other", env.float)],  env.float,  float_mul),
            PrimFunction::new("__div__", [("other", env.float)],  env.float,  float_div),
            PrimFunction::new("__rem__", [("other", env.float)],  env.float,  float_rem),
        ]),
        PrimMethod::new(env.function, [
            PrimFunction::new("__cl__",  [("arguments", env.any)], env.any, function_cl),
        ]),
        PrimMethod::new(env.int, [
            PrimFunction::new("__str__",  [],                   env.string, int_str),
            PrimFunction::new("__eq__",   [("other", env.any)], env.bool,   int_eq),
            PrimFunction::new("__lt__",   [("other", env.int)], env.bool,   int_lt),
            PrimFunction::new("__pos__",  [],                   env.int,    int_pos),
            PrimFunction::new("__neg__",  [],                   env.int,    int_neg),
            PrimFunction::new("__add__",  [("other", env.int)], env.int,    int_add),
            PrimFunction::new("__sub__",  [("other", env.int)], env.int,    int_sub),
            PrimFunction::new("__mul__",  [("other", env.int)], env.int,    int_mul),
            PrimFunction::new("__div__",  [("other", env.int)], env.int,    int_div),
            PrimFunction::new("__rem__",  [("other", env.int)], env.int,    int_rem),
            PrimFunction::new("__bnot__", [],                   env.int,    int_bnot),
            PrimFunction::new("__band__", [("other", env.int)], env.int,    int_band),
            PrimFunction::new("__bor__",  [("other", env.int)], env.int,    int_bor),
            PrimFunction::new("__bxor__", [("other", env.int)], env.int,    int_bxor),
            PrimFunction::new("__bls__",  [("other", env.int)], env.int,    int_bls),
            PrimFunction::new("__brs__",  [("other", env.int)], env.int,    int_brs),
            PrimFunction::new("__bcls__", [("other", env.int)], env.int,    int_bcls),
            PrimFunction::new("__bcrs__", [("other", env.int)], env.int,    int_bcrs),
        ]),
        PrimMethod::new(env.method, [
            PrimFunction::new("__cl__",  [("arguments", env.any)], env.any, method_cl),
        ]),
        PrimMethod::new(env.object, [
            PrimFunction::new("__str__", [],                       env.string, object_str),
            PrimFunction::new("__cn__",  [("member", env.string)], env.any,    object_cn),
        ]),
        PrimMethod::new(env.string, [
            PrimFunction::new("__str__", [],                      env.string, string_str),
            PrimFunction::new("__eq__",  [("other", env.any)],    env.string, string_eq),
            PrimFunction::new("__add__", [("other", env.any)], env.string, string_add),
        ]),
    ]
}

pub fn get_list_methods<'a>(env: &Env<'a>, _: GcClass<'a>, _args: &[GcClass<'a>]) -> [PrimFunction<'a>; 6] {
    [
        PrimFunction::new("__str__", [], env.string, list_str),
        PrimFunction::new("__cl__",  [("args", env.any)], env.int, list_cl),
        PrimFunction::new_rest("insert",  [("index", env.int)], ("elems", env.any), env.void, list_insert),
        PrimFunction::new_rest("prepend", [], ("elems", env.any), env.void, list_prepend),
        PrimFunction::new_rest("append", [], ("elems", env.any), env.void, list_append),
        PrimFunction::new("remove",  [("index", env.int)], env.void, list_remove),
    ]
}

fn any_str<'a>(engine: &mut Engine<'a>, _: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_string("[OBJECT]"))
}

fn any_cn<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let receiver = args[0];
    let method = receiver.class.get_method(args[1].as_string().as_ref()).unwrap();
    Ok(engine.new_method(receiver, method))
}

fn any_ne<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let result = args[0].call_method_self(engine, engine.frame().pos(), "__eq__", args)?.as_bool();
    Ok(engine.new_bool(!result))
}

fn any_gt<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let left  = args[0].call_method_self(engine, engine.frame().pos(), "__lt__", args)?;
    let right = args[0].call_method_self(engine, engine.frame().pos(), "__eq__", args)?;
    Ok(engine.new_bool(!left.as_bool() && !right.as_bool()))
}


fn any_le<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let left  = args[0].call_method_self(engine, engine.frame().pos(), "__lt__", args)?;
    let right = args[0].call_method_self(engine, engine.frame().pos(), "__eq__", args)?;
    Ok(engine.new_bool(left.as_bool() || right.as_bool()))
}
fn any_ge<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let result = args[0].call_method_self(engine, engine.frame().pos(), "__lt__", args)?;
    Ok(engine.new_bool(!result.as_bool()))
}

fn bool_str<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_string(&args[0].as_bool().to_string()))
}

fn bool_eq<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_bool(if args[1].isa(engine.env.bool) {
        args[0].as_bool() == args[1].as_bool()
    } else {
        false
    }))
}

fn bool_not<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_bool(!args[0].as_bool()))
}

fn class_cl<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let class = args[0].as_class();
    let args = args[1].as_list();
    if let Some(init) = class.get_static("__init__") {
        return init.as_function().call(engine, engine.frame().pos(), &args.values());
    }

    todo!("ERROR");
}

fn function_cl<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    args[0].as_function().call(engine, engine.frame().pos(), &args[1].as_list().values())
}

fn float_str<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_string(&args[0].as_float().to_string()))
}

fn float_eq<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_bool(if args[1].isa(engine.env.float) {
        args[0].as_float() == args[1].as_float()
    } else {
        false
    }))
}

fn float_lt<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_bool(args[0].as_float() < args[1].as_float()))
}

fn float_pos<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_float(args[0].as_float()))
}

fn float_neg<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_float(-args[0].as_float()))
}

fn float_add<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_float(args[0].as_float() + args[1].as_float()))
}

fn float_sub<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_float(args[0].as_float() - args[1].as_float()))
}

fn float_mul<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_float(args[0].as_float() * args[1].as_float()))
}

fn float_div<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_float(args[0].as_float() / args[1].as_float()))
}

fn float_rem<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_float(args[0].as_float() % args[1].as_float()))
}

fn int_str<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_string(&args[0].as_int().to_string()))
}

fn int_eq<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_bool(if args[1].isa(engine.env.int) {
        args[0].as_int() == args[1].as_int()
    } else {
        false
    }))
}

fn int_lt<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_bool(args[0].as_int() < args[1].as_int()))
}

fn int_pos<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_int(args[0].as_int()))
}

fn int_neg<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_int(-args[0].as_int()))
}

fn int_add<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_int(args[0].as_int() + args[1].as_int()))
}

fn int_sub<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_int(args[0].as_int() - args[1].as_int()))
}

fn int_mul<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_int(args[0].as_int() * args[1].as_int()))
}

fn int_div<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_int(args[0].as_int() / args[1].as_int()))
}

fn int_rem<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_int(args[0].as_int() % args[1].as_int()))
}

fn int_bnot<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_int(!args[0].as_int()))
}

fn int_band<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_int(args[0].as_int() & args[1].as_int()))
}

fn int_bor<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_int(args[0].as_int() | args[1].as_int()))
}

fn int_bxor<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_int(args[0].as_int() ^ args[1].as_int()))
}

fn int_bls<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_int(args[0].as_int() << args[1].as_int()))
}

fn int_brs<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_int(args[0].as_int() >> args[1].as_int()))
}

fn int_bcls<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let x = args[0].as_int();
    let y = args[1].as_int();
    Ok(engine.new_int((x << y) | (x >> (-y & size_of::<i64>() as i64))))
}

fn int_bcrs<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let x = args[0].as_int();
    let y = args[1].as_int();
    Ok(engine.new_int((x >> y) | (x << (-y & size_of::<i64>() as i64))))
}

fn method_cl<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let method = args[0].as_method();
    let args = std::iter::once(method.receiver)
        .chain(args[1].as_list().values().iter().copied())
        .collect::<Box<_>>();

    method.function.as_function().call(engine, engine.frame().pos(), &args)
}

fn list_str<'a>(engine: &mut Engine<'a>, _: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_string("[LIST]"))
}

fn list_cl<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let index = args[1].as_list().values()[0].as_int() as usize;
    let r#ref = args[0].as_list().get_ref(index);
    Ok(engine.new_ref(r#ref))
}

fn list_insert<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let index = args[1].as_int() as usize;
    args[0].as_list().insert(engine.env.any, index, args[2]);
    Ok(engine.new_void())
}

fn list_append<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    for index in 1 .. args.len() {
        args[0].as_list().append(engine.env.any, args[index]);
    }

    Ok(engine.new_void())
}

fn list_prepend<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    for index in 1 .. args.len() {
        args[0].as_list().insert(engine.env.any, index - 1, args[index]);
    }

    Ok(engine.new_void())
}

fn list_remove<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let index = args[1].as_int() as usize;
    args[0].as_list().remove(index);
    Ok(engine.new_void())
}

fn object_str<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let mut string = "{".to_string();
    string.push_str(&args[0].as_object().attributes.iter()
        .filter_map(|(name, attribute)| attribute.content().map(|attribute| (name, attribute)))
        .map(|(name, attribute)| Ok(format!("{}: {}", &name, &attribute.call_method(engine, engine.frame().pos(), "__str__", &[])?.as_string().as_ref())))
        .collect::<Res<Box<[_]>>>()?
        .join(", ")
    );

    string.push('}');
    Ok(engine.new_string(&string))
}

fn object_cn<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let receiver = args[0];
    let name = args[1].as_string();
    if let Some(method) = receiver.class.get_method(name.as_ref()) {
        return Ok(engine.new_method(receiver, method));
    }

    let mut object = receiver.as_object();
    let r#ref = object.get_attr(name.as_ref(), engine.env.any);
    Ok(engine.new_ref(r#ref))
}

fn string_str<'a>(_: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(args[0])
}

fn string_eq<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_bool(if args[1].isa(engine.env.string) {
        args[0].as_string() == args[1].as_string()
    } else {
        false
    }))
}

fn string_add<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let left = args[0].as_string();
    let right = args[1].call_method(engine, engine.frame().pos(), "__str__", &[])?.as_string();
    Ok(engine.new_string(&format!("{}{}", left.as_ref(), right.as_ref())))
}
