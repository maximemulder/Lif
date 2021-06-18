use crate::runtime::engine::Engine;
use crate::runtime::r#return::{ Flow, Jump, Return, ReturnFlow };
use crate::runtime::value::GcValue;
use crate::walker::WNode;

pub fn new_jump<'a>(engine: &mut Engine<'a>, jump: Jump, node: Option<&WNode>) -> ReturnFlow<'a> {
    let reference = if let Some(node) = node {
        let value = get!(engine.walk(node)?).read()?;
        engine.new_constant(value)
    } else {
        engine.undefined()
    };

    Flow::new_jump(reference, jump)
}

pub fn new_type<'a>(engine: &mut Engine<'a>, node: Option<&WNode>) -> Return<Option<GcValue<'a>>> {
    node.map(|r#type| engine.walk(r#type)?.none()?.read()).transpose()
}
