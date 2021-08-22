use crate::runtime::r#return::Jump;
use crate::walker::ANode;
use crate::walker::nodes::{ AExpression, AJumpTrait };

pub struct ABreak {
    expression: Option<ANode<AExpression>>,
}

impl ABreak {
    pub fn new(expression: Option<ANode<AExpression>>) -> Self {
        Self {
            expression,
        }
    }
}

impl AJumpTrait for ABreak {
    fn jump(&self) -> Jump {
        Jump::Break
    }

    fn expression(&self) -> Option<&ANode<AExpression>> {
        self.expression.as_ref()
    }
}
