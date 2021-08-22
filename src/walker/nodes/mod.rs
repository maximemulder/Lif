#![allow(dead_code)]

#![macro_use]

macro_rules! get {
    ( $flow:expr ) => {{
        use crate::runtime::r#return::Jump;

        let flow = $flow;
        if flow.jump == Jump::None {
            flow.reference
        } else {
            return Ok(flow);
        }
    }
}}

macro_rules! get_loop {
    ( $flow:expr ) => {{
        use crate::runtime::r#return::Jump;

        if $flow.jump != Jump::Return {
            $flow.reference
        } else {
            return Ok($flow);
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
mod structures;
mod controls;
mod jumps;
mod literals;
mod expression;
mod r#let;

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
pub use structures::*;
pub use controls::*;
pub use jumps::*;
pub use literals::*;
pub use expression::*;
pub use r#let::ALet;
