mod def;
mod expr;
mod stmt;
mod write;

use crate::runtime::bis::data::{Function, Generic, FunctionBody};
use crate::runtime::bis::engine::Engine;
use crate::runtime::bis::flow::{Flow, Jump, JumpKind, ResValue};
use crate::runtime::bis::scope::Scope;
use crate::runtime::bis::value::Value;

impl<'a> Function<'a> {
    pub fn call(&self, engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
        match self.body {
            FunctionBody::Block(block) => {
                let frame = engine.alloc(Scope::new(Some(self.scope)));
                engine.with_frame(frame, |engine| {
                    for (param, arg) in std::iter::zip(self.params.iter(), args.iter().copied()) {
                        engine.write(&param.name, arg);
                    }

                    match block.eval(engine)? {
                        Flow::Value(_) => {
                            Ok(engine.new_void())
                        },
                        Flow::Jump(Jump { jump: JumpKind::Return, value }) => {
                            if let Some(value) = value {
                                Ok(value)
                            } else {
                                Ok(engine.new_void())
                            }
                        },
                        Flow::Jump(Jump { jump: _, .. }) => panic!("TODO ERROR"),
                    }
                })
            },
            FunctionBody::Primitive(primitive) => primitive(engine, args),
        }
    }
}

impl<'a> Generic<'a> {
    pub fn apply(&self, engine: &mut Engine<'a>, args: &[Value<'a>]) -> ResValue<'a> {
        engine.with_scope(|engine| {
            for (param, arg) in self.params.iter().zip(args.iter().copied()) {
                if arg.isa(param.r#type) {
                    engine.write(&param.name, arg);
                } else {
                    todo!("ERROR");
                }
            }

            self.node.eval_def(engine)
        })
    }
}

impl<'a> Value<'a> {
    pub fn call_method(self, engine: &mut Engine<'a>, name: &str, args: &[Value<'a>]) -> ResValue<'a> {
        let args = std::iter::once(self)
            .chain(args.iter().copied())
            .collect::<Box<_>>();
        self.call_method_self(engine, name, &args)
    }

    pub fn call_method_self(self, engine: &mut Engine<'a>, name: &str, args: &[Value<'a>]) -> ResValue<'a> {
        let method = self.class.get_method(name).unwrap().as_function();
        method.call(engine, &args)
    }
}
