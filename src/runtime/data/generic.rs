use std::marker::PhantomData;

use crate::ast::nodes::ADef;
use crate::memory::Ref;
use crate::runtime::gc::{GcRef, GcTrace};
use crate::runtime::data::{Param, GcClass};
use crate::runtime::engine::Engine;
use crate::runtime::flow::ResValue;
use crate::runtime::scope::GcScope;

pub struct Generic<'a> {
    pub name: Box<str>,
    pub scope: GcScope<'a>,
    pub params: Box<[Param<'a>]>,
    pub body: GenericBody,
    phantom: PhantomData<&'a ()>,
}

pub enum GenericBody {
    Node(Ref<ADef>),
    Primitive(for<'a> fn(&mut Engine<'a>, &[GcClass<'a>]) -> ResValue<'a>),
}

impl<'a> Generic<'a> {
    pub fn new_node(name: &str, scope: GcScope<'a>, params: Box<[Param<'a>]>, node: Ref<ADef>) -> Self {
        Self {
            name: Box::from(name),
            scope,
            params,
            body: GenericBody::Node(node),
            phantom: PhantomData
        }
    }

    pub fn new_primitive(
        name: &str,
        scope: GcScope<'a>,
        params: Box<[Param<'a>]>,
        primitive: for<'b> fn(&mut Engine<'b>, &[GcClass<'b>]) -> ResValue<'b>
    ) -> Self {
        Self {
            name: Box::from(name),
            scope,
            params,
            body: GenericBody::Primitive(primitive),
            phantom: PhantomData
        }
    }
}

impl GcTrace for Generic<'_> {
    fn trace(&mut self) {
        self.scope.trace();
        for param in self.params.iter_mut() {
            param.trace();
        }
    }
}

pub type GcGeneric<'a> = GcRef<Generic<'a>>;
