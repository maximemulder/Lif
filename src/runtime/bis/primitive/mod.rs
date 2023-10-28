mod classes;
mod functions;
mod generics;
mod methods;
mod statics;

use crate::runtime::bis::data::{Class, Function, Generic, Param};
use crate::runtime::bis::engine::Engine;

use std::collections::HashMap;

pub fn populate(engine: &mut Engine) {
    let prim_classes = classes::get_classes();
    for prim_class in prim_classes.iter() {
        let parent = (prim_class.parent)(&engine.env);
        let class = Class::new(prim_class.name, parent, HashMap::new(), HashMap::new());
        let class = engine.alloc(class);
        *(prim_class.env)(&mut engine.env) = class;
    }

    for prim_class in prim_classes.iter() {
        let class = *(prim_class.env)(&mut engine.env);
        let value = engine.new_class_primitive(class);
        engine.write(prim_class.name, value);
    }

    let prim_generics = generics::get_generics(&mut engine.env);
    for prim_generic in prim_generics {
        let generic = Generic::new_primitive(engine.get_scope(), prim_generic.params, prim_generic.primitive);
        let generic = engine.alloc(generic);
        *(prim_generic.env)(&mut engine.env) = generic;
        let value = engine.new_generic_primitive(generic);
        engine.write(prim_generic.name, value);
    }

    let prim_mets_classes = methods::get_methods(&engine.env);
    for mut prim_mets in prim_mets_classes {
        for prim_met in prim_mets.methods.into_vec() {
            let mut params = Vec::new();
            params.push(Param::new("self", prim_mets.class));
            params.append(&mut prim_met.params.into_vec());
            let function = Function::new_primitive(
                prim_met.name,
                engine.get_scope(),
                params.into_boxed_slice(),
                prim_met.rest,
                prim_met.ret,
                prim_met.primitive
            );

            prim_mets.class.add_method(prim_met.name, engine.new_function(function));
        }
    }

    let prim_funs = functions::get_functions(&engine.env);
    for prim_fun in prim_funs {
        let function = Function::new_primitive(
            prim_fun.name,
            engine.get_scope(),
            prim_fun.params,
            prim_fun.rest,
            prim_fun.ret,
            prim_fun.primitive
        );

        let function = engine.new_function(function);
        engine.write(prim_fun.name, function);
    }

    let any = engine.new_class_primitive(engine.env.any);
    let list = engine.env.list;
    engine.env.list_any = list.apply(engine, &[any]).ok().unwrap().as_class();
}
