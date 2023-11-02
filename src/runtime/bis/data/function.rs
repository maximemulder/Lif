use crate::ast::nodes::ABlock;
use crate::memory::Ref;
use crate::runtime::gc::{GcRef, GcTrace};
use crate::runtime::bis::data::{Param, GcClass};
use crate::runtime::bis::engine::Engine;
use crate::runtime::bis::flow::ResValue;
use crate::runtime::bis::scope::GcScope;
use crate::runtime::bis::value::Value;

pub struct Function<'a> {
    pub name: Box<str>,
    pub scope: GcScope<'a>,
    pub params: Box<[Param<'a>]>,
    pub rest: Option<Param<'a>>,
    pub ret: GcClass<'a>,
    pub body: FunctionBody,
}

pub enum FunctionBody {
    Block(Ref<ABlock>),
    Primitive(for<'a> fn(&mut Engine<'a>, &[Value<'a>]) -> ResValue<'a>),
}

impl<'a> Function<'a> {
    pub fn new_block(
        name: &str,
        scope: GcScope<'a>,
        params: Box<[Param<'a>]>,
        rest: Option<Param<'a>>,
        ret: GcClass<'a>,
        block: Ref<ABlock>
    ) -> Self {
        Self {
            name: Box::from(name),
            scope,
            params,
            rest,
            ret,
            body: FunctionBody::Block(block)
        }
    }

    pub fn new_primitive(
        name: &str,
        scope: GcScope<'a>,
        params: Box<[Param<'a>]>,
        rest: Option<Param<'a>>,
        ret: GcClass<'a>,
        primitive: for<'b> fn(&mut Engine<'b>, &[Value<'b>]) -> ResValue<'b>
    ) -> Self {
        Self {
            name: Box::from(name),
            scope,
            params,
            rest,
            ret,
            body: FunctionBody::Primitive(primitive)
        }
    }
}

impl GcTrace for Function<'_> {
    fn trace(&mut self) {
        self.scope.trace();
        for param in self.params.iter_mut() {
            param.trace();
        }

        if let Some(rest) = self.rest.as_mut() {
            rest.trace();
        }

        self.ret.trace();
    }
}

pub type GcFunction<'a> = GcRef<Function<'a>>;
