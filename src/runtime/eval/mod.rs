mod def;
mod expr;
mod stmt;
mod r#type;
pub mod errors;

use crate::ast::Pos;
use crate::runtime::{Engine, Frame, Value};
use crate::runtime::data::{GcFunction, FunctionBody, GcGeneric, GenericBody, GcClass};
use crate::runtime::flow::{Flow, Jump, JumpKind, Res, ResValue};
use errors::*;

use std::iter::zip;

impl<'a> GcFunction<'a> {
    pub fn call(self, engine: &mut Engine<'a>, pos: Pos, args: &[Value<'a>]) -> ResValue<'a> {
        self.check_args(pos, args)?;
        self.check_rest(pos, args)?;
        engine.with_frame(Frame::new_function(pos, self), |engine| {
            match self.body {
                FunctionBody::Block(block) => {
                    self.write_args(engine, args)?;
                    self.write_rest(engine, pos, args)?;
                    let flow = block.eval(engine)?;
                    self.get_return_value(engine, flow)
                },
                FunctionBody::Primitive(primitive) => {
                    primitive(engine, args)
                },
            }
        })
    }

    fn check_args(self, pos: Pos, args: &[Value<'a>]) -> Res<()> {
        match self.rest {
            Some(_) => if self.params.len() > args.len() {
                return error_function_call_rest_arity(pos, self, args);
            },
            None => if self.params.len() != args.len() {
                return error_function_call_arity(pos, self, args);
            }
        }

        for (param, arg) in zip(self.params.iter(), args.iter().copied()) {
            if !arg.isa(param.r#type) {
                return error_type(pos, arg, param.r#type);
            }
        }

        Ok(())
    }

    fn check_rest(self, pos: Pos, args: &[Value<'a>]) -> Res<()> {
        let Some(rest) = self.rest.as_ref() else {
            return Ok(());
        };

        for arg in args[self.params.len()..].iter().copied() {
            if !arg.isa(rest.r#type) {
                return error_type(pos, arg, rest.r#type);
            }
        }

        Ok(())
    }

    fn write_args(self, engine: &mut Engine<'a>, args: &[Value<'a>]) -> Res<()> {
        for (param, arg) in zip(self.params.iter(), args.iter().copied()) {
            engine.write(&param.name, param.r#type, arg);
        }

        Ok(())
    }

    fn write_rest(self, engine: &mut Engine<'a>, pos: Pos, args: &[Value<'a>]) -> Res<()> {
        if let Some(rest) = self.rest.as_ref() {
            let elements = &args[self.params.len()..];
            let value = engine.new_list(elements);
            let class = engine.get_generic(pos, engine.env.list, Box::from([rest.r#type]))?.as_class();
            engine.write(&rest.name, class, value);
        }

        Ok(())
    }

    fn get_return_value(self, engine: &mut Engine<'a>, flow: Flow<'a>) -> ResValue<'a> {
        match flow {
            Flow::None(_) => {
                Ok(engine.new_void())
            },
            Flow::Jump(Jump { jump: JumpKind::Return, value, .. }) => {
                if let Some(value) = value {
                    Ok(value)
                } else {
                    Ok(engine.new_void())
                }
            },
            Flow::Jump(jump) => {
                return error_jump_loop(jump);
            },
        }
    }
}

impl<'a> GcGeneric<'a> {
    pub fn apply(self, engine: &mut Engine<'a>, pos: Pos, args: &[GcClass<'a>]) -> ResValue<'a> {
        self.check_args(pos, args)?;
        engine.with_frame(Frame::new_generic(pos, self, Box::from(args)), |engine| {
            match self.body {
                GenericBody::Node(node) => {
                    self.write_args(engine, args);
                    node.eval_def(engine)
                }
                GenericBody::Primitive(primitive) => {
                    primitive(engine, &args)
                },
            }
        })
    }

    fn check_args(self, pos: Pos, args: &[GcClass<'a>]) -> Res<()> {
        if self.params.len() != args.len() {
            return error_generic_apply_arity(pos, self, args);
        }

        for (param, arg) in zip(self.params.iter(), args.iter().copied()) {
            if !arg.isa(param.r#type) {
                return error_generic_type(pos, param.r#type, arg);
            }
        }

        Ok(())
    }

    fn write_args(self, engine: &mut Engine<'a>, args: &[GcClass<'a>]) {
        for (param, arg) in zip(self.params.iter(), args.iter().copied()) {
            let arg = engine.new_class_primitive(arg);
            engine.write(&param.name, engine.env.class, arg);
        }
    }
}

impl<'a> Value<'a> {
    pub fn call_method(self, engine: &mut Engine<'a>, pos: Pos, name: &str, args: &[Value<'a>]) -> ResValue<'a> {
        let args = std::iter::once(self)
            .chain(args.iter().copied())
            .collect::<Box<_>>();
        self.call_method_self(engine, pos, name, &args)
    }

    pub fn call_method_self(self, engine: &mut Engine<'a>, pos: Pos, name: &str, args: &[Value<'a>]) -> ResValue<'a> {
        let method = self.class.get_method(name).unwrap().as_function();
        method.call(engine, pos, &args)
    }
}
