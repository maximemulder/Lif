use std::marker::Unsize;
use std::mem::MaybeUninit;

pub struct Arena<T: ?Sized> {
	elements: Vec<MaybeUninit<Box<T>>>,
}

impl<T: ?Sized> Arena<T> {
	pub fn new() -> Self {
		return Self {
			elements: Vec::new(),
		};
	}

	pub fn reserve(&mut self) -> usize {
		let index = self.elements.len();
		self.elements.push(MaybeUninit::uninit());
		return index;
	}

	pub fn insert<N: Unsize<T>>(&mut self, index: usize, element: N) {
		self.elements[index] = MaybeUninit::new(Box::<N>::new(element));
	}

	pub fn push<N: Unsize<T>>(&mut self, element: N) -> usize {
		let index = self.reserve();
		self.insert(index, element);
		return index;
	}

	pub fn get(&self, index: usize) -> &T {
		return unsafe { self.elements[index].get_ref().as_ref() };
	}

	pub fn get_mut(&mut self, index: usize) -> &mut T {
		return unsafe { self.elements[index].get_mut().as_mut() };
	}
}
