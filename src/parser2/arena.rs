use std::marker::Unsize;
use std::mem::MaybeUninit;

pub struct Arena<T: ?Sized> {
	index: usize,
	elements: Vec<MaybeUninit<Box<T>>>,
}

impl<T: ?Sized> Arena<T> {
	pub fn new() -> Self {
		return Self {
			index: 0,
			elements: Vec::new(),
		};
	}

	pub fn index(&mut self) -> usize {
		let index = self.index;
		self.index += 1;
		return index;
	}

	pub fn insert<N: Unsize<T>>(&mut self, index: usize, element: N) {
		while self.elements.len() <= index {
			self.elements.push(MaybeUninit::uninit());
		}

		self.elements[index] = MaybeUninit::new(Box::<N>::new(element));
	}

	pub fn push<N: Unsize<T>>(&mut self, element: N) -> usize {
		let index = self.elements.len();
		self.elements.push(MaybeUninit::new(Box::<N>::new(element)));
		return index;
	}

	pub fn get(&self, index: usize) -> &T {
		return unsafe { self.elements[index].get_ref().as_ref() };
	}

	pub fn get_mut(&mut self, index: usize) -> &mut T {
		return unsafe { self.elements[index].get_mut().as_mut() };
	}
}
