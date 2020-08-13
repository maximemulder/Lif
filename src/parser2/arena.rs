use std::cell::{ Ref, RefCell, RefMut };
use std::marker::Unsize;
use std::mem::MaybeUninit;

pub struct Arena<T: ?Sized> {
	elements: RefCell<Vec<MaybeUninit<Box<T>>>>,
}

impl<T: ?Sized> Arena<T> {
	pub fn new() -> Self {
		return Self {
			elements: RefCell::new(Vec::new()),
		};
	}

	pub fn declare(&self) -> usize {
		let index = self.elements.borrow().len();
		self.elements.borrow_mut().push(MaybeUninit::uninit());
		return index;
	}

	pub fn define<N: Unsize<T>>(&self, index: usize, element: N) {
		self.elements.borrow_mut()[index] = MaybeUninit::new(Box::<N>::new(element));
	}

	pub fn create<N: Unsize<T>>(&self, element: N) -> usize {
		let index = self.declare();
		self.define(index, element);
		return index;
	}

	pub fn get(&self, index: usize) -> Ref<Box<T>> {
		return Ref::map(self.elements.borrow(), |elements| unsafe { elements[index].get_ref() });
	}

	pub fn get_mut(&self, index: usize) -> RefMut<Box<T>> {
		return RefMut::map(self.elements.borrow_mut(), |elements| unsafe { elements[index].get_mut() });
	}
}
