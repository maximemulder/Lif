use crate::runtime::bis::error::Error;
use crate::runtime::bis::value::Value;

pub enum JumpKind {
    Continue,
    Break,
    Return,
}

pub struct Jump<'a> {
    pub jump: JumpKind,
    pub value: Option<Value<'a>>,
}

impl<'a> Jump<'a> {
    pub fn new(jump: JumpKind, value: Option<Value<'a>>) -> ResJump<'a> {
        Ok(Some(Self { jump, value }))
    }

    pub fn some(jump: Jump<'a>) -> ResJump<'a> {
        Ok(Some(jump))
    }

    pub fn none() -> ResJump<'a> {
        Ok(None)
    }
}

pub enum Flow<'a> {
    Value(Value<'a>),
    Jump(Jump<'a>),
}

impl<'a> Flow<'a> {
    pub fn value(value: Value<'a>) -> ResFlow<'a> {
        Ok(Self::Value(value))
    }

    pub fn jump(jump: JumpKind, value: Option<Value<'a>>) -> ResFlow<'a> {
        Ok(Self::Jump(Jump { jump, value }))
    }
}

pub type Res<T> = Result<T, Error>;

pub type ResFlow<'a> = Res<Flow<'a>>;

pub type ResJump<'a> = Res<Option<Jump<'a>>>;

pub type ResValue<'a> = Res<Value<'a>>;
