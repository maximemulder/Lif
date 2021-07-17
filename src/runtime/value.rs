use crate::runtime::data::{ Array, Class, Function, Generic, Method, Nullable, Object };
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::r#return::{ Return, ReturnReference, ReturnValue };
use crate::runtime::utilities::parameters;
use crate::runtime::utilities::tag::Tag;

use std::ops::Deref;

impl GcTrace for String {
    fn trace(&mut self) {}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Value<'a> {
    pub class: GcRef<Class<'a>>,
    data: usize,
}

pub trait Primitive<'a> {
    fn set(class: GcRef<Class<'a>>, primitive: Self) -> Value<'a>;
    fn get(engine: &Engine<'a>, value: Value<'a>) -> Self;
}

pub trait PrimitiveGc<'a> : GcTrace + Sized {
    fn get_gc(engine: &Engine<'a>, value: Value<'a>) -> GcRef<Self>;
}

pub trait Reflective<'a> : PrimitiveGc<'a> {
    fn class(engine: &Engine<'a>) -> GcRef<Class<'a>>;
}

impl<'a> Primitive<'a> for bool {
    fn set(class: GcRef<Class<'a>>, primitive: Self) -> Value<'a> {
        Value {
            class,
            data: unsafe {
                std::mem::transmute::<bool, u8>(primitive) as usize
            },
        }
    }

    fn get(engine: &Engine<'a>, value: Value<'a>) -> Self {
        debug_assert!(value.class == engine.primitives.boolean);
        unsafe {
            std::mem::transmute::<u8, bool>(value.data as u8)
        }
    }
}

impl<'a> Primitive<'a> for isize {
    fn set(class: GcRef<Class<'a>>, primitive: Self) -> Value<'a> {
        Value {
            class,
            data: unsafe {
                std::mem::transmute::<isize, usize>(primitive)
            },
        }
    }

    fn get(engine: &Engine<'a>, value: Value<'a>) -> Self {
        debug_assert!(value.class == engine.primitives.integer);
        unsafe {
            std::mem::transmute::<usize, isize>(value.data)
        }
    }
}

impl<'a> Primitive<'a> for f64 {
    fn set(class: GcRef<Class<'a>>, primitive: Self) -> Value<'a> {
        Value {
            class,
            data: unsafe {
                std::mem::transmute::<f64, usize>(primitive)
            },
        }
    }

    fn get(engine: &Engine<'a>, value: Value<'a>) -> Self {
        debug_assert!(value.class == engine.primitives.float);
        unsafe {
            std::mem::transmute::<usize, f64>(value.data)
        }
    }
}

impl<'a, T: GcTrace> Primitive<'a> for GcRef<T> {
    fn set(class: GcRef<Class<'a>>, primitive: Self) -> Value<'a> {
        Value {
            class,
            data: unsafe {
                std::mem::transmute::<GcRef<T>, usize>(primitive)
            },
        }
    }

    fn get(_: &Engine<'a>, value: Value<'a>) -> Self {
        unsafe {
            std::mem::transmute::<usize, GcRef<T>>(value.data)
        }
    }
}

impl<'a> PrimitiveGc<'a> for Array<'a> {
    fn get_gc(engine: &Engine<'a>, value: Value<'a>) -> GcRef<Self> {
        debug_assert!(value.isa_generic(engine.primitives.array));
        value.get::<GcRef<Array>>(engine)
    }
}

impl<'a> PrimitiveGc<'a> for Class<'a> {
    fn get_gc(engine: &Engine<'a>, value: Value<'a>) -> GcRef<Self> {
        debug_assert!(value.isa(engine.primitives.class));
        value.get::<GcRef<Class>>(engine)
    }
}

impl<'a> PrimitiveGc<'a> for Function<'a> {
    fn get_gc(engine: &Engine<'a>, value: Value<'a>) -> GcRef<Self> {
        debug_assert!(value.isa(engine.primitives.function));
        value.get::<GcRef<Function>>(engine)
    }
}

impl<'a> PrimitiveGc<'a> for Generic<'a> {
    fn get_gc(engine: &Engine<'a>, value: Value<'a>) -> GcRef<Self> {
        debug_assert!(value.isa(engine.primitives.generic));
        value.get::<GcRef<Generic>>(engine)
    }
}

impl<'a> PrimitiveGc<'a> for Method<'a> {
    fn get_gc(engine: &Engine<'a>, value: Value<'a>) -> GcRef<Self> {
        debug_assert!(value.isa(engine.primitives.method));
        value.get::<GcRef<Method>>(engine)
    }
}

impl<'a> PrimitiveGc<'a> for Nullable<'a> {
    fn get_gc(engine: &Engine<'a>, value: Value<'a>) -> GcRef<Self> {
        debug_assert!(value.isa_generic(engine.primitives.nullable));
        value.get::<GcRef<Nullable>>(engine)
    }
}

impl<'a> PrimitiveGc<'a> for Object<'a> {
    fn get_gc(engine: &Engine<'a>, value: Value<'a>) -> GcRef<Self> {
        debug_assert!(value.isa(engine.primitives.object));
        value.get::<GcRef<Object>>(engine)
    }
}

impl<'a> PrimitiveGc<'a> for String {
    fn get_gc(engine: &Engine<'a>, value: Value<'a>) -> GcRef<Self> {
        debug_assert!(value.isa(engine.primitives.string));
        value.get::<GcRef<String>>(engine)
    }
}

impl<'a> Value<'a> {
    pub fn new<T: Primitive<'a>>(class: GcRef<Class<'a>>, primitive: T) -> Self {
        T::set(class, primitive)
    }

    pub fn new_gc<T: GcTrace>(class: GcRef<Class<'a>>, value: GcRef<T>) -> Self {
        Self::new(class, value)
    }

    pub fn get<T: Primitive<'a>>(self, engine: &Engine<'a>) -> T {
        T::get(engine, self)
    }

    pub fn get_gc<T: PrimitiveGc<'a>>(self, engine: &Engine<'a>) -> GcRef<T> {
        T::get_gc(engine, self)
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
        self.cast(engine.primitives.boolean)?;
        Ok(self.get(engine))
    }

    pub fn get_cast_array(self, engine: &Engine<'a>) -> Return<GcRef<Array<'a>>> {
        self.cast_generic(engine.primitives.array)?;
        Ok(self.get(engine))
    }

    pub fn get_cast_class(self, engine: &Engine<'a>) -> Return<GcRef<Class<'a>>> {
        self.cast(engine.primitives.class)?;
        Ok(self.get(engine))
    }
}

impl GcTrace for Value<'_> {
    fn trace(&mut self) {
        self.class.trace();
    }
}

impl<'a> Value<'a> {
    pub fn get_tag(&self, engine: &Engine<'a>) -> Tag {
        if self.isa(engine.primitives.class) {
            self.get_gc::<Class>(engine).tag().clone()
        } else if self.isa(engine.primitives.function) {
            self.get_gc::<Function>(engine).tag().clone()
        } else if self.isa(engine.primitives.generic) {
            self.get_gc::<Generic>(engine).tag().clone()
        } else {
            panic!();
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
