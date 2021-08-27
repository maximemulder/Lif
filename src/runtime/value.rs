use crate::runtime::data::{ Data, Primitive, PrimitiveClass, PrimitiveGeneric };
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::primitives::{ Array, Class, Function, Generic };
use crate::runtime::r#return::{ Return, ReturnReference, ReturnValue };
use crate::runtime::utilities::parameters;

use std::ops::Deref;

impl GcTrace for String {}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Value<'a> {
    pub class: GcRef<Class<'a>>,
    data: Data<'a>,
}

impl<'a> Value<'a> {
    pub fn new<T: Primitive<'a>>(class: GcRef<Class<'a>>, primitive: T) -> Self {
        Self {
            class,
            data: Data::new(primitive),
        }
    }

    pub fn primitive<T: Primitive<'a> + PrimitiveClass<'a>>(engine: &Engine<'a>, primitive: T) -> Self {
        Self::new(T::get_class(engine), primitive)
    }

    pub fn primitive_gc<T: PrimitiveClass<'a> + GcTrace>(engine: &Engine<'a>, primitive: GcRef<T>) -> Self {
        Self::new(T::get_class(engine), primitive)
    }

    pub fn alloc<T: GcTrace>(engine: &mut Engine<'a>, class: GcRef<Class<'a>>, primitive: T) -> Self {
        Self::new(class, engine.alloc(primitive))
    }

    pub fn alloc_primitive<T: PrimitiveClass<'a> + GcTrace>(engine: &mut Engine<'a>, primitive: T) -> Self {
        Self::alloc(engine, T::get_class(engine), primitive)
    }

    pub fn get<T: Primitive<'a> + PrimitiveClass<'a>>(self, engine: &Engine<'a>) -> T {
        debug_assert!(self.isa(T::get_class(engine)));
        T::get(self.data.bits)
    }

    pub fn get_gc<T: PrimitiveClass<'a> + GcTrace>(self, engine: &Engine<'a>) -> GcRef<T> {
        debug_assert!(self.isa(T::get_class(engine)));
        GcRef::get(self.data.bits)
    }

    pub fn get_gn<T: PrimitiveGeneric<'a> + GcTrace>(self, engine: &Engine<'a>) -> GcRef<T> {
        debug_assert!(self.isa_generic(T::get_generic(engine)));
        GcRef::get(self.data.bits)
    }

    pub fn get_unchecked<T: Primitive<'a>>(self) -> T {
        T::get(self.data.bits)
    }
}

impl<'a> Value<'a> {
    pub fn isa(self, class: GcRef<Class<'a>>) -> bool {
        self.class.is(class)
    }

    pub fn isa_generic(self, generic: GcRef<Generic<'a>>) -> bool {
        self.class.is_generic(generic)
    }

    pub fn cast(self, class: GcRef<Class<'a>>) -> Return<()> {
        if self.isa(class) {
            Ok(())
        } else {
            Err(error_cast(self.class, class))
        }
    }

    pub fn cast_generic(self, generic: GcRef<Generic<'a>>) -> Return<()> {
        if self.isa_generic(generic) {
            Ok(())
        } else {
            Err(error_cast_generic(self.class, generic))
        }
    }
}

impl<'a> Value<'a> {
    pub fn get_method(&self, name: &str) -> ReturnValue<'a> {
        if let Some(method) = self.class.get_method(name) {
            Ok(method)
        } else {
            Err(error_undefined_method(name, self.class))
        }
    }

    pub fn call_method(self, engine: &mut Engine<'a>, name: &str, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
        let mut values = Vec::new();
        values.push(self);
        values.extend_from_slice(arguments);
        self.call_method_self(engine, name, &mut values)
    }

    pub fn call_method_self(self, engine: &mut Engine<'a>, name: &str, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
        let method = self.get_method(name)?;
        let array = parameters::pack(engine, arguments);
        method.get_method("__cl__")?.get_gc::<Function>(engine).call(engine, &mut [method, array])
    }

    pub fn call_fstr(self, engine: &mut Engine<'a>) -> Return<String> {
        Ok(self.call_method(engine, "__fstr__", &mut [])?.read()?.get_gc::<String>(engine).deref().clone())
    }

    pub fn call_sstr(self, engine: &mut Engine<'a>) -> Return<String> {
        Ok(self.call_method(engine, "__sstr__", &mut [])?.read()?.get_gc::<String>(engine).deref().clone())
    }
}

impl<'a> Value<'a> {
    pub fn get_cast_boolean(self, engine: &Engine<'a>) -> Return<bool> {
        self.cast(engine.environment.boolean)?;
        Ok(self.get(engine))
    }

    pub fn get_cast_array(self, engine: &Engine<'a>) -> Return<GcRef<Array<'a>>> {
        self.cast_generic(engine.environment.array)?;
        Ok(self.get_gn(engine))
    }

    pub fn get_cast_class(self, engine: &Engine<'a>) -> Return<GcRef<Class<'a>>> {
        self.cast(engine.environment.class)?;
        Ok(self.get_gc(engine))
    }
}

impl GcTrace for Value<'_> {
    fn trace(&mut self) {
        self.class.trace();
        if self.class.gc() {
            self.data.trace();
        }
    }
}

fn error_undefined_method<'a>(method: &str, class: GcRef<Class<'a>>) -> Error {
    Error::new_runtime(&format!("Method `{}` is undefined for the type `{}`.", method, class.tag()))
}

fn error_cast<'a>(value: GcRef<Class<'a>>, r#type: GcRef<Class<'a>>) -> Error {
    Error::new_runtime(&format!("Cannot cast a value of the type `{}` to the type `{}`.", value.tag(), r#type.tag()))
}

fn error_cast_generic<'a>(value: GcRef<Class<'a>>, r#type: GcRef<Generic<'a>>) -> Error {
    Error::new_runtime(&format!("Cannot cast a value of the type `{}` to the type `{}`.", value.tag(), r#type.tag()))
}
