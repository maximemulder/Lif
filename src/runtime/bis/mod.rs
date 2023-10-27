pub mod data;
pub mod engine;
pub mod env;
pub mod error;
pub mod eval;
pub mod flow;
pub mod primitive;
pub mod scope;
pub mod value;

pub use value::Value;

pub enum ValueRef<'a> {
    Value(Value<'a>),
    Undeclared,
    Undefined,
}
