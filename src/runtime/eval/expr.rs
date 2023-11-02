use crate::ast::Pos;
use crate::ast::nodes::*;
use crate::runtime::data::Ref;
use crate::runtime::engine::Engine;
use crate::runtime::flow::{Flow, FlowT, Jump, JumpKind, ResFlow, ResFlowT};
use super::r#type::read_type_any;

macro_rules! flow {
    ( $flow:expr ) => {{
        use crate::runtime::flow::FlowT;
        match $flow? {
            FlowT::None(value) => value,
            FlowT::Jump(jump) => return Ok(FlowT::Jump(jump)),
        }
    }}
}

pub(crate) use flow;

macro_rules! jump_flow {
    ( $jump:expr ) => {{
        match $jump? {
            Some(Jump { pos, jump, value }) => return Flow::jump(pos, jump, value),
            None => (),
        }
    }}
}

impl AExpr {
    pub fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        match self {
            AExpr::Void(void)           => void.eval(engine),
            AExpr::Bool(bool)           => bool.eval(engine),
            AExpr::Int(int)             => int.eval(engine),
            AExpr::Float(float)         => float.eval(engine),
            AExpr::String(string)       => string.eval(engine),
            AExpr::Ident(ident)         => ident.eval(engine),
            AExpr::Var(var)             => var.eval(engine),
            AExpr::Chain(chain)         => chain.eval(engine),
            AExpr::Apply(apply)         => apply.eval(engine),
            AExpr::Call(call)           => call.eval(engine),
            AExpr::Continue(r#continue) => r#continue.eval(engine),
            AExpr::Break(r#break)       => r#break.eval(engine),
            AExpr::Return(r#return)     => r#return.eval(engine),
            AExpr::Preop(preop)         => preop.eval(engine),
            AExpr::Binop(binop)         => binop.eval(engine),
            AExpr::Or(or)               => or.eval(engine),
            AExpr::And(and)             => and.eval(engine),
            AExpr::Block(block)         => block.eval(engine),
            AExpr::If(r#if)             => r#if.eval(engine),
            AExpr::Loop(r#loop)         => r#loop.eval(engine),
            AExpr::While(r#while)       => r#while.eval(engine),
            AExpr::For(r#for)           => r#for.eval(engine),
            AExpr::Assign(assign)       => assign.eval(engine),
        }
    }

    pub fn pos(&self) -> Pos {
        match self {
            AExpr::Void     (node) => node.pos,
            AExpr::Bool     (node) => node.pos,
            AExpr::Int      (node) => node.pos,
            AExpr::Float    (node) => node.pos,
            AExpr::String   (node) => node.pos,
            AExpr::Ident    (node) => node.pos,
            AExpr::Var      (node) => node.pos,
            AExpr::Chain    (node) => node.pos,
            AExpr::Apply    (node) => node.pos,
            AExpr::Call     (node) => node.pos,
            AExpr::Continue (node) => node.pos,
            AExpr::Break    (node) => node.pos,
            AExpr::Return   (node) => node.pos,
            AExpr::Preop    (node) => node.pos,
            AExpr::Binop    (node) => node.pos,
            AExpr::Or       (node) => node.pos,
            AExpr::And      (node) => node.pos,
            AExpr::Block    (node) => node.pos,
            AExpr::If       (node) => node.pos,
            AExpr::Loop     (node) => node.pos,
            AExpr::While    (node) => node.pos,
            AExpr::For      (node) => node.pos,
            AExpr::Assign   (node) => node.pos,
        }
    }

    pub fn read<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let value = flow!(self.eval(engine));
        Flow::none(value.read(self.pos())?)
    }

    pub fn read_bool<'a>(&self, engine: &mut Engine<'a>) -> ResFlowT<'a, bool> {
        let value = flow!(self.read(engine));
        value.isa_type(self.pos(), engine.env.bool)?;
        FlowT::none(value.as_bool())
    }

    pub fn read_ref<'a>(&self, engine: &mut Engine<'a>) -> ResFlowT<'a, Ref<'a>> {
        let value = flow!(self.eval(engine));
        value.isa_type(self.pos(), engine.env.r#ref)?;
        FlowT::none(value.as_ref())
    }
}

impl AExprVoid {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        Flow::none(engine.new_void())
    }
}

impl AExprBool {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        Flow::none(engine.new_bool(self.bool))
    }
}

impl AExprInt {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let literal = self.literal.replace("_", "");
        let int = match literal.chars().nth(1) {
            Some('b') => i64::from_str_radix(&literal[2..], 2).unwrap(),
            Some('o') => i64::from_str_radix(&literal[2..], 8).unwrap(),
            Some('x') => i64::from_str_radix(&literal[2..], 16).unwrap(),
            _ => literal.parse::<i64>().unwrap(),
        };

