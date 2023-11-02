use crate::runtime::data::{GcClass, GcGeneric};
use crate::runtime::gc::GcTrace;

pub struct Env<'a> {
    pub list:     GcGeneric<'a>,
    pub list_any: GcClass<'a>,
    pub any:      GcClass<'a>,
    pub bool:     GcClass<'a>,
    pub class:    GcClass<'a>,
    pub float:    GcClass<'a>,
    pub function: GcClass<'a>,
    pub generic:  GcClass<'a>,
    pub int:      GcClass<'a>,
    pub method:   GcClass<'a>,
    pub object:   GcClass<'a>,
    pub r#ref:    GcClass<'a>,
    pub string:   GcClass<'a>,
    pub void:     GcClass<'a>,
}

impl Env<'_> {
    pub fn new() -> Self {
        Self {
            list: GcGeneric::null(),
            any: GcClass::null(), bool: GcClass::null(), class: GcClass::null(),
            float: GcClass::null(), function: GcClass::null(), generic: GcClass::null(),
            int: GcClass::null(), method: GcClass::null(), list_any: GcClass::null(),
            object: GcClass::null(), string: GcClass::null(), r#ref: GcClass::null(),
            void: GcClass::null(),
        }
    }
}

impl<'a> Env<'a> {
    fn get_classes_mut(&mut self) -> [&mut GcClass<'a>; 13] {
        [
            &mut self.list_any,
            &mut self.any, &mut self.bool, &mut self.class, &mut self.float, &mut self.function,
            &mut self.generic, &mut self.int, &mut self.method, &mut self.object, &mut self.r#ref,
            &mut self.string, &mut self.void,
        ]
    }

    fn get_generics_mut(&mut self) -> [&mut GcGeneric<'a>; 1] {
        [ &mut self.list ]
    }
}

impl GcTrace for Env<'_> {
    fn trace(&mut self) {
        for class in self.get_classes_mut() {
            class.trace()
        }
    }
}
