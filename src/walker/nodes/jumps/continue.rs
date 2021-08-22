use crate::runtime::r#return::Jump;
use crate::walker::ANode;
use crate::walker::nodes::{ AExpression, AJumpTrait };

pub struct AContinue {
    expression: Option<ANode<AExpression>>,
}

impl AContinue {
    pub fn new(expression: Option<ANode<AExpression>>) -> Self {
        Self {
            expression,
        }
    }
}

impl AJumpTrait for AContinue {
    fn jump(&self) -> Jump {
        Jump::Continue
    }

    fn expression(&self) -> Option<&ANode<AExpression>> {
        self.expression.as_ref()
    }
}
