use crate::ast::Pos;
use crate::runtime::gc::GcTrace;
use crate::runtime::bis::scope::GcScope;

use super::data::{GcClass, GcFunction, GcGeneric};

pub struct Frame<'a> {
    pos: Pos,
    body: FrameBody<'a>,
}

enum FrameBody<'a> {
    Main(FrameMain<'a>),
    Function(FrameFunction<'a>),
    Generic(FrameGeneric<'a>),
}

pub struct FrameMain<'a> {
    scope: GcScope<'a>,
}

pub struct FrameFunction<'a> {
    function: GcFunction<'a>,
}

pub struct FrameGeneric<'a> {
    generic: GcGeneric<'a>,
    args: Box<[GcClass<'a>]>,
}

impl<'a> Frame<'a> {
    pub fn new_main(pos: Pos, scope: GcScope<'a>) -> Self {
        Self {
            pos,
            body: FrameBody::Main(FrameMain { scope }),
        }
    }

    pub fn new_function(pos: Pos, function: GcFunction<'a>) -> Self {
        Self {
            pos,
            body: FrameBody::Function(FrameFunction { function }),
        }
    }

    pub fn new_generic(pos: Pos, generic: GcGeneric<'a>, args: Box<[GcClass<'a>]>) -> Self {
        Self {
            pos,
            body: FrameBody::Generic(FrameGeneric { generic, args }),
        }
    }

    pub fn name(&self) -> &str {
        match &self.body {
            FrameBody::Main     (_)    => "main",
            FrameBody::Function (body) => body.function.name.as_ref(),
            FrameBody::Generic  (body) => body.generic.name.as_ref(),
        }
    }

    pub fn pos(&self) -> Pos {
        self.pos
    }

    pub fn scope(&self) -> GcScope<'a> {
        match &self.body {
            FrameBody::Main     (body) => body.scope,
            FrameBody::Function (body) => body.function.scope,
            FrameBody::Generic  (body) => body.generic.scope,
        }
    }

    pub fn generics(&self) -> &[GcClass<'a>] {
        match &self.body {
            FrameBody::Generic(body) => body.args.as_ref(),
            _ => &[],
        }
    }
}

impl GcTrace for Frame<'_> {
    fn trace(&mut self) {
        match &mut self.body {
            FrameBody::Main(body) => {
                body.scope.trace();
            },
            FrameBody::Function(body) => {
                body.function.trace();
            },
            FrameBody::Generic(body) => {
                body.generic.trace();
                for arg in body.args.iter_mut() {
                    arg.trace();
                }
            },
        }
    }
}
