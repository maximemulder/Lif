use crate::runtime::data::{ Array, Class, Function, Generic, Method, Nullable, Object };
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::r#return::{ Return, ReturnReference, ReturnValue };
use crate::runtime::utilities::parameters;
use crate::runtime::utilities::tag::Tag;

use std::ops::Deref;
use std::marker::PhantomData;

impl GcTrace for String {
    fn trace(&mut self) {}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Value<'a> {
    pub class: GcRef<Class<'a>>,
    data: Data<'a>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Data<'a> {
    bits: usize,
    phantom: PhantomData<&'a ()>
}

impl<'a> Data<'a> {
    pub fn new(bits: usize) -> Self {
        Self {
            bits,
            phantom: PhantomData,
        }
    }
}

pub trait Primitive<'a> {
    fn get(bits: usize) -> Self;
    fn set(primitive: Self) -> usize;
}

pub trait PrimitiveClass<'a> {
    fn get_class(engine: &Engine<'a>) -> GcRef<Class<'a>>;
}

pub trait PrimitiveGeneric<'a> {
    fn get_generic(engine: &Engine<'a>) -> GcRef<Generic<'a>>;
}

impl<'a> Primitive<'a> for bool {
    fn get(bits: usize) -> Self {
        unsafe {
            std::mem::transmute::<u8, bool>(bits as u8)
        }
    }

    fn set(primitive: Self) -> usize {
        unsafe {
            std::mem::transmute::<bool, u8>(primitive) as usize
        }
    }
}

impl<'a> Primitive<'a> for isize {
    fn get(bits: usize) -> Self {
        unsafe {
            std::mem::transmute::<usize, isize>(bits)
        }
    }

    fn set(primitive: Self) -> usize {
        unsafe {
            std::mem::transmute::<isize, usize>(primitive)
        }
    }
}

impl<'a> Primitive<'a> for f64 {
    fn get(bits: usize) -> Self {
        unsafe {
            std::mem::transmute::<usize, f64>(bits)
        }
    }

    fn set(primitive: Self) -> usize {
        unsafe {
            std::mem::transmute::<f64, usize>(primitive)
        }
    }
}

impl<'a, T: GcTrace> Primitive<'a> for GcRef<T> {
    fn get(bits: usize) -> Self {
        unsafe {
            std::mem::transmute::<usize, GcRef<T>>(bits)
        }
    }

    fn set(primitive: Self) -> usize {
        unsafe {
            std::mem::transmute::<GcRef<T>, usize>(primitive)
        }
    }
}

impl<'a> PrimitiveGeneric<'a> for Array<'a> {
    fn get_generic(engine: &Engine<'a>) -> GcRef<Generic<'a>> {
        engine.primitives.array
    }
}

impl<'a> PrimitiveGeneric<'a> for Nullable<'a> {
    fn get_generic(engine: &Engine<'a>) -> GcRef<Generic<'a>> {
        engine.primitives.nullable
    }
}

impl<'a> PrimitiveClass<'a> for bool {
    fn get_class(engine: &Engine<'a>) -> GcRef<Class<'a>> {
        engine.primitives.boolean
    }
}

impl<'a> PrimitiveClass<'a> for isize {
    fn get_class(engine: &Engine<'a>) -> GcRef<Class<'a>> {
        engine.primitives.integer
    }
}

impl<'a> PrimitiveClass<'a> for f64 {
    fn get_class(engine: &Engine<'a>) -> GcRef<Class<'a>> {
        engine.primitives.float
    }
}

impl<'a> PrimitiveClass<'a> for Class<'a> {
    fn get_class(engine: &Engine<'a>) -> GcRef<Class<'a>> {
        engine.primitives.class
    }
}

impl<'a> PrimitiveClass<'a> for Function<'a> {
    fn get_class(engine: &Engine<'a>) -> GcRef<Class<'a>> {
        engine.primitives.function
    }
}

impl<'a> PrimitiveClass<'a> for Generic<'a> {
    fn get_class(engine: &Engine<'a>) -> GcRef<Class<'a>> {
        engine.primitives.generic
    }
}

impl<'a> PrimitiveClass<'a> for Method<'a> {
    fn get_class(engine: &Engine<'a>) -> GcRef<Class<'a>> {
        engine.primitives.method
    }
}

impl<'a> PrimitiveClass<'a> for Object<'a> {
    fn get_class(engine: &Engine<'a>) -> GcRef<Class<'a>> {
        engine.primitives.object
    }
}

impl<'a> PrimitiveClass<'a> for String {
    fn get_class(engine: &Engine<'a>) -> GcRef<Class<'a>> {
        engine.primitives.string
    }
}

impl<'a> Value<'a> {
    pub fn new<T: Primitive<'a> + PrimitiveClass<'a>>(engine: &Engine<'a>, primitive: T) -> Self {
        Self {
            class: T::get_class(engine),
            data: Data::new(T::set(primitive)),
        }
    }

    pub fn new_gc<T: PrimitiveClass<'a> + GcTrace>(engine: &Engine<'a>, primitive: GcRef<T>) -> Self {
        Self {
            class: T::get_class(engine),
            data: Data::new(GcRef::set(primitive)),
        }
    }

    pub fn new_class<T: GcTrace>(class: GcRef<Class<'a>>, primitive: GcRef<T>) -> Self {
        Self {
            class,
            data: Data::new(GcRef::set(primitive)),
        }
    }

    pub fn get<T: Primitive<'a> + PrimitiveClass<'a>>(self, engine: &Engine<'a>) -> T {
        debug_assert!(self.isa(T::get_class(engine)));
        T::get(self.data.bits)
    }

    pub fn get_gc<T: PrimitiveClass<'a> + GcTrace>(self, engine: &Engine<'a>) -> GcRef<T> {
        debug_assert!(self.isa(T::get_class(engine)));
        GcRef::<T>::get(self.data.bits)
    }

    pub fn get_gn<T: PrimitiveGeneric<'a> + GcTrace>(self, engine: &Engine<'a>) -> GcRef<T> {
        debug_assert!(self.isa_generic(T::get_generic(engine)));
        GcRef::<T>::get(self.data.bits)
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
        Ok(self.get_gn(engine))
    }

    pub fn get_cast_class(self, engine: &Engine<'a>) -> Return<GcRef<Class<'a>>> {
        self.cast(engine.primitives.class)?;
        Ok(self.get_gc(engine))
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
