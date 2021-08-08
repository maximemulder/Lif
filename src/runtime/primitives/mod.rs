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

use crate::runtime::data::{ Primitive, PrimitiveClass };
use crate::runtime::engine::Engine;
use crate::runtime::gc::{ GcRef, GcTrace };

use std::mem::transmute;

impl<'a> Primitive<'a> for bool {
    fn get(bits: usize) -> Self {
        unsafe {
            transmute::<u8, bool>(bits as u8)
        }
    }

    fn set(primitive: Self) -> usize {
        unsafe {
            transmute::<bool, u8>(primitive) as usize
        }
    }
}

impl<'a> Primitive<'a> for isize {
    fn get(bits: usize) -> Self {
        unsafe {
            transmute::<usize, isize>(bits)
        }
    }

    fn set(primitive: Self) -> usize {
        unsafe {
            transmute::<isize, usize>(primitive)
        }
    }
}

impl<'a> Primitive<'a> for f64 {
    fn get(bits: usize) -> Self {
        unsafe {
            transmute::<usize, f64>(bits)
        }
    }

    fn set(primitive: Self) -> usize {
        unsafe {
            transmute::<f64, usize>(primitive)
        }
    }
}

impl<'a, T: GcTrace> Primitive<'a> for GcRef<T> {
    fn get(bits: usize) -> Self {
        unsafe {
            transmute::<usize, GcRef<T>>(bits)
        }
    }

    fn set(primitive: Self) -> usize {
        unsafe {
            transmute::<GcRef<T>, usize>(primitive)
        }
    }
}

impl<'a> PrimitiveClass<'a> for bool {
    fn get_class(engine: &Engine<'a>) -> GcRef<Class<'a>> {
        engine.environment.boolean
    }
}

impl<'a> PrimitiveClass<'a> for isize {
    fn get_class(engine: &Engine<'a>) -> GcRef<Class<'a>> {
        engine.environment.integer
    }
}

impl<'a> PrimitiveClass<'a> for f64 {
    fn get_class(engine: &Engine<'a>) -> GcRef<Class<'a>> {
        engine.environment.float
    }
}

impl<'a> PrimitiveClass<'a> for String {
    fn get_class(engine: &Engine<'a>) -> GcRef<Class<'a>> {
        engine.environment.string
    }
}
