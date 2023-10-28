use crate::ast::nodes::*;
use crate::runtime::bis::engine::Engine;
use crate::runtime::bis::flow::{Flow, Jump, JumpKind, ResFlow};

macro_rules! flow {
    ( $flow:expr ) => {{
        let flow = $flow?;
        if let Flow::Value(value) = flow {
            value
        } else {
            return Ok(flow);
        }
    }}
}

pub(crate) use flow;

macro_rules! jump_flow {
    ( $jump:expr ) => {{
        let jump = $jump?;
        match jump {
            Some(Jump { jump, value }) => return Flow::jump(jump, value),
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
}

impl AExprVoid {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        Flow::value(engine.new_void())
    }
}

impl AExprBool {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        Flow::value(engine.new_bool(self.bool))
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

        Flow::value(engine.new_int(int))
    }
}

impl AExprFloat {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let literal = self.literal.replace("_", "");
        let float = literal.parse::<f64>().unwrap();
        Flow::value(engine.new_float(float))
    }
}

impl AExprString {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        Flow::value(engine.new_string(&self.literal[1 .. self.literal.len() - 1]))
    }
}

impl AExprIdent {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        Flow::value(engine.read(self.pos, &self.ident)?)
    }
}

impl AExprVar {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let void = engine.new_void();
        engine.write(&self.ident, void);
        Flow::value(void)
    }
}

impl AExprChain {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let value = flow!(self.expr.eval(engine));
        let attr = engine.new_string(&self.member);
        Flow::value(value.call_method(engine, "__cn__", &[attr])?)
    }
}

impl AExprApply {
    pub fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let generic = flow!(self.expr.eval(engine)).as_generic();
        let mut args = Vec::new();
        for arg in self.args.iter() {
            args.push(flow!(arg.eval(engine)))
        }

        Flow::value(generic.apply(engine, &args)?)
    }
}

impl AExprCall {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let receiver = flow!(self.expr.eval(engine));
        let function = receiver.class.get_method("__cl__").expect("TODO").as_function();
        let mut values = Vec::new();
        for arg in self.args.iter() {
            values.push(flow!(arg.eval(engine)))
        }

        let args = engine.new_list(values);
        Flow::value(function.call(engine, &[receiver, args])?)
    }
}

impl AExprContinue {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let value = if let Some(expr) = self.expr.as_ref() {
            Some(flow!(expr.eval(engine)))
        } else {
            None
        };

        Flow::jump(JumpKind::Continue, value)
    }
}

impl AExprBreak {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let value = if let Some(expr) = self.expr.as_ref() {
            Some(flow!(expr.eval(engine)))
        } else {
            None
        };

        Flow::jump(JumpKind::Break, value)
    }
}

impl AExprReturn {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let value = if let Some(expr) = self.expr.as_ref() {
            Some(flow!(expr.eval(engine)))
        } else {
            None
        };

        Flow::jump(JumpKind::Return, value)
    }
}

impl AExprPreop {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let value = flow!(self.expr.eval(engine));
        let name = match self.op.as_ref() {
            "~" => "__bnot__",
            "+" => "__pos__",
            "-" => "__neg__",
            "!" => "__not__",
            _   => panic!(),
        };

        Flow::value(value.call_method(engine, name, &[])?)
    }
}

impl AExprBinop {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let left = flow!(self.left.eval(engine));
        let right = flow!(self.right.eval(engine));
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

        Flow::value(left.call_method(engine, name, &[right])?)
    }
}

impl AExprOr {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        if flow!(self.left.eval(engine)).as_bool() {
            Flow::value(engine.new_bool(true))
        } else {
            let right = flow!(self.right.eval(engine)).as_bool();
            Flow::value(engine.new_bool(right))
        }
    }
}

impl AExprAnd {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        if flow!(self.left.eval(engine)).as_bool() {
            let right = flow!(self.right.eval(engine)).as_bool();
            Flow::value(engine.new_bool(right))
        } else {
            Flow::value(engine.new_bool(false))
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
                expr.eval(engine)
            } else {
                Flow::value(engine.new_void())
            }
        })
    }
}

impl AProgram {
    pub fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        for stmt in self.stmts.iter() {
            jump_flow!(stmt.eval_stmt(engine));
        }

        Flow::value(engine.new_void())
    }
}

impl AIf {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        if flow!(self.cond.eval(engine)).as_bool() {
            self.then.eval(engine)
        } else if let Some(r#else) = self.r#else.as_ref() {
            r#else.eval(engine)
        } else {
            Flow::value(engine.new_void())
        }
    }
}

impl ALoop {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let mut values = Vec::new();
        loop {
            match self.body.eval(engine)? {
                Flow::Value(value) => {
                    values.push(value)
                },
                Flow::Jump(Jump { jump: JumpKind::Continue, value }) => {
                    if let Some(value) = value {
                        values.push(value);
                    }

                    continue;
                },
                Flow::Jump(Jump { jump: JumpKind::Break, value }) => {
                    if let Some(value) = value {
                        values.push(value);
                    }

                    break;
                },
                Flow::Jump(Jump { jump: JumpKind::Return, value }) => {
                    return Flow::jump(JumpKind::Return, value);
                },
            }
        }

        Flow::value(engine.new_list(values))
    }
}

impl AWhile {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let mut values = Vec::new();
        while flow!(self.cond.eval(engine)).as_bool() {
            match self.body.eval(engine)? {
                Flow::Value(value) => {
                    values.push(value)
                },
                Flow::Jump(Jump { jump: JumpKind::Continue, value }) => {
                    if let Some(value) = value {
                        values.push(value);
                    }

                    continue;
                },
                Flow::Jump(Jump { jump: JumpKind::Break, value }) => {
                    if let Some(value) = value {
                        values.push(value);
                    }

                    break;
                },
                Flow::Jump(Jump { jump: JumpKind::Return, value }) => {
                    return Flow::jump(JumpKind::Return, value);
                },
            }
        }

        Flow::value(engine.new_list(values))
    }
}

impl AFor {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let mut values = Vec::new();
        engine.with_scope(|engine| {
            for element in flow!(self.list.eval(engine)).as_list().iter() {
                engine.write(&self.element, element);
                match self.body.eval(engine)? {
                    Flow::Value(value) => {
                        values.push(value)
                    },
                    Flow::Jump(Jump { jump: JumpKind::Continue, value }) => {
                        if let Some(value) = value {
                            values.push(value);
                        }

                        continue;
                    },
                    Flow::Jump(Jump { jump: JumpKind::Break, value }) => {
                        if let Some(value) = value {
                            values.push(value);
                        }

                        break;
                    },
                    Flow::Jump(Jump { jump: JumpKind::Return, value }) => {
                        return Flow::jump(JumpKind::Return, value);
                    },
                }
            }

            Flow::value(engine.new_list(values))
        })
    }
}

impl AExprAssign {
    fn eval<'a>(&self, engine: &mut Engine<'a>) -> ResFlow<'a> {
        let value = flow!(self.right.eval(engine));
        self.left.write(engine, value)?;
        Flow::value(value)
    }
}
