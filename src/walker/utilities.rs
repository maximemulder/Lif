use crate::runtime::gc::GcRef;
use crate::runtime::engine::Engine;
use crate::runtime::primitives::Class;
use crate::runtime::r#return::Return;
use crate::walker::WNode;

pub fn new_type<'a>(engine: &mut Engine<'a>, node: Option<&WNode>) -> Return<Option<GcRef<Class<'a>>>> {
    node.map(|r#type| engine.walk(r#type)?.none()?.read()?.get_cast_class(engine)).transpose()
}
