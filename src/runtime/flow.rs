use crate::ast::Pos;
use crate::runtime::error::Error;
use crate::runtime::value::Value;

pub enum JumpKind {
    Continue,
    Break,
    Return,
}

pub struct Jump<'a> {
    pub pos: Pos,
    pub jump: JumpKind,
    pub value: Option<Value<'a>>,
}

impl<'a> Jump<'a> {
    pub fn new(pos: Pos, jump: JumpKind, value: Option<Value<'a>>) -> ResJump<'a> {
        Ok(Some(Self { pos, jump, value }))
    }

    pub fn some(jump: Jump<'a>) -> ResJump<'a> {
        Ok(Some(jump))
    }

    pub fn none() -> ResJump<'a> {
        Ok(None)
    }
}

pub type Flow<'a> = FlowT<'a, Value<'a>>;

pub enum FlowT<'a, T> {
    None(T),
    Jump(Jump<'a>),
}

impl<'a, T> FlowT<'a, T> {
    pub fn none(value: T) -> ResFlowT<'a, T> {
        Ok(Self::None(value))
    }

    pub fn jump(pos: Pos, jump: JumpKind, value: Option<Value<'a>>) -> ResFlowT<'a, T> {
        Ok(Self::Jump(Jump { pos, jump, value }))
    }
}

pub type Res<T> = Result<T, Error>;

pub type ResFlowT<'a, T> = Res<FlowT<'a, T>>;

pub type ResFlow<'a> = Res<Flow<'a>>;

pub type ResJump<'a> = Res<Option<Jump<'a>>>;

pub type ResValue<'a> = Res<Value<'a>>;

pub type ResVoid = Res<()>;
