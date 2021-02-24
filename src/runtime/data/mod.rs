mod array;
mod class;
mod function;
mod generic;
mod method;
mod nullable;
mod object;
mod tag;

pub use array::Array;
pub use class::Class;
pub use function::{ Function, FunctionCode, FunctionPrimitive };
pub use generic::{ Generic, GenericCode, GenericPrimitive };
pub use method::Method;
pub use nullable::Nullable;
pub use object::Object;
pub use tag::{ Tag, Tagger };

use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::gc::GcTrace;
use crate::runtime::scope::GcScope;
use crate::runtime::reference::GcReference;
use crate::runtime::utilities::Callable;
use crate::runtime::value::GcValue;

pub enum Data<'a> {
    Array(Array<'a>),
    Boolean(bool),
    Class(Class<'a>),
    FunctionCode(FunctionCode<'a>),
    FunctionPrimitive(FunctionPrimitive<'a>),
    GenericCode(GenericCode<'a>),
    GenericPrimitive(GenericPrimitive<'a>),
    Integer(isize),
    Method(Method<'a>),
    Nullable(Nullable<'a>),
    Object(Object<'a>),
    String(String),
}

impl<'a> Data<'a> {
    pub fn new_array(elements: Vec<GcReference<'a>>) -> Self {
        Data::Array(Array::new(elements))
    }

    pub fn new_boolean(boolean: bool) -> Self {
        Data::Boolean(boolean)
    }

    pub fn new_class(tag: Tag, parent: Option<GcValue<'a>>) -> Self {
        Data::Class(Class::new(tag, parent))
    }

    pub fn new_function(tag: Tag, parameters: Box<[GcValue<'a>]>, rest: Option<GcValue<'a>>, names: Box<[Ref<str>]>, r#return: Option<GcValue<'a>>, scope: GcScope<'a>, block: Ref<Node>) -> Self {
        Data::FunctionCode(Function::new_code(tag, parameters, rest, names, r#return, scope, block))
    }

    pub fn new_function_primitive(tag: Tag, parameters: Box<[GcValue<'a>]>, rest: Option<GcValue<'a>>, r#return: Option<GcValue<'a>>, callback: &'a Callable<'a>) -> Self {
        Data::FunctionPrimitive(Function::new_primitive(tag, parameters, rest, r#return, callback))
    }

    pub fn new_generic(tag: Tag, scope: GcScope<'a>, parameters: Ref<[Ref<str>]>, node: Ref<dyn Executable>) -> Self {
        Data::GenericCode(Generic::new_code(tag, scope, parameters, node))
    }

    pub fn new_generic_primitive(tag: Tag, scope: GcScope<'a>, parameters: Vec<Box<str>>, callback: &'a Callable<'a>) -> Self {
        Data::GenericPrimitive(Generic::new_primitive(tag, scope, parameters, callback))
    }

    pub fn new_integer(integer: isize) -> Self {
        Data::Integer(integer)
    }

    pub fn new_method(function: GcValue<'a>, this: GcValue<'a>) -> Self {
        Data::Method(Method::new(function, this))
    }

    pub fn new_nullable(value: Option<GcValue<'a>>) -> Self {
        Data::Nullable(Nullable::new(value))
    }

    pub fn new_object() -> Self {
        Data::Object(Object::new())
    }

    pub fn new_string(string: String) -> Self {
        Data::String(string)
    }
}

impl GcTrace for Data<'_> {
    fn trace(&mut self) {
        match self {
            Data::Array(array)                => array.trace(),
            Data::Class(class)                => class.trace(),
            Data::FunctionCode(function)      => function.trace(),
            Data::FunctionPrimitive(function) => function.trace(),
            Data::GenericCode(generic)        => generic.trace(),
            Data::GenericPrimitive(generic)   => generic.trace(),
            Data::Method(method)              => method.trace(),
            Data::Nullable(nullable)          => nullable.trace(),
            Data::Object(object)              => object.trace(),
            _ => (),
        }
    }
}
