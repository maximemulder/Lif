use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, ReturnFlow };
use crate::walker::ANode;
use crate::walker::nodes::ADeclaration;
use crate::walker::traits::WExpression;

pub struct ALet {
    declaration: ANode<ADeclaration>,
}

impl ALet {
    pub fn new(declaration: ANode<ADeclaration>) -> Self {
        Self {
            declaration,
        }
    }
}

impl WExpression for ALet {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        Flow::reference(self.declaration.get().walk(engine)?.build(engine))
    }
}
