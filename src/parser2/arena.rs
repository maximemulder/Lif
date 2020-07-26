use std::mem::MaybeUninit;

pub struct Arena<T: Copy> {
	index: usize,
	elements: Vec<MaybeUninit<T>>,
}

impl<T: Copy> Arena<T> {
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

	pub fn insert(&mut self, index: usize, element: T) {
		while self.elements.len() <= index {
			self.elements.push(MaybeUninit::uninit());
		}

		self.elements[index] = MaybeUninit::new(element);
	}

	pub fn get(&self, index: usize) -> T {
		return unsafe { self.elements[index].assume_init() };
	}
}
