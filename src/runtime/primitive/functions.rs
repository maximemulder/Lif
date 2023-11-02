use crate::parser::Code;
use crate::runtime::data::{Param, GcClass, Function};
use crate::runtime::engine::Engine;
use crate::runtime::env::Env;
use crate::runtime::value::Value;
use crate::runtime::flow::ResValue;

use std::{fs, process};

pub struct PrimFunction<'a> {
    pub name: &'static str,
    pub params: Box<[(&'static str, GcClass<'a>)]>,
    pub rest: Option<(&'static str, GcClass<'a>)>,
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
        let params = Box::from(params);
        Self { name, params, rest: None, ret, primitive }
    }

    pub fn new_rest<const N: usize>(
        name: &'static str,
        params: [(&'static str, GcClass<'a>); N],
        rest: (&'static str, GcClass<'a>),
        ret: GcClass<'a>,
        primitive: for<'b> fn(&mut Engine<'b>, &[Value<'b>]) -> ResValue<'b>
    ) -> Self {
        let params = Box::from(params);
        Self { name, params, rest: Some(rest), ret, primitive }
    }

    pub fn to_function(&self, engine: &mut Engine<'a>) -> Function<'a> {
        Function::new_primitive(
            self.name,
            engine.scope,
            self.get_params(),
            self.get_rest(),
            self.ret,
            self.primitive
        )
    }

    pub fn to_method(&self, engine: &mut Engine<'a>, class: GcClass<'a>) -> Function<'a> {
        let mut params = Vec::new();
        params.push(Param::new("self", class));
        params.append(&mut self.get_params().into_vec());
        Function::new_primitive(
            self.name,
            engine.scope,
            params.into_boxed_slice(),
            self.get_rest(),
            self.ret,
            self.primitive
        )
    }

    fn get_params(&self) -> Box<[Param<'a>]> {
        self.params.iter()
            .map(|param| Param::new(param.0, param.1))
            .collect()
    }

    fn get_rest(&self) -> Option<Param<'a>> {
        self.rest.map(|rest| {
            Param::new(rest.0, rest.1)
        })
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
    if args[0].as_bool() {
        panic!("TODO: panic assert");
    }

    Ok(engine.new_void())
}

fn error<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let string = &args[0].call_method(engine, engine.frame().pos(), "__str__", &[])?.as_string();
    writeln!(engine.io.err, "{}", string.as_ref()).unwrap();
    Ok(engine.new_void())
}

fn eval<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let code = Code::new(&engine.grammar, engine.grammar.program, None, Box::from(args[0].as_string().as_ref()));
    Ok(match engine.run(code) {
        Some(value) => value,
        None => engine.new_void(),
    })
}

fn exec<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let code = Code::new(&engine.grammar, engine.grammar.program, None, Box::from(args[0].as_string().as_ref()));
    engine.run(code);
    Ok(engine.new_void())
}

fn exit<'a>(_: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    process::exit(args[0].as_int() as i32);
}

fn include<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let name = args[0].as_string();
    let text = fs::read_to_string(name.as_ref()).unwrap().into_boxed_str();
    let code = Code::new(&engine.grammar, engine.grammar.program, Some(name.as_ref()), text);
    engine.run(code);
    Ok(engine.new_void())
}

fn new<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    Ok(engine.new_object(args[0].as_class()))
}

fn print<'a>(engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
    let value = args[0].call_method(engine, engine.frame().pos(), "__str__", &[])?.as_string();
    writeln!(engine.io.out, "{}", value.as_ref()).unwrap();
    Ok(engine.new_void())
}
