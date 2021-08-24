use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::ANode;
use crate::walker::nodes::{ ABlock, AExpression };
use crate::walker::traits::WControl;

pub struct AIf {
    condition: ANode<AExpression>,
    then:      ANode<ABlock>,
    r#else:    Option<ANode<ABlock>>,
}

impl AIf {
    pub fn new(condition: ANode<AExpression>, then: ANode<ABlock>, r#else: Option<ANode<ABlock>>) -> Self {
        Self {
            condition,
            then,
            r#else,
        }
    }
}

impl WControl for AIf {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let reference = flow!(self.condition.get().walk(engine)?);
        let condition = reference.read()?.get_cast_boolean(engine)?;
        if condition {
            self.then.get().walk(engine)
        } else if let Some(r#else) = self.r#else.as_ref() {
            r#else.get().walk(engine)
        } else {
            Flow::reference(engine.undefined())
        }
    }
}
