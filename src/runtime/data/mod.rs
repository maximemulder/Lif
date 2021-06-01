mod array;
mod class;
mod function;
mod generic;
mod method;
mod nullable;
mod object;

pub use array::Array;
pub use class::Class;
pub use function::{ Function, FunctionImplementation, FunctionCode, FunctionPrimitive };
pub use generic::{ Generic, GenericImplementation, GenericCode, GenericPrimitive };
pub use method::Method;
pub use nullable::Nullable;
pub use object::Object;

use crate::runtime::gc::GcTrace;
use crate::runtime::scope::GcScope;
use crate::runtime::reference::GcReference;
use crate::runtime::utilities::tag::Tag;
use crate::runtime::utilities::variable::Variable;
use crate::runtime::value::GcValue;

pub enum Data<'a> {
    Array(Array<'a>),
    Boolean(bool),
    Class(Class<'a>),
    Float(f64),
    Function(Function<'a>),
    Generic(Generic<'a>),
    Integer(isize),
    Method(Method<'a>),
    Nullable(Nullable<'a>),
    Object(Object<'a>),
    String(String),
}

impl<'a> Data<'a> {
    pub fn array(elements: Vec<GcReference<'a>>) -> Self {
        Data::Array(Array::new(elements))
    }

    pub fn boolean(boolean: bool) -> Self {
        Data::Boolean(boolean)
    }

    pub fn class(tag: Tag, scope: GcScope<'a>, parent: Option<GcValue<'a>>) -> Self {
        Data::Class(Class::new(tag, scope, parent))
    }

    pub fn float(float: f64) -> Self {
        Data::Float(float)
    }

    pub fn function(
        tag: Tag, scope: GcScope<'a>, parameters: Box<[Variable<'a>]>, rest: Option<Variable<'a>>, r#return: Option<GcValue<'a>>, implementation: impl FunctionImplementation<'a> + 'a
    ) -> Self {
        Data::Function(Function::new(tag, scope, parameters, rest, r#return, implementation))
    }

    pub fn generic(tag: Tag, scope: GcScope<'a>, parameters: Box<[Box<str>]>, implementation: impl GenericImplementation<'a> + 'a) -> Self {
        Data::Generic(Generic::new(tag, scope, parameters, implementation))
    }

    pub fn integer(integer: isize) -> Self {
        Data::Integer(integer)
    }

    pub fn method(function: GcValue<'a>, this: GcValue<'a>) -> Self {
        Data::Method(Method::new(function, this))
    }

    pub fn nullable(value: Option<GcValue<'a>>) -> Self {
        Data::Nullable(Nullable::new(value))
    }

    pub fn object() -> Self {
        Data::Object(Object::new())
    }

    pub fn string(string: String) -> Self {
        Data::String(string)
    }
}

impl GcTrace for Data<'_> {
    fn trace(&mut self) {
        match self {
            Data::Array(array)       => array.trace(),
            Data::Class(class)       => class.trace(),
            Data::Function(function) => function.trace(),
            Data::Generic(generic)   => generic.trace(),
            Data::Method(method)     => method.trace(),
            Data::Nullable(nullable) => nullable.trace(),
            Data::Object(object)     => object.trace(),
            _ => (),
        }
    }
}
