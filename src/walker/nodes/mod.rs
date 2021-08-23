#![allow(dead_code)]

#![macro_use]

macro_rules! jump {
    ( $jump:expr ) => {{
        let jump = $jump;
        if jump.is_some() {
            return Ok(jump);
        }
    }
}}

macro_rules! jump_flow {
    ( $jump:expr ) => {{
        if let Some(jump) = $jump {
            return Flow::jump(jump);
        }
    }
}}

macro_rules! flow {
    ( $flow:expr ) => {{
        let flow = $flow;
        if let Flow::Reference(reference) = flow {
            reference
        } else {
            return Ok(flow);
        }
    }
}}

macro_rules! flow_loop {
    ( $flow:expr ) => {{
        use crate::runtime::r#return::{ Flow, Jump, JumpType };
        let flow = $flow;
        match $flow {
            Flow::Reference(reference) => reference,
            Flow::Jump(Jump { r#type: JumpType::Return, .. }) => return Ok(flow),
            Flow::Jump(Jump { reference, .. }) => reference,
        }
    }
}}

mod program;
mod statements;
mod statement;
mod assignment;
mod preop;
mod binop;
mod chain;
mod sequence;
mod declaration;
mod r#type;
mod executable;
mod jump;
mod r#let;
mod expression;
mod structures;
mod controls;
mod literals;

pub use program::AProgram;
pub use statements::AStatements;
pub use statement::*;
pub use assignment::AAssignment;
pub use preop::APreop;
pub use binop::ABinop;
pub use chain::AChain;
pub use sequence::ASequence;
pub use declaration::ADeclaration;
pub use r#type::AType;
pub use executable::AExecutableTrait;
pub use jump::AJump;
pub use expression::*;
pub use r#let::ALet;
pub use structures::*;
pub use controls::*;
pub use literals::*;
