use std::marker::Unsize;
use std::mem::MaybeUninit;

pub struct ArenaItem<T> {
    pointer: *mut MaybeUninit<T>,
}

impl<T> ArenaItem<T> {
    pub fn new(pointer: usize) -> Self {
        Self {
            pointer: pointer as *mut MaybeUninit<T>,
        }
    }

    pub fn read<N>(&self) -> ArenaRef<N> where T: Unsize<N> {
        ArenaRef::new(self.pointer as *mut MaybeUninit<N>)
    }

    pub fn write(&mut self, value: T) {
        unsafe { self.pointer.write(MaybeUninit::new(value)) };
    }
}

pub struct ArenaRef<T: ?Sized> {
    pointer: *mut T,
}

impl<T: Sized> ArenaRef<T> {
    pub fn new(pointer: *mut MaybeUninit<T>) -> Self {
        Self {
            // pointer: unsafe { std::mem::transmute::<*mut MaybeUninit<N>, *mut T>(pointer) }
            pointer: unsafe { pointer.as_mut().unwrap().as_mut_ptr() }
        }
    }

    pub fn read(&self) -> &T {
        unsafe { self.pointer.as_ref().unwrap() }
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

    pub fn declare<T>(&mut self) -> ArenaItem<T> {
        let element = self.new_undefined::<T>();
        ArenaItem::new(element)
    }

    pub fn define<T>(&mut self, value: T) ->  ArenaItem<T> {
        let mut reference = self.declare();
        reference.write(value);
        reference
    }
}
