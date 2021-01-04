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

use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::scope::GcScope;
use crate::runtime::reference::GcReference;
use crate::runtime::value::GcValue;

pub enum Data<'a> {
    Array(Vec<GcReference<'a>>),
    Boolean(bool),
    Callable(Box<dyn Callable<'a> + 'a>),
    Class(Class<'a>),
    Generic(Generic),
    Integer(isize),
    Method(Method<'a>),
    Object(Object<'a>),
    String(String),
    Null,
}

impl<'a> Data<'a> {
    pub fn new_array(elements: Vec<GcReference<'a>>) -> Self {
        Data::Array(elements)
    }

    pub fn new_boolean(boolean: bool) -> Self {
        Data::Boolean(boolean)
    }

    pub fn new_class(tag: Tag, parent: Option<GcValue<'a>>) -> Self {
        Data::Class(Class::new(tag, parent))
    }

    pub fn new_function(tag: Tag, scope: GcScope<'a>, parameters: Ref<[Node]>, r#type: Option<GcValue<'a>>, block: Ref<Node>) -> Self {
        Data::Callable(Box::new(Function::new(tag, scope, parameters, r#type, block)))
    }

    pub fn new_integer(integer: isize) -> Self {
        Data::Integer(integer)
    }

    pub fn new_generic(tag: Tag, generics: Ref<[Ref<str>]>, node: Ref<dyn Executable>) -> Self {
        Data::Generic(Generic::new(tag, generics, node))
    }

    pub fn new_method(function: GcValue<'a>, this: GcValue<'a>) -> Self {
        Data::Method(Method::new(function, this))
    }

    pub fn new_object() -> Self {
        Data::Object(Object::new())
    }

    pub fn new_primitive(tag: Tag, parameters: Box<[GcValue<'a>]>, callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> ReturnReference<'a>) -> Self {
        Data::Callable(Box::new(Primitive::new(tag, parameters, callback)))
    }

    pub fn new_string(string: String) -> Self {
        Data::String(string)
    }
}

impl GcTrace for Data<'_> {
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
