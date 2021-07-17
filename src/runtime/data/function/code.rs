use crate::memory::Ref;
use crate::runtime::data::function::FunctionImplementation;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::r#return::{ Jump, ReturnReference };
use crate::runtime::utilities::parameters::Parameters;
use crate::runtime::value::Value;
use crate::walker::WNode;

pub struct FunctionCode {
    block: Ref<WNode>,
}

impl FunctionCode {
    pub fn new(block: Ref<WNode>) -> Self {
        Self {
            block,
        }
    }
}

impl<'a> FunctionImplementation<'a> for FunctionCode {
    fn call(&self, engine: &mut Engine<'a>, parameters: &Parameters<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
        parameters.build(engine, arguments);
        let executable = Ref::as_ref(&self.block);
        let flow = engine.walk(executable)?;
        if flow.jump == Jump::Return && flow.reference.is_defined() {
            Ok(engine.new_constant(flow.reference.get_value()))
        } else if flow.jump == Jump::None {
            Ok(engine.undefined())
        } else {
            Err(error_jump())
        }
    }
}


fn error_jump() -> Error {
    Error::new_runtime("Incorrect jump use.")
}
