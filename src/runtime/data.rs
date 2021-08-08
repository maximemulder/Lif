use crate::runtime::engine::Engine;
use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::primitives::{ Class, Generic };

use std::marker::PhantomData;

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
