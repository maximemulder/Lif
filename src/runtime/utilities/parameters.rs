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
            parameter.cast(argument)?;
        }

        if let Some(parameter) = self.rest.as_ref() {
            if let Some(r#type) = parameter.r#type.as_ref() {
                if r#type.is_generic(engine.primitives.array) {
                    let class = r#type.data_class().constructor.unwrap().arguments[0];
                    for argument in arguments[self.elements.len()..].iter().copied() {
                        argument.cast(class)?;
                    }
                }
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

            let value = if let Some(r#type) = parameter.r#type {
                if r#type.is_generic(engine.primitives.array) {
                    engine.new_array_value(r#type, elements)
                } else {
                    engine.new_array_any_value(elements)
                }
            } else {
                engine.new_array_any_value(elements)
            };

            parameter.build(engine).set_value(value);
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

pub fn unpack(value: GcValue<'_>) -> Return<Box<[GcValue<'_>]>> {
    value.data_array().elements().iter()
        .copied()
        .map(|element| element.read())
        .collect()
}

fn error_arguments(parameters: usize, arguments: usize) -> Error {
    Error::new_runtime(&format!("Provided {} arguments while the function expects {} parameters.", arguments, parameters))
}
