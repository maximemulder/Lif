mod callable;
mod class;
mod function;
mod generic;
mod method;
mod object;
mod primitive;
mod tag;

pub use callable::Callable;
pub use class::Class;
pub use function::Function;
pub use generic::Generic;
pub use method::Method;
pub use object::Object;
pub use primitive::Primitive;
pub use tag::{ Tag, Tagger };

use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTraceable;
use crate::runtime::scope::GcScope;
use crate::runtime::reference::GcReference;
use crate::runtime::value::GcValue;

pub enum Data<'a, 'b> {
    Array(Vec<GcReference<'a, 'b>>),
    Boolean(bool),
    Callable(Box<dyn Callable<'a, 'b> + 'b>),
    Class(Class<'a, 'b>),
    Generic(Generic<'a, 'b>),
    Integer(usize),
    Method(Method<'a, 'b>),
    Object(Object<'a, 'b>),
    String(String),
    Null,
}

impl<'a, 'b> Data<'a, 'b> {
    pub fn new_array(elements: Vec<GcReference<'a, 'b>>) -> Self {
        Data::Array(elements)
    }

    pub fn new_boolean(boolean: bool) -> Self {
        Data::Boolean(boolean)
    }

    pub fn new_class(tag: Tag, parent: Option<GcValue<'a, 'b>>) -> Self {
        Data::Class(Class::new(tag, parent))
    }

    pub fn new_function(tag: Tag, scope: GcScope<'a, 'b>, parameters: &'b [Node<'a>], r#type: Option<GcValue<'a, 'b>>, block: &'b Node<'a>) -> Self {
        Data::Callable(Box::new(Function::new(tag, scope, parameters, r#type, block)))
    }

    pub fn new_integer(integer: usize) -> Self {
        Data::Integer(integer)
    }

    pub fn new_generic(tag: Tag, generics: &'b [&'a str], node: &'b dyn Executable<'a>) -> Self {
        Data::Generic(Generic::new(tag, generics, node))
    }

    pub fn new_method(function: GcValue<'a, 'b>, this: GcValue<'a, 'b>) -> Self {
        Data::Method(Method::new(function, this))
    }

    pub fn new_object() -> Self {
        Data::Object(Object::new())
    }

    pub fn new_primitive(tag: Tag, parameters: Box<[GcValue<'a, 'b>]>, callback: &'b dyn Fn(&mut Engine<'a, 'b>, Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b>) -> Self {
        Data::Callable(Box::new(Primitive::new(tag, parameters, callback)))
    }

    pub fn new_string(string: String) -> Self {
        Data::String(string)
    }
}

impl GcTraceable for Data<'_, '_> {
    fn trace(&mut self) {
        match self {
            Data::Array(references)  => for reference in references.iter_mut() {
                reference.trace();
            },
            Data::Callable(callable) => callable.trace(),
            Data::Class(class)       => class.trace(),
            Data::Method(method)     => method.trace(),
            Data::Object(object)     => object.trace(),
            _ => (),
        }
    }
}
