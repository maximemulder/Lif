use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct Function<'a> {
    parameters: Box<[Node<'a>]>,
    r#type: Option<Node<'a>>,
    block: Node<'a>,
}

impl<'a> Function<'a> {
    pub fn new(parameters: Box<[Node<'a>]>, r#type: Option<Node<'a>>, block: Node<'a>) -> Self {
        return Self {
            parameters,
            r#type,
            block,
        };
    }
}

impl<'a> Executable<'a> for Function<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        let r#type = if let Some(r#type) = self.r#type.as_ref() {
            Some(r#type.execute(engine)?.read()?)
        } else {
            None
        };

        return Ok(engine.new_function(&self.parameters, r#type, &self.block));
    }
}
