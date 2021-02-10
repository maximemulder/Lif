use crate::runtime::ReturnReference;
use crate::runtime::data::Tag;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::gc::GcTrace;
use crate::runtime::reference::GcReference;
use crate::runtime::scope::GcScope;
use crate::runtime::value::GcValue;

#[derive(Clone)]
pub struct GenericPrimitive<'a> {
    pub tag: Tag,
    scope: GcScope<'a>,
    parameters: Vec<Box<str>>,
    callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> ReturnReference<'a>,
    memoizes: Vec<(Box<[GcValue<'a>]>, GcReference<'a>)>,
}

impl<'a> GenericPrimitive<'a> {
    pub fn new(tag: Tag, scope: GcScope<'a>, parameters: Vec<Box<str>>, callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> ReturnReference<'a>) -> Self {
        Self {
            tag,
            scope,
            parameters,
            callback,
            memoizes: Vec::new(),
        }
    }

    pub fn call(&mut self, engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
        if arguments.len() != self.parameters.len() {
            return Err(Error::new_arguments(self.parameters.len(), arguments.len()));
        }

        'outer: for memoize in self.memoizes.iter() {
            for (value, argument) in memoize.0.iter().zip(arguments.iter()) {
                if !value.is(*argument) {
                    continue 'outer;
                }
            }

            return Ok(memoize.1);
        }

        engine.push_frame(self.scope);
        for (parameter, argument) in self.parameters.iter().zip(arguments.iter()) {
            let reference = engine.new_constant(*argument);
            engine.add_variable(parameter, reference);
        }

        let reference = (self.callback)(engine, arguments.clone())?;
        self.memoizes.push((arguments.into_boxed_slice(), reference));
        engine.pop_frame();
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
