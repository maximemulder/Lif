use crate::ast::nodes::*;
use crate::memory::Ref;
use crate::runtime::bis::data::{Class, Function, Param, GcClass, Generic};
use crate::runtime::bis::engine::Engine;
use crate::runtime::bis::flow::{Res, ResValue, Flow};
use crate::runtime::bis::value::Value;

use std::collections::HashMap;

impl ADef {
    pub fn eval_def<'a>(&self, engine: &mut Engine<'a>) -> ResValue<'a> {
        if !get_generics(self).is_empty() {
            return make_generic(self, engine);
        }

        let value = match self {
            ADef::Class(class) => class.eval_def(engine),
            ADef::Function(function) => function.eval_def(engine),
        }?;

        engine.write(get_name(self), value);
        Ok(value)
    }
}

impl AClass {
    fn eval_def<'a>(&self, engine: &mut Engine<'a>) -> ResValue<'a> {
        let class = make_class(self, engine)?;
        engine.write(&self.name, class);
        Ok(class)
    }
}

impl AFunction {
    fn eval_def<'a>(&self, engine: &mut Engine<'a>) -> ResValue<'a> {
        let function = make_function(self, engine)?;
        engine.write(&self.name, function);
        Ok(function)
    }
}

fn get_name(node: &ADef) -> &str {
    match node {
        ADef::Class(class) => &class.name,
        ADef::Function(function) => &function.name,
    }
}

fn get_generics(node: &ADef) -> &[AGeneric] {
    match node {
        ADef::Class(class) => class.generics.as_ref(),
        ADef::Function(function) => function.generics.as_ref(),
    }
}

fn make_generic<'a>(node: &ADef, engine: &mut Engine<'a>) -> Res<Value<'a>> {
    let generics = get_generics(node).iter()
        .map(|generic| Ok(Param::new(&generic.name, read_type(&generic.constraint, engine)?)))
        .collect::<Res<Box<_>>>()?;

    let generic = Generic::new_node(engine.get_scope(), generics, Ref::new(node));
    Ok(engine.new_generic(generic))
}

fn make_class<'a>(node: &AClass, engine: &mut Engine<'a>) -> Res<Value<'a>> {
    let methods = node.methods.iter()
        .map(|method| Ok(make_function(method, engine)?))
        .collect::<Res<Box<_>>>()?
        .iter()
        .copied()
        .map(|method| (method.as_function().name.clone(), method))
        .collect::<HashMap<_, _>>();

    let parent = read_parent(&node.parent, engine)?;
    let class = Class::new(&node.name, Some(parent), methods, HashMap::new());
    Ok(engine.new_class(class))
}

fn make_function<'a>(node: &AFunction, engine: &mut Engine<'a>) -> Res<Value<'a>> {
    let params = node.params.iter()
        .map(|param| make_parameter(param, engine))
        .collect::<Res<Box<[_]>>>()?;
    let rest = if let Some(rest) = node.rest.as_ref() {
        Some(make_parameter(rest, engine)?)
    } else {
        None
    };

    let ret = read_type(&node.ret, engine)?;
    let function = Function::new_block(&node.name, engine.get_scope(), params, rest, ret, Ref::new(&node.body));
    Ok(engine.new_function(function))
}

fn make_parameter<'a>(node: &AParameter, engine: &mut Engine<'a>) -> Res<Param<'a>> {
    Ok(Param::new(&node.name,  read_type(&node.r#type, engine)?))
}

fn read_type<'a>(r#type: &Option<Box<AExpr>>, engine: &mut Engine<'a>) -> Res<GcClass<'a>> {
    if let Some(r#type) = r#type {
        match r#type.as_ref() {
            AExpr::Ident(ident) => Ok(engine.read(ident.pos, &ident.ident)?.as_class()),
            AExpr::Apply(apply) => Ok(match apply.eval(engine)? {
                Flow::Value(value) => value,
                _ => panic!("TODO panic"),
            }.as_class()),
            _ => todo!("TODO panic"),
        }
    } else {
        Ok(engine.env.any)
    }
}

fn read_parent<'a>(r#type: &Option<Box<AExpr>>, engine: &mut Engine<'a>) -> Res<GcClass<'a>> {
    if let Some(r#type) = r#type {
        match r#type.as_ref() {
            AExpr::Ident(ident) => Ok(engine.read(ident.pos, &ident.ident)?.as_class()),
            _ => todo!("TODO panic"),
        }
    } else {
        Ok(engine.env.object)
    }
}
