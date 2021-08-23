use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::ANode;
use crate::walker::nodes::{ AExpression, AStatements, AControlTrait };

pub struct ABlock {
    statements: ANode<AStatements>,
    expression: Option<ANode<AExpression>>,
}

impl ABlock {
    pub fn new(statements: ANode<AStatements>, expression: Option<ANode<AExpression>>) -> Self {
        Self {
            statements,
            expression,
        }
    }
}

impl AControlTrait for ABlock {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        engine.run_scope(|engine| {
            jump_flow!(self.statements.get().walk(engine)?);
            Flow::reference(if let Some(expression) = self.expression.as_ref() {
                flow!(expression.get().walk(engine)?)
            } else {
                engine.undefined()
            })
        })
    }
}
