use crate::memory::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::primitives::function::FunctionImplementation;
use crate::runtime::r#return::{ Flow, JumpType, ReturnReference };
use crate::runtime::utilities::parameters::Parameters;
use crate::runtime::value::Value;
use crate::walker::ANode;
use crate::walker::nodes::ABlock;
use crate::walker::traits::WControl;

pub struct FunctionCode {
    block: Ref<ANode<ABlock>>,
}

impl FunctionCode {
    pub fn new(block: Ref<ANode<ABlock>>) -> Self {
        Self {
            block,
        }
    }
}

impl<'a> FunctionImplementation<'a> for FunctionCode {
    fn call(&self, engine: &mut Engine<'a>, parameters: &Parameters<'a>, arguments: &mut [Value<'a>]) -> ReturnReference<'a> {
        parameters.build(engine, arguments);
        let block = Ref::as_ref(&self.block);
        let flow = block.get().walk(engine)?;
        if let Some(reference) = flow.is_jump_reference(JumpType::Return) {
            if reference.is_defined() {
                Ok(engine.new_constant(reference.get_value()))
            } else {
                Ok(engine.undefined())
            }
        } else if let Flow::Reference(_) = flow {
            Ok(engine.undefined())
        } else {
            Err(error_jump())
        }
    }
}


fn error_jump() -> Error {
    Error::new_runtime("Incorrect jump use.")
}
