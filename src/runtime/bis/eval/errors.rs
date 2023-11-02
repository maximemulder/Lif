use crate::ast::Pos;
use crate::runtime::bis::Value;
use crate::runtime::bis::data::{GcClass, GcFunction, GcGeneric};
use crate::runtime::bis::error::Error;
use crate::runtime::bis::flow::{Jump, Res};

pub fn error_generic_apply_arity<'a, T>(pos: Pos, generic: GcGeneric<'a>, args: &[GcClass<'a>]) -> Res<T> {
    Error::new(pos, format!("`{}` has {} generic parameters but {} arguments were supplied", generic.name, generic.params.len(), args.len()))
}

pub fn error_generic_type<T>(pos: Pos, param: GcClass<'_>, arg: GcClass<'_>) -> Res<T> {
    Error::new(pos, format!("expected subclass of `{}` but found class `{}`", arg.name, param.name))
}

pub fn error_function_call_arity<T>(pos: Pos, function: GcFunction<'_>, args: &[Value<'_>]) -> Res<T> {
    Error::new(pos, format!("function `{}` expects {} arguments but {} were supplied", function.name, function.params.len(), args.len()))
}

pub fn error_function_call_rest_arity<T>(pos: Pos, function: GcFunction<'_>, args: &[Value<'_>]) -> Res<T> {
    Error::new(pos, format!("function `{}` expects at least {} arguments but {} were supplied", function.name, function.params.len(), args.len()))
}

pub fn error_type<T>(pos: Pos, value: Value<'_>, r#type: GcClass<'_>) -> Res<T> {
    Error::new(pos, format!("expected value of type `{}` but found value of type `{}`", r#type.name, value.class.name))
}

pub fn error_jump<T>(jump: Jump) -> Res<T> {
    Error::new(jump.pos, format!("invalid jump"))
}

pub fn error_jump_loop<T>(jump: Jump) -> Res<T> {
    Error::new(jump.pos, format!("invalid loop jump"))
}

pub fn error_undeclared<T>(pos: Pos, name: &str) -> Res<T> {
    Error::new(pos, format!("undeclared variable `{name}`"))
}

pub fn error_undefined<T>(pos: Pos) -> Res<T> {
    Error::new(pos, format!("undefined variable"))
}
