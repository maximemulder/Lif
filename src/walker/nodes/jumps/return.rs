use crate::runtime::r#return::Jump;
use crate::walker::ANode;
use crate::walker::nodes::{ AExpression, AJumpTrait };

pub struct AReturn {
    expression: Option<ANode<AExpression>>
}

impl AReturn {
    pub fn new(expression: Option<ANode<AExpression>>) -> Self {
        Self {
            expression,
        }
    }
}

impl AJumpTrait for AReturn {
    fn jump(&self) -> Jump {
        Jump::Return
    }

    fn expression(&self) -> Option<&ANode<AExpression>> {
        self.expression.as_ref()
    }
}
