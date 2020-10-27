use std::mem::MaybeUninit;

pub struct ArenaRef<T> {
    pointer: *mut MaybeUninit<T>,
}

impl<T> ArenaRef<T> {
    pub fn new(pointer: usize) -> Self {
        Self {
            pointer: pointer as *mut MaybeUninit<T>,
        }
    }

    pub fn read(&self) -> &T {
        unsafe { self.pointer.as_ref().unwrap().assume_init_ref() }
    }

    pub fn write(&mut self, value: T) {
        unsafe { self.pointer.write(MaybeUninit::new(value)) };
    }
}

pub struct Arena {
    elements: Vec<usize>,
}

impl Arena {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    fn new_undefined<T>(&mut self) -> usize {
        let element = Box::into_raw(Box::<T>::new_uninit()) as usize;
        self.elements.push(element);
        element
    }

    pub fn declare<T>(&mut self) -> ArenaRef<T> {
        let element = self.new_undefined::<T>();
        ArenaRef::new(element)
    }

    pub fn define<T>(&mut self, value: T) ->  ArenaRef<T> {
        let mut reference = self.declare();
        reference.write(value);
        reference
    }
}
