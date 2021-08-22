use crate::runtime::r#return::Jump;
use crate::walker::WNode;
use crate::walker::nodes::AJumpTrait;

pub struct AContinue {
    expression: Option<WNode>,
}

impl AContinue {
    pub fn new(expression: Option<WNode>) -> Self {
        Self {
            expression,
        }
    }
}

impl AJumpTrait for AContinue {
    fn jump(&self) -> Jump {
        Jump::Continue
    }

    fn expression(&self) -> Option<&WNode> {
        self.expression.as_ref()
    }
}
