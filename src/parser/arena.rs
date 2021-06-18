use std::cell::{ Ref, RefCell, RefMut };
use std::marker::{ PhantomData, Unsize };

pub struct ArenaRef<T: ?Sized> {
    index: usize,
    phantom: PhantomData<T>,
}

impl<T: ?Sized> ArenaRef<T> {
    fn new(index: usize) -> Self {
        Self {
            index,
            phantom: PhantomData,
        }
    }
}

impl<T: ?Sized> Clone for ArenaRef<T> {
    fn clone(&self) -> Self {
        Self::new(self.index)
    }
}

impl<T: ?Sized> Copy for ArenaRef<T> {}

pub struct Arena<T: ?Sized> {
    elements: RefCell<Vec<Option<Box<T>>>>,
}

impl<T: ?Sized> Arena<T> {
    pub fn new() -> Self {
        Self {
            elements: RefCell::new(Vec::new()),
        }
    }

    pub fn declare(&self) -> ArenaRef<T> {
        let index = self.elements().len();
        self.elements_mut().push(None);
        ArenaRef::new(index)
    }

    pub fn define<N: Unsize<T>>(&self, element: N) -> ArenaRef<T> {
        let r#ref = self.declare();
        self.elements_mut()[r#ref.index] = Some(Box::<N>::new(element));
        r#ref
    }

    pub fn swap(&self, r#ref: ArenaRef<T>, other: ArenaRef<T>) {
        self.elements_mut().swap(r#ref.index, other.index);
    }

    pub fn get(&self, r#ref: ArenaRef<T>) -> Ref<T> {
        Ref::map(self.elements(), |elements| elements[r#ref.index].as_ref().unwrap().as_ref())
    }

    pub fn get_mut(&self, r#ref: ArenaRef<T>) -> RefMut<T> {
        RefMut::map(self.elements_mut(), |elements| elements[r#ref.index].as_mut().unwrap().as_mut())
    }

    fn elements(&self) -> Ref<Vec<Option<Box<T>>>> {
        self.elements.borrow()
    }

    fn elements_mut(&self) -> RefMut<Vec<Option<Box<T>>>> {
        self.elements.borrow_mut()
    }
}
