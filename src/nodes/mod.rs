#![allow(dead_code)]
mod binary;
mod expression;
mod r#if;
mod statement;
mod statements;
mod unary;

pub trait Node {
	fn execute(&self);
}
