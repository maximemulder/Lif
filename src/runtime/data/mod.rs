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

use std::marker::PhantomData;
use crate::runtime::engine::Engine;
use crate::runtime::gc::{ GcRef, GcTrace };

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Data<'a> {
    pub bits: usize,
    phantom: PhantomData<&'a ()>
}

impl<'a> Data<'a> {
    pub fn new<T: Primitive<'a>>(primitive: T) -> Self {
        Self {
            bits: Primitive::set(primitive),
            phantom: PhantomData,
        }
    }
}

impl GcTrace for Data<'_> {
    fn trace(&mut self) {
        unsafe {
            std::mem::transmute::<usize, GcRef<()>>(self.bits).trace();
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