        Flow::none(engine.new_int(int))
    }
}

impl AExprFloat {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let literal = self.literal.replace("_", "");
        let float = literal.parse::<f64>().unwrap();
        Flow::none(engine.new_float(float))
    }
}

impl AExprString {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        Flow::none(engine.new_string(&self.literal[1 .. self.literal.len() - 1]))
    }
}

impl AExprIdent {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let r#ref = engine.read(self.pos, &self.ident)?;
        let value = engine.new_ref(r#ref);
        Flow::none(value)
    }
}

impl AExprVar {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let r#type = read_type_any(&self.r#type, engine)?;
        engine.declare(&self.ident, r#type);
        let r#ref = engine.read(self.pos, &self.ident)?;
        let value = engine.new_ref(r#ref);
        Flow::none(value)
    }
}

impl AExprChain {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let value = flow!(self.expr.read(engine));
        let attr = engine.new_string(&self.member);
        Flow::none(value.call_method(engine, self.pos, "__cn__", &[attr])?)
    }
}

impl AExprApply {
    pub fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let generic = flow!(self.expr.read(engine)).as_generic();
        let mut args = Vec::new();
        for arg in self.args.iter() {
            args.push(flow!(arg.read(engine)).as_class())
        }

        Flow::none(engine.get_generic(self.pos, generic, args.into_boxed_slice())?)
    }
}

impl AExprCall {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let receiver = flow!(self.expr.read(engine));
        let function = receiver.class.get_method("__cl__").expect("TODO").as_function();
        let mut values = Vec::new();
        for arg in self.args.iter() {
            values.push(flow!(arg.read(engine)))
        }

        let args = engine.new_list(&values);
        Flow::none(function.call(engine, self.pos, &[receiver, args])?)
    }
}

impl AExprContinue {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let value = if let Some(expr) = self.expr.as_ref() {
            Some(flow!(expr.read(engine)))
        } else {
            None
        };

        Flow::jump(self.pos, JumpKind::Continue, value)
    }
}

impl AExprBreak {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let value = if let Some(expr) = self.expr.as_ref() {
            Some(flow!(expr.read(engine)))
        } else {
            None
        };

        Flow::jump(self.pos, JumpKind::Break, value)
    }
}

impl AExprReturn {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let value = if let Some(expr) = self.expr.as_ref() {
            Some(flow!(expr.read(engine)))
        } else {
            None
        };

        Flow::jump(self.pos, JumpKind::Return, value)
    }
}

impl AExprPreop {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let value = flow!(self.expr.read(engine));
        let name = match self.op.as_ref() {
            "~" => "__bnot__",
            "+" => "__pos__",
            "-" => "__neg__",
            "!" => "__not__",
            _   => panic!(),
        };

        Flow::none(value.call_method(engine, self.pos, name, &[])?)
    }
}

impl AExprBinop {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let left = flow!(self.left.read(engine));
        let right = flow!(self.right.read(engine));
        let name = match self.op.as_ref() {
            "==" => "__eq__",
            "!=" => "__ne__",
            "<" => "__lt__",
            ">" => "__gt__",
            "<=" => "__le__",
            ">=" => "__ge__",
            "+" => "__add__",
            "-" => "__sub__",
            "*" => "__mul__",
            "/" => "__div__",
            "%" => "__rem__",
            "&" => "__band__",
            "|" => "__bor__",
            "^" => "__bxor__",
            "<<" => "__bls__",
            ">>" => "__brs__",
            "<<<" => "__bcls__",
            ">>>" => "__bcrs__",
            _   => panic!(),
        };

