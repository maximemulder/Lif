use std::marker::Unsize;
use std::mem::MaybeUninit;
use crate::parser2::rule::Rule;

pub struct Arena<'a, 'b> {
	index: usize,
	elements: Vec<MaybeUninit<Box<dyn Rule<'a> + 'b>>>,
}

impl<'a, 'b> Arena<'a, 'b> {
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

	pub fn insert<T>(&mut self, index: usize, element: T) where T: Unsize<dyn Rule<'a> + 'b> {
		while self.elements.len() <= index {
			self.elements.push(MaybeUninit::uninit());
		}

		self.elements[index] = MaybeUninit::new(Box::<T>::new(element));
	}

	pub fn get(&'b self, index: usize) -> &'b dyn Rule<'a> {
		return unsafe { self.elements[index].get_ref().as_ref() };
	}

	pub fn get_mut(&'b mut self, index: usize) -> &'b mut dyn Rule<'a> {
		return unsafe { self.elements[index].get_mut().as_mut() };
	}
}
