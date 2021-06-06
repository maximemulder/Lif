use crate::runtime::reference::GcReference;
use crate::runtime::r#return::jump::Jump;

pub struct Flow<'a> {
    pub reference: GcReference<'a>,
    pub jump: Jump,
}

impl<'a> Flow<'a> {
    pub fn new(reference: GcReference<'a>, jump: Jump) -> Self {
        Self {
            reference,
            jump,
        }
    }
}
