use crate::parser::Code;
use crate::runtime::bis::data::{Param, GcClass};
use crate::runtime::bis::engine::Engine;
use crate::runtime::bis::env::Env;
use crate::runtime::bis::value::Value;
use crate::runtime::bis::flow::ResValue;
use crate::walker::nodes::{AExpression, AProgram};

use std::process;

pub struct PrimFunction<'a> {
    pub name: &'static str,
    pub params: Box<[Param<'a>]>,
    pub rest: Option<Param<'a>>,
    pub ret: GcClass<'a>,
    pub primitive: for<'b> fn(&mut Engine<'b>, &[Value<'b>]) -> ResValue<'b>,
}

impl<'a> PrimFunction<'a> {
    pub fn new<const N: usize>(
        name: &'static str,
        params: [(&'static str, GcClass<'a>); N],
        ret: GcClass<'a>,
        primitive: for<'b> fn(&mut Engine<'b>, &[Value<'b>]) -> ResValue<'b>
    ) -> Self {
        let params = params.iter()
            .map(|param| Param { name: Box::from(param.0), r#type: param.1 })
            .collect();

        Self { name, params, rest: None, ret, primitive }
    }

    pub fn new_rest<const N: usize>(
        name: &'static str,
        params: [(&'static str, GcClass<'a>); N],
        rest: (&'static str, GcClass<'a>),
        ret: GcClass<'a>,
        primitive: for<'b> fn(&mut Engine<'b>, &[Value<'b>]) -> ResValue<'b>
    ) -> Self {
        let params = params.iter()
            .map(|param| Param { name: Box::from(param.0), r#type: param.1 })
            .collect();

        let rest = Some(Param { name: Box::from(rest.0), r#type: rest.1 });
        Self { name, params, rest, ret, primitive }
    }
}

pub fn get_functions<'a>(env: &Env<'a>) -> [PrimFunction<'a>; 8] {
    [
        PrimFunction::new("assert",  [("value", env.bool)],  env.void, assert),
        PrimFunction::new("error",   [("value", env.any)],   env.void, error),
        PrimFunction::new("eval",    [("expr", env.string)], env.any,  eval),
        PrimFunction::new("exec",    [("prog", env.string)], env.void, exec),
        PrimFunction::new("exit",    [("code", env.int)],    env.any,  exit),
        PrimFunction::new("include", [("path", env.string)], env.void, include),
        PrimFunction::new("print",   [("value", env.any)],   env.void, print),
        PrimFunction::new("new",     [("class", env.class)], env.any,  new),
    ]
}

fn assert<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    if  args[0].as_bool() {
        panic!("TODO: panic assert");
    }

    Ok(engine.new_void())
}

fn error<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let string = &args[0].call_method(engine, "__str__", &[])?.as_string();
    writeln!(engine.io.err, "{}", string.as_ref()).unwrap();
    Ok(engine.new_void())
}

fn eval<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let code = Code::from_string::<AExpression>(&engine.grammar, engine.grammar.expression, args[0].as_string().as_ref());
    Ok(match engine.run(code) {
        Some(value) => value,
        None => engine.new_void(),
    })
}

fn exec<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let code = Code::from_string::<AProgram>(&engine.grammar, engine.grammar.program, args[0].as_string().as_ref());
    engine.run(code);
    Ok(engine.new_void())
}

fn exit<'a>(_: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    process::exit(args[0].as_int() as i32);
}

fn include<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    engine.with_frame(engine.get_scope(), |engine| {
        let code = Code::from_file::<AProgram>(&engine.grammar, engine.grammar.program, args[0].as_string().as_ref()).unwrap();
        engine.run(code);
    });

    Ok(engine.new_void())
}

fn new<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_object(args[0].as_class()))
}

fn print<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let value = args[0].call_method(engine, "__str__", &[])?.as_string();
    writeln!(engine.io.out, "{}", value.as_ref()).unwrap();
    Ok(engine.new_void())
}
