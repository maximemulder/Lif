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
mod r#if;
mod r#loop;
mod r#while;
mod do_while;
mod for_in;
mod assignment;
mod preop;
mod binop;
mod chain;
mod sequence;
mod declaration;
mod generic;
mod structure;
mod class;
mod function;
mod block;
mod group;
mod r#true;
mod r#false;
mod integer;
mod float;
mod string;
mod identifier;
mod r#continue;
mod r#break;
mod r#return;

pub use program::Program;
pub use statements::Statements;
pub use statement::Statement;
pub use r#if::If;
pub use r#loop::Loop;
pub use r#while::While;
pub use do_while::DoWhile;
pub use for_in::ForIn;
pub use assignment::Assignment;
pub use preop::Preop;
pub use binop::Binop;
pub use chain::Chain;
pub use sequence::Sequence;
pub use declaration::Declaration;
pub use generic::Generic;
pub use structure::Structure;
pub use class::Class;
pub use function::Function;
pub use block::Block;
pub use group::Group;
pub use r#true::True;
pub use r#false::False;
pub use integer::Integer;
pub use float::Float;
pub use string::String;
pub use identifier::Identifier;
pub use r#continue::Continue;
pub use r#break::Break;
pub use r#return::Return;
