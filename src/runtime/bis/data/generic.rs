use std::marker::PhantomData;

use crate::ast::nodes::ADef;
use crate::memory::Ref;
use crate::runtime::gc::{GcRef, GcTrace};
use crate::runtime::bis::data::function::Param;

pub struct Generic<'a> {
    pub params: Box<[Param<'a>]>,
    pub node: Ref<ADef>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Generic<'a> {
    pub fn new(params: Box<[Param<'a>]>, node: Ref<ADef>) -> Self {
        Self { params, node, phantom: PhantomData }
    }
}

impl GcTrace for Generic<'_> {
    fn trace(&mut self) {
        for param in self.params.iter_mut() {
            param.r#type.trace();
        }
    }
}

pub type GcGeneric<'a> = GcRef<Generic<'a>>;
