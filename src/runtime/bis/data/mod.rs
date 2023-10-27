pub mod class;
pub mod function;
pub mod generic;
pub mod list;
pub mod method;
pub mod object;
pub mod r#ref;
pub mod string;

pub use class::{Class, GcClass};
pub use function::{Function, FunctionBody, GcFunction};
pub use generic::{Generic, GcGeneric};
pub use list::{List, GcList};
pub use method::{Method, GcMethod};
pub use object::{Object, GcObject};
pub use r#ref::Ref;
pub use string::{String, GcString};

use crate::runtime::gc::GcTrace;

#[derive(Clone, Copy)]
pub enum Data<'a> {
    Bool(bool),
    Float(f64),
    Int(i64),
    Void(()),
    Ref(Ref<'a>),
    Class(GcClass<'a>),
    Function(GcFunction<'a>),
    Generic(GcGeneric<'a>),
    List(GcList<'a>),
    Method(GcMethod<'a>),
    Object(GcObject<'a>),
    String(GcString<'a>),
}

impl GcTrace for Data<'_> {
    fn trace(&mut self) {
        match self {
            Data::Class(class) => class.trace(),
            Data::Function(function) => function.trace(),
            Data::Generic(generic) => generic.trace(),
            Data::List(list) => list.trace(),
            Data::Method(method) => method.trace(),
            Data::Object(object) => object.trace(),
            Data::Ref(r#ref) => r#ref.trace(),
            _ => (),
        }
    }
}
