use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ ANode, WNode };
use crate::walker::nodes::{ ABlock, AControlTrait };

pub struct AIf {
    condition: WNode,
    then:      ANode<ABlock>,
    r#else:    Option<ANode<ABlock>>,
}

impl AIf {
    pub fn new(condition: WNode, then: ANode<ABlock>, r#else: Option<ANode<ABlock>>) -> Self {
        Self {
            condition,
            then,
            r#else,
        }
    }
}

impl AControlTrait for AIf {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        let reference = get!(engine.walk(&self.condition)?);
        let condition = reference.read()?.get_cast_boolean(engine)?;
        if condition {
            self.then.get().walk(engine)
        } else if let Some(r#else) = self.r#else.as_ref() {
            r#else.get().walk(engine)
        } else {
            Flow::new(engine.undefined())
        }
    }
}
