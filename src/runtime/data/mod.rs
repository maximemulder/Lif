mod class;
mod function_primitive;
mod function_standard;
// mod generic_primitive;
mod generic_standard;
mod method;
mod nullable;
mod object;
mod tag;

pub use class::Class;
pub use function_primitive::FunctionPrimitive;
pub use function_standard::FunctionStandard;
// pub use generic_primitive::GenericPrimitive;
pub use generic_standard::GenericStandard;
pub use method::Method;
pub use nullable::Nullable;
pub use object::Object;
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
    Class(Class<'a>),
    FunctionPrimitive(FunctionPrimitive<'a>),
    FunctionStandard(FunctionStandard<'a>),
//    GenericPrimitive(GenericPrimitive<'a>),
    GenericStandard(GenericStandard<'a>),
    Integer(isize),
    Method(Method<'a>),
    Nullable(Nullable<'a>),
    Object(Object<'a>),
    String(String),
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
        Data::FunctionStandard(FunctionStandard::new(tag, scope, parameters, r#type, block))
    }

    pub fn new_integer(integer: isize) -> Self {
        Data::Integer(integer)
    }

    pub fn new_generic(tag: Tag, scope: GcScope<'a>, generics: Ref<[Ref<str>]>, node: Ref<dyn Executable>) -> Self {
        Data::GenericStandard(GenericStandard::new(tag, scope, generics, node))
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

    pub fn new_primitive(tag: Tag, parameters: Box<[GcValue<'a>]>, callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> ReturnReference<'a>) -> Self {
        Data::FunctionPrimitive(FunctionPrimitive::new(tag, parameters, callback))
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
            Data::FunctionPrimitive(function) => function.trace(),
            Data::FunctionStandard(function)  => function.trace(),
            Data::Class(class)                => class.trace(),
//            Data::GenericPrimitive(generic)   => generic.trace(),
            Data::GenericStandard(generic)    => generic.trace(),
            Data::Method(method)              => method.trace(),
            Data::Nullable(nullable)          => nullable.trace(),
            Data::Object(object)              => object.trace(),
            _ => (),
        }
    }
}
