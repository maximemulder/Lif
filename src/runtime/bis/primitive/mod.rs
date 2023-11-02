mod classes;
mod functions;
mod generics;
mod methods;
mod statics;

use crate::ast::Pos;
use crate::runtime::bis::data::{Class, Generic};
use crate::runtime::bis::engine::Engine;

use std::collections::HashMap;

pub fn populate(engine: &mut Engine) {
    let prim_classes = classes::get_classes();
    for prim_class in prim_classes.iter() {
        let parent = (prim_class.parent)(&engine.env);
        let class = Class::new(prim_class.name, parent, Box::new([]), HashMap::new());
        let class = engine.alloc(class);
        *(prim_class.env)(&mut engine.env) = class;
    }

    for prim_class in prim_classes.iter() {
        let class = *(prim_class.env)(&mut engine.env);
        let value = engine.new_class_primitive(class);
        engine.write_value(prim_class.name, value);
    }

    let prim_generics = generics::get_generics(&mut engine.env);
    for prim_generic in prim_generics {
        let generic = Generic::new_primitive(prim_generic.name, engine.scope, prim_generic.params, prim_generic.primitive);
        let generic = engine.alloc(generic);
        *(prim_generic.env)(&mut engine.env) = generic;
        let value = engine.new_generic_primitive(generic);
        engine.write_value(prim_generic.name, value);
    }

    let prim_mets_classes = methods::get_methods(&engine.env);
    for mut prim_mets in prim_mets_classes {
        for prim_met in prim_mets.methods.into_vec() {
            let function = prim_met.to_method(engine, prim_mets.class);
            let value = engine.new_function(function);
            prim_mets.class.add_method(prim_met.name, value);
        }
    }

    let prim_funs = functions::get_functions(&engine.env);
    for prim_fun in prim_funs {
        let function = prim_fun.to_function(engine);
        let value = engine.new_function(function);
        engine.write_value(prim_fun.name, value);
    }

    let any = engine.new_class_primitive(engine.env.any);
    let list = engine.env.list;
    engine.env.list_any = engine.get_generic(Pos::DUMMY, list, Box::new([any.as_class()])).ok().unwrap().as_class();
}
