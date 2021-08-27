use crate::memory::Ref;
use crate::parser::CNode;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::{ AExpression, AStatements };
use crate::walker::traits::WStructure;

pub struct ABlock {
    statements: SNode<AStatements>,
    expression: Option<SNode<AExpression>>,
}

impl ABlock {
    pub fn new(statements: SNode<AStatements>, expression: Option<SNode<AExpression>>) -> Self {
        Self {
            statements,
            expression,
        }
    }
}

impl ANode for ABlock {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(
            SNode::build(node.front(1)),
            (node.children().len() == 4).then(|| SNode::build(node.front(2)))
        )
    }
}

impl WStructure for ABlock {
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
