use crate::runtime::ReturnReference;
use crate::runtime::data::Tag;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::gc::GcTrace;
use crate::runtime::value::GcValue;
use crate::runtime::reference::GcReference;

#[derive(Clone)]
pub struct GenericPrimitive<'a> {
    pub tag: Tag,
    parameters: usize,
    callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> ReturnReference<'a>,
    memoizes: Vec<(Box<[GcValue<'a>]>, GcReference<'a>)>,
}

impl<'a> GenericPrimitive<'a> {
    pub fn new(tag: Tag, parameters: usize, callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> ReturnReference<'a>) -> Self {
        Self {
            tag,
            parameters,
            callback,
            memoizes: Vec::new(),
        }
    }

    pub fn call(&mut self, engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
        if arguments.len() != self.parameters {
            return Err(Error::new_arguments(self.parameters, arguments.len()));
        }

        'outer: for memoize in self.memoizes.iter() {
            for (value, argument) in memoize.0.iter().zip(arguments.iter()) {
                if !value.is(*argument) {
                    continue 'outer;
                }
            }

            return Ok(memoize.1);
        }

        let reference = (self.callback)(engine, arguments.clone())?;
        self.memoizes.push((arguments.into_boxed_slice(), reference));
        Ok(reference)
    }
}

impl GcTrace for GenericPrimitive<'_> {
    fn trace(&mut self) {
        for memoize in self.memoizes.iter_mut() {
            memoize.1.trace();
            for value in memoize.0.iter_mut() {
                value.trace()
            }
        }
    }
}
