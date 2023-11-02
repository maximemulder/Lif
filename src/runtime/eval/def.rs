use crate::ast::nodes::*;
use crate::memory::Ref;
use crate::runtime::{Engine, Value};
use crate::runtime::data::{Class, Function, Param, Generic};
use crate::runtime::flow::{Res, ResValue};

use std::collections::HashMap;

use super::r#type::{read_type, read_type_any};

impl ADef {
    pub fn eval_def<'a>(&self, engine: &mut Engine<'a>) -> ResValue<'a> {
        let value = if !get_generics(self).is_empty() {
            make_generic(self, engine)?
        } else {
            match self {
                ADef::Class(class) => class.eval_def(engine),
                ADef::Function(function) => function.eval_def(engine),
            }?
        };

        engine.write_value(get_name(self), value);
        Ok(value)
    }
}

impl AClass {
    fn eval_def<'a>(&self, engine: &mut Engine<'a>) -> ResValue<'a> {
        make_class(self, engine)
    }
}

impl AFunction {
    fn eval_def<'a>(&self, engine: &mut Engine<'a>) -> ResValue<'a> {
        make_function(self, engine)
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
        .map(|generic| Ok(Param::new(&generic.name, read_type_any(&generic.constraint, engine)?)))
        .collect::<Res<Box<_>>>()?;

    let generic = Generic::new_node(get_name(node), engine.scope, generics, Ref::new(node));
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

    let parent = read_type(&node.parent, engine)?.unwrap_or(engine.env.object);
    let class = Class::new(&node.name, Some(parent), Box::from(engine.frame().generics()), methods);
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

    let ret = read_type_any(&node.ret, engine)?;
    let function = Function::new_block(&node.name, engine.scope, params, rest, ret, Ref::new(&node.body));
    Ok(engine.new_function(function))
}

fn make_parameter<'a>(node: &AParameter, engine: &mut Engine<'a>) -> Res<Param<'a>> {
    Ok(Param::new(&node.name, read_type_any(&node.r#type, engine)?))
}
