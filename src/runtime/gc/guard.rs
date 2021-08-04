use crate::runtime::gc::GcTrace;

use std::mem::transmute;
use std::ptr::{ from_raw_parts_mut, DynMetadata };

pub struct GcGuard {
    pub flag: bool,
    pub object: *mut (),
    metadata: *const (),
}

impl GcGuard {
    pub fn new<T: GcTrace>(object: T) -> Self {
        unsafe {
            let pointer: *mut dyn GcTrace = Box::into_raw(Box::new(object));
            let (object, metadata) = pointer.to_raw_parts();
            Self {
                flag: false,
                object,
                metadata: transmute::<DynMetadata<dyn GcTrace>, *const ()>(metadata),
            }
        }
    }

    fn object(&mut self) -> &mut dyn GcTrace {
        unsafe {
            let metadata = transmute::<*const (), DynMetadata<dyn GcTrace>>(self.metadata);
            from_raw_parts_mut::<dyn GcTrace>(self.object, metadata).as_mut().unwrap()
        }
    }
}

impl Drop for GcGuard {
    fn drop(&mut self) {
        unsafe {
            Box::<dyn GcTrace>::from_raw(self.object());
        }
    }
}

impl GcTrace for GcGuard {
    fn trace(&mut self) {
        if !self.flag {
            self.flag = true;
            self.object().trace();
        }
    }
}
