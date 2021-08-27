use crate::memory::Ref;
use crate::parser::CNode;
use crate::parser::elements;
use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Jump, ReturnFlow, ReturnJump, ReturnReference };
use crate::walker::{ ANode, SNode };
use crate::walker::nodes::{ AAssignment, ABinop, AChain, AJump, ALet, ALiteral, APreop, ASequence, AStructure };
use crate::walker::traits::{ WExecutable, WExpression, WStatement };

pub struct AExpression {
    expression: Box<SNode<dyn WExpression>>,
}

impl AExpression {
    pub fn new(expression: Box<SNode<dyn WExpression>>) -> Self {
        Self {
            expression,
        }
    }

    pub fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnFlow<'a> {
        engine.run_gc(|engine| self.expression.get().walk(engine))
    }
}

impl ANode for AExpression {
    fn build(node: Ref<CNode>) -> Self {
        let child = node.front(0);
        Self::new(match child.element {
            &elements::productions::STRUCTURE  => Box::new(SNode::<AStructure>::build(child)),
            &elements::expressions::JUMP       => Box::new(SNode::<AJump>::build(child)),
            &elements::expressions::LET        => Box::new(SNode::<ALet>::build(child)),
            &elements::expressions::LITERAL    => Box::new(SNode::<ALiteral>::build(child)),
            &elements::expressions::CHAIN      => Box::new(SNode::<AChain>::build(child)),
            &elements::expressions::SEQUENCE   => Box::new(SNode::<ASequence>::build(child)),
            &elements::expressions::BINOP      => Box::new(SNode::<ABinop>::build(child)),
            &elements::expressions::PREOP      => Box::new(SNode::<APreop>::build(child)),
            &elements::expressions::ASSIGNMENT => Box::new(SNode::<AAssignment>::build(child)),
            _ => panic!(),
        })
    }
}

impl WExecutable for AExpression {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnReference<'a> {
        self.walk(engine)?.get()
    }
}

impl WStatement for AExpression {
    fn walk<'a>(&self, engine: &mut Engine<'a>) -> ReturnJump<'a> {
        Jump::flow(self.walk(engine)?)
    }
}