        Flow::none(left.call_method(engine, self.pos, name, &[right])?)
    }
}

impl AExprOr {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        if flow!(self.left.read_bool(engine)) {
            Flow::none(engine.new_bool(true))
        } else {
            let right = flow!(self.right.read_bool(engine));
            Flow::none(engine.new_bool(right))
        }
    }
}

impl AExprAnd {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        if flow!(self.left.read_bool(engine)) {
            let right = flow!(self.right.read(engine)).as_bool();
            Flow::none(engine.new_bool(right))
        } else {
            Flow::none(engine.new_bool(false))
        }
    }
}

impl ABlock {
    pub fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        engine.with_scope(|engine| {
            for stmt in self.stmts.iter() {
                jump_flow!(stmt.eval_stmt(engine));
            }

            if let Some(expr) = self.expr.as_ref() {
                expr.read(engine)
            } else {
                Flow::none(engine.new_void())
            }
        })
    }
}

impl AProgram {
    pub fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        for stmt in self.stmts.iter() {
            jump_flow!(stmt.eval_stmt(engine));
        }

        Flow::none(engine.new_void())
    }
}

impl AIf {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        if flow!(self.cond.read_bool(engine)) {
            self.then.eval(engine)
        } else if let Some(r#else) = self.r#else.as_ref() {
            r#else.eval(engine)
        } else {
            Flow::none(engine.new_void())
        }
    }
}

impl ALoop {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let mut values = Vec::new();
        loop {
            match self.body.eval(engine)? {
                Flow::None(value) => {
                    values.push(value)
                },
                Flow::Jump(Jump { jump: JumpKind::Continue, value, .. }) => {
                    if let Some(value) = value {
                        values.push(value);
                    }

                    continue;
                },
                Flow::Jump(Jump { jump: JumpKind::Break, value, .. }) => {
                    if let Some(value) = value {
                        values.push(value);
                    }

                    break;
                },
                Flow::Jump(jump) => {
                    return Ok(Flow::Jump(jump));
                },
            }
        }

        Flow::none(engine.new_list(&values))
    }
}

impl AWhile {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let mut values = Vec::new();
        while flow!(self.cond.read_bool(engine)) {
            match self.body.eval(engine)? {
                Flow::None(value) => {
                    values.push(value)
                },
                Flow::Jump(Jump { jump: JumpKind::Continue, value, .. }) => {
                    if let Some(value) = value {
                        values.push(value);
                    }

                    continue;
                },
                Flow::Jump(Jump { jump: JumpKind::Break, value, .. }) => {
                    if let Some(value) = value {
                        values.push(value);
                    }

                    break;
                },
                Flow::Jump(jump) => {
                    return Ok(Flow::Jump(jump));
                },
            }
        }

        Flow::none(engine.new_list(&values))
    }
}

impl AFor {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let mut values = Vec::new();
        engine.with_scope(|engine| {
            for element in flow!(self.list.read(engine)).as_list().values().iter().copied() {
                engine.write_value(&self.element, element);
                match self.body.eval(engine)? {
                    Flow::None(value) => {
                        values.push(value)
                    },
                    Flow::Jump(Jump { jump: JumpKind::Continue, value, .. }) => {
                        if let Some(value) = value {
                            values.push(value);
                        }

                        continue;
                    },
                    Flow::Jump(Jump { jump: JumpKind::Break, value, .. }) => {
                        if let Some(value) = value {
                            values.push(value);
                        }

                        break;
                    },
                    Flow::Jump(jump) => {
                        return Ok(Flow::Jump(jump));
                    },
                }
            }

            Flow::none(engine.new_list(&values))
        })
    }
}

impl AExprAssign {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let value = flow!(self.right.read(engine));
        let mut r#ref = flow!(self.left.read_ref(engine));
        r#ref.write(self.pos, value)?;
        Flow::none(value)
    }
}
