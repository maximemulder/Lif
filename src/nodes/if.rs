use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;

pub struct If<'a> {
    condition: Node<'a>,
    then:      Node<'a>,
    r#else:    Option<Node<'a>>,
}

impl<'a> If<'a> {
    pub fn new(condition: Node<'a>, then: Node<'a>, r#else: Option<Node<'a>>) -> Self {
        return Self {
            condition,
            then,
            r#else,
        };
    }
}

impl<'a> Executable<'a> for If<'a> {
    fn execute<'b>(&'b self, engine: &mut Engine<'a, 'b>) -> ReturnReference<'a, 'b> {
        return if {
            let reference = execute!(engine, &self.condition);
            *reference.read()?.get_cast_boolean(engine)?
        } {
            engine.execute(&self.then)
        } else if let Some(r#else) = self.r#else.as_ref() {
            engine.execute(r#else)
        } else {
            Ok(engine.undefined())
        }
    }
}
