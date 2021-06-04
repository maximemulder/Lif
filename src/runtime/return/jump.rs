use crate::runtime::reference::GcReference;
use crate::runtime::r#return::control::Control;

pub struct Jump<'a> {
    pub control: Control,
    pub reference: GcReference<'a>,
}

impl<'a> Jump<'a> {
    pub fn new(control: Control, reference: GcReference<'a>) -> Self {
        Self {
            control,
            reference,
        }
    }
}
