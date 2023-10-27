use crate::runtime::bis::env::Env;
use crate::runtime::bis::data::GcClass;

pub struct PrimClass {
    pub name: &'static str,
    pub parent: for<'a> fn(&Env<'a>) -> Option<GcClass<'a>>,
    pub env: for<'a, 'b> fn(&'b mut Env<'a>) -> &'b mut GcClass<'a>,
}

impl PrimClass {
    fn new(
        name: &'static str,
        parent: for<'a> fn(&Env<'a>) -> Option<GcClass<'a>>,
        env: for<'a, 'b> fn(&'b mut Env<'a>) -> &'b mut GcClass<'a>,
    ) -> Self {
        Self { name, parent, env }
    }
}

pub fn get_classes() -> [PrimClass; 12] {
    [
        PrimClass::new("Any",      |_|   None,          |env| &mut env.any),
        PrimClass::new("Bool",     |env| Some(env.any), |env| &mut env.bool),
        PrimClass::new("Class",    |env| Some(env.any), |env| &mut env.class),
        PrimClass::new("Float",    |env| Some(env.any), |env| &mut env.float),
        PrimClass::new("Function", |env| Some(env.any), |env| &mut env.function),
        PrimClass::new("Generic",  |env| Some(env.any), |env| &mut env.generic),
        PrimClass::new("Int",      |env| Some(env.any), |env| &mut env.int),
        PrimClass::new("List",     |env| Some(env.any), |env| &mut env.list),
        PrimClass::new("Method",   |env| Some(env.any), |env| &mut env.method),
        PrimClass::new("Object",   |env| Some(env.any), |env| &mut env.object),
        PrimClass::new("String",   |env| Some(env.any), |env| &mut env.string),
        PrimClass::new("Void",     |env| Some(env.any), |env| &mut env.void),
    ]
}
