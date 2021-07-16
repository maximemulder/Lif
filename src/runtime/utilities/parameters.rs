use crate::runtime::data::{ Array, Class };
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::gc::GcTrace;
use crate::runtime::r#return::Return;
use crate::runtime::utilities::variable::Variable;
use crate::runtime::value::GcValue;

pub struct Parameters<'a> {
    elements: Box<[Variable<'a>]>,
    rest: Option<Variable<'a>>,
}

impl<'a> Parameters<'a> {
    pub fn new(elements: Box<[Variable<'a>]>, rest: Option<Variable<'a>>) -> Self {
        Self {
            elements,
            rest,
        }
    }

    pub fn get_rest_array(&self, engine: &Engine<'a>) -> Option<GcValue<'a>> {
        self.rest.as_ref()
            .and_then(|parameter| parameter.r#type)
            .and_then(|class| class.is_generic(engine, engine.primitives.array).then_some(class))
    }

    pub fn get_rest_array_type(&self, engine: &Engine<'a>) -> Option<GcValue<'a>> {
        self.get_rest_array(engine).map(|class| class.get_ref::<Class>(engine).constructor().unwrap().arguments[0])
    }

    pub fn validate(&self, engine: &Engine<'a>, arguments: &[GcValue<'a>]) -> Return<()> {
        let condition = if self.rest.is_some() {
            arguments.len() < self.elements.len()
        } else {
            arguments.len() != self.elements.len()
        };

        if condition {
            return Err(error_arguments(self.elements.len(), arguments.len()));
        }

        for (parameter, argument) in self.elements.iter().zip(arguments.iter().copied()) {
            parameter.cast(engine, argument)?;
        }

        if let Some(r#type) = self.get_rest_array_type(engine) {
            for argument in arguments[self.elements.len()..].iter().copied() {
                argument.cast(engine, r#type)?;
            }
        }

        Ok(())
    }

    pub fn build(&self, engine: &mut Engine<'a>, arguments: &[GcValue<'a>]) {
        for (parameter, argument) in self.elements.iter().zip(arguments.iter().copied()) {
            parameter.build(engine).set_value(argument);
        }

        if let Some(parameter) = self.rest.as_ref() {
            let elements = arguments[self.elements.len()..].iter()
                .copied()
                .map(|argument| engine.new_reference(argument))
                .collect();

            let array = if let Some(class) = self.get_rest_array(engine) {
                engine.new_array_value(class, elements)
            } else {
                engine.new_array_any_value(elements)
            };

            parameter.build(engine).set_value(array);
        }
    }
}

impl GcTrace for Parameters<'_> {
    fn trace(&mut self) {
        for element in self.elements.iter_mut() {
            element.trace();
        }

        if let Some(rest) = self.rest.as_mut() {
            rest.trace();
        }
    }
}

pub fn length(arguments: usize, parameters: usize) -> Return<()> {
    if arguments != parameters {
        return Err(error_arguments(parameters, arguments));
    }

    Ok(())
}

pub fn pack<'a>(engine: &mut Engine<'a>, values: &mut [GcValue<'a>]) -> GcValue<'a> {
    let elements = values.iter()
        .copied()
        .map(|value| engine.new_constant(value))
        .collect();

    engine.new_array_any_value(elements)
}

pub fn unpack<'a>(engine: &Engine<'a>, value: GcValue<'a>) -> Return<Box<[GcValue<'a>]>> {
    value.get_ref::<Array>(engine).elements().iter()
        .copied()
        .map(|element| element.read())
        .collect()
}

fn error_arguments(parameters: usize, arguments: usize) -> Error {
    Error::new_runtime(&format!("Provided {} arguments while the function expects {} parameters.", arguments, parameters))
}
