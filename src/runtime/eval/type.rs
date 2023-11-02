use crate::ast::nodes::AExpr;
use crate::runtime::Engine;
use crate::runtime::data::GcClass;
use crate::runtime::flow::{Flow, Res};

pub fn read_type<'a>(node: &Option<Box<AExpr>>, engine: &mut Engine<'a>) -> Res<Option<GcClass<'a>>> {
    Ok(if let Some(node) = node {
        match node.as_ref() {
            AExpr::Ident(ident) => Some(engine.read(ident.pos, &ident.ident)?.read(ident.pos)?.as_class()),
            AExpr::Apply(apply) => Some(match apply.eval(engine)? {
                Flow::None(value) => value,
                _ => panic!("TODO panic"),
            }.as_class()),
            _ => todo!("TODO panic"),
        }
    } else {
        None
    })
}

pub fn read_type_any<'a>(node: &Option<Box<AExpr>>, engine: &mut Engine<'a>) -> Res<GcClass<'a>> {
    if let Some(r#type) = read_type(node, engine)? {
        Ok(r#type)
    } else {
        Ok(engine.env.any)
    }
}
