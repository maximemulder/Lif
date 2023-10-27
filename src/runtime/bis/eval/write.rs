use crate::ast::nodes::*;
use crate::runtime::bis::engine::Engine;
use crate::runtime::bis::eval::expr::flow;
use crate::runtime::bis::eval::stmt::flow_jump;
use crate::runtime::bis::flow::{Flow, Jump, ResFlow, ResJump};
use crate::runtime::bis::value::Value;

impl AExpr {
    pub fn write<'a>(&self, engine: &mut Engine<'a>, value: Value<'a>) -> ResJump<'a> {
        let mut r#ref = flow_jump!(self.read_ref(engine)).as_ref();
        r#ref.write(value);
        Jump::none()
    }

    fn read_ref<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        match self {
            AExpr::Ident(ident) => ident.read_ref(engine),
            AExpr::Var(var) => var.read_ref(engine),
            AExpr::Chain(chain) => chain.read_ref(engine),
            _ => todo!("error")
        }
    }
}

impl AExprIdent {
    fn read_ref<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let value = engine.read_ref(self.pos, &self.ident)?;
        Flow::value(value)
    }
}

impl AExprVar {
    fn read_ref<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let void = engine.new_void();
        engine.write(&self.ident, void);
        let value = engine.read_ref(self.pos, &self.ident)?;
        Flow::value(value)
    }
}

impl AExprChain {
    fn read_ref<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let mut object = flow!(self.expr.eval(engine)).as_object();
        let r#ref = object.get_attr_ref(self.member.as_ref());
        Flow::value(engine.new_ref(r#ref))
    }
}
