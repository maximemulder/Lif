use crate::runtime::ReturnReference;
use crate::runtime::data::{ Callable, Tag };
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::gc::GcTrace;
use crate::runtime::value::GcValue;

#[derive(Clone)]
pub struct Primitive<'a> {
    tag: Tag,
    parameters: Box<[GcValue<'a>]>,
    callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> ReturnReference<'a>,
}

impl<'a> Primitive<'a> {
    pub fn new(tag: Tag, parameters: Box<[GcValue<'a>]>, callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> ReturnReference<'a>) -> Self {
        Self {
            tag,
            parameters,
            callback,
        }
    }
}

impl<'a> Callable<'a> for Primitive<'a> {
    fn execute(&self, engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
        if arguments.len() != self.parameters.len() {
            return Err(Error::new_arguments(self.parameters.len(), arguments.len()));
        }

        for (parameter, argument) in self.parameters.iter().zip(&arguments) {
            argument.cast(*parameter)?;
        }

        (self.callback)(engine, arguments)
    }

    fn duplicate<'c>(&'c self) -> Box<dyn Callable<'a> + 'c> {
        Box::new(self.clone())
    }

    fn get_tag(&self) -> Tag {
        self.tag.clone()
    }
}

impl GcTrace for Primitive<'_> {
    fn trace(&mut self) {
        for parameter in self.parameters.iter_mut() {
            parameter.trace();
        }
    }
}
