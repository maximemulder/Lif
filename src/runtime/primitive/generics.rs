use std::collections::HashMap;

use crate::runtime::data::{Class, Param, GcClass, GcGeneric};
use crate::runtime::engine::Engine;
use crate::runtime::env::Env;
use crate::runtime::flow::ResValue;
use super::methods;
use super::statics;

pub struct PrimGeneric<'a> {
    pub name: &'static str,
    pub params: Box<[Param<'a>]>,
    pub primitive: for<'b> fn(&mut Engine<'b>, &[GcClass<'b>]) -> ResValue<'b>,
    pub env: for<'b> fn(&'b mut Env<'a>) -> &'b mut GcGeneric<'a>,
}

impl<'a> PrimGeneric<'a> {
    pub fn new<const N: usize>(
        name: &'static str,
        params: [(&'static str, GcClass<'a>); N],
        primitive: for<'b> fn(&mut Engine<'b>, &[GcClass<'b>]) -> ResValue<'b>,
        env: for<'b> fn(&'b mut Env<'a>) -> &'b mut GcGeneric<'a>,
    ) -> Self {
        let params = params.iter()
            .map(|param| Param::new(&param.0, param.1))
            .collect();

        Self { name, params, primitive, env }
    }
}

pub fn get_generics<'a>(env: &mut Env<'a>) -> [PrimGeneric<'a>; 1] {
    [
        PrimGeneric::new("List", [("T", env.any)], list, |env| &mut env.list),
    ]
}

fn list<'a>(engine: &mut Engine<'a>, args: &[GcClass<'a>]) -> ResValue<'a> {
    let mut class = engine.alloc(Class::new("List", Some(engine.env.any), Box::new([args[0]]), HashMap::new()));
    let prim_stats = statics::get_list_statics(&engine.env, class);
    for prim_stat in prim_stats {
        let function = prim_stat.to_function(engine);
        class.add_static(prim_stat.name, engine.new_function(function));
    }

    let prim_mets = methods::get_list_methods(&engine.env, class, args);
    for prim_met in prim_mets {
        let function = prim_met.to_method(engine, class);
        class.add_method(prim_met.name, engine.new_function(function));
    }

    Ok(engine.new_class_primitive(class))
}
