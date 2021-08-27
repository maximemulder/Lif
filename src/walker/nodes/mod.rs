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

mod definitions;
mod expressions;
mod literals;
mod structures;
mod program;
mod statements;
mod statement;
mod definition;
mod structure;
mod expression;
mod generics;
mod parameters;
mod declaration;
mod r#type;

pub use definitions::*;
pub use expressions::*;
pub use literals::*;
pub use structures::*;
pub use program::AProgram;
pub use statements::AStatements;
pub use statement::AStatement;
pub use definition::ADefinition;
pub use structure::AStructure;
pub use expression::AExpression;
pub use generics::AGenerics;
pub use parameters::AParameters;
pub use declaration::ADeclaration;
pub use r#type::AType;
