use crate::runtime::bis::data::GcClass;
use crate::runtime::gc::GcTrace;

pub struct Env<'a> {
    pub any:      GcClass<'a>,
    pub bool:     GcClass<'a>,
    pub class:    GcClass<'a>,
    pub float:    GcClass<'a>,
    pub function: GcClass<'a>,
    pub generic:  GcClass<'a>,
    pub int:      GcClass<'a>,
    pub list:     GcClass<'a>,
    pub method:   GcClass<'a>,
    pub object:   GcClass<'a>,
    pub string:   GcClass<'a>,
    pub void:     GcClass<'a>,
}

impl Env<'_> {
    pub fn new() -> Self {
        Self {
            any: GcClass::null(), bool: GcClass::null(), class: GcClass::null(),
            float: GcClass::null(), function: GcClass::null(), generic: GcClass::null(),
            int: GcClass::null(), list: GcClass::null(), method: GcClass::null(),
            object: GcClass::null(), string: GcClass::null(), void: GcClass::null(),
        }
    }
}

impl<'a> Env<'a> {
    fn get_classes_mut(&mut self) -> [&mut GcClass<'a>; 12] {
        [
            &mut self.any, &mut self.bool, &mut self.class, &mut self.float, &mut self.function,
            &mut self.generic, &mut self.int, &mut self.list, &mut self.method, &mut self.object,
            &mut self.string, &mut self.void
        ]
    }
}

impl GcTrace for Env<'_> {
    fn trace(&mut self) {
        for class in self.get_classes_mut() {
            class.trace()
        }
    }
}
