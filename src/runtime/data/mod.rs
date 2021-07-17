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

/*
use crate::runtime::scope::GcScope;
use crate::runtime::reference::GcReference;
use crate::runtime::utilities::parameters::Parameters;
use crate::runtime::utilities::tag::Tag;
use crate::runtime::value::GcValue;

impl<'a> Data<'a> {
    pub fn class(tag: Tag, scope: GcScope<'a>, parent: Option<GcValue<'a>>) -> Self {
        Data::Class(Class::new(tag, scope, parent))
    }

    pub fn function(
        tag: Tag, scope: GcScope<'a>, parameters: Parameters<'a>, r#return: Option<GcValue<'a>>, implementation: impl FunctionImplementation<'a> + 'a
    ) -> Self {
        Data::Function(Function::new(tag, scope, parameters, r#return, implementation))
    }

    pub fn generic(tag: Tag, scope: GcScope<'a>, parameters: Box<[Box<str>]>, implementation: impl GenericImplementation<'a> + 'a) -> Self {
        Data::Generic(Generic::new(tag, scope, parameters, implementation))
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
 */
