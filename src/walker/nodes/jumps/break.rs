use crate::runtime::r#return::Jump;
use crate::walker::WNode;
use crate::walker::nodes::AJumpTrait;

pub struct ABreak {
    expression: Option<WNode>,
}

impl ABreak {
    pub fn new(expression: Option<WNode>) -> Self {
        Self {
            expression,
        }
    }
}

impl AJumpTrait for ABreak {
    fn jump(&self) -> Jump {
        Jump::Break
    }

    fn expression(&self) -> Option<&WNode> {
        self.expression.as_ref()
    }
}
