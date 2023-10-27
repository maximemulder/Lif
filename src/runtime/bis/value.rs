use crate::runtime::bis::data::{Data, Ref, GcClass, GcFunction, GcGeneric, GcList, GcMethod, GcObject, GcString};
use crate::runtime::gc::{GcRef, GcTrace};

#[derive(Copy, Clone)]
pub struct Value<'a> {
    pub class: GcClass<'a>,
    pub data: Data<'a>,
}

impl<'a> Value<'a> {
    pub fn new(class: GcClass<'a>, data: Data<'a>) -> Self {
        Self { class, data }
    }

    pub fn dummy() -> Self {
        Self { class: GcRef::null(), data: Data::Void(()) }
    }
}

impl<'a> Value<'a> {
    pub fn isa(self, class: GcClass<'a>) -> bool {
        self.class.isa(class)
    }

    pub fn read(self) -> Value<'a> {
        match self.data {
            Data::Ref(r#ref) => r#ref.read().expect("TODO"),
            _ => self,
        }
    }

    pub fn as_bool(self) -> bool {
        match self.data {
            Data::Bool(bool) => bool,
            _ => panic!("TODO: Error handling"),
        }
    }

    pub fn as_class(self) -> GcClass<'a> {
        match self.data {
            Data::Class(class) => class,
            _ => panic!("TODO: Error handling"),
        }
    }

    pub fn as_float(self) -> f64 {
        match self.data {
            Data::Float(float) => float,
            _ => panic!("TODO: Error handling"),
        }
    }

    pub fn as_function(self) -> GcFunction<'a> {
        match self.data {
            Data::Function(function) => function,
            _ => panic!("TODO: Error handling"),
        }
    }

    pub fn as_generic(self) -> GcGeneric<'a> {
        match self.data {
            Data::Generic(generic) => generic,
            _ => panic!("TODO: Error handling"),
        }
    }

    pub fn as_int(self) -> i64 {
        match self.data {
            Data::Int(int) => int,
            _ => panic!("TODO: Error handling"),
        }
    }

    pub fn as_list(self) -> GcList<'a> {
        match self.data {
            Data::List(list) => list,
            _ => panic!("TODO: Error handling"),
        }
    }

    pub fn as_method(self) -> GcMethod<'a> {
        match self.data {
            Data::Method(method) => method,
            _ => panic!("TODO: Error handling"),
        }
    }

    pub fn as_object(self) -> GcObject<'a> {
        match self.data {
            Data::Object(object) => object,
            _ => panic!("TODO: Error handling"),
        }
    }

    pub fn as_ref(self) -> Ref<'a> {
        match self.data {
            Data::Ref(r#ref) => r#ref,
            _ => panic!("TODO: Error handling"),
        }
    }

    pub fn as_string(self) -> GcString<'a> {
        match self.data {
            Data::String(string) => string,
            _ => panic!("TODO: Error handling"),
        }
    }
}

impl GcTrace for Value<'_> {
    fn trace(&mut self) {
        self.class.trace();
        self.data.trace();
    }
}
