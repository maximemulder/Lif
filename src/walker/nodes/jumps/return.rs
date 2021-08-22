use crate::runtime::r#return::Jump;
use crate::walker::WNode;
use crate::walker::nodes::AJumpTrait;

pub struct AReturn {
    expression: Option<WNode>}

impl AReturn {
    pub fn new(expression: Option<WNode>) -> Self {
        Self {
            expression,
        }
    }
}

impl AJumpTrait for AReturn {
    fn jump(&self) -> Jump {
        Jump::Return
    }

    fn expression(&self) -> Option<&WNode> {
        self.expression.as_ref()
    }
}
