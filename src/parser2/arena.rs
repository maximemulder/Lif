use crate::parser2::matcher::Matcher;
use std::mem::MaybeUninit;

pub struct Arena<'a, 'b> {
	index: usize,
	matchers: Vec<MaybeUninit<&'b dyn Matcher<'a>>>,
}

impl<'a, 'b> Arena<'a, 'b> {
	pub fn new() -> Self {
		return Self {
			index: 0,
			matchers: Vec::new(),
		};
	}

	pub fn index(&mut self) -> usize {
		let index = self.index;
		self.index += 1;
		return index;
	}

	pub fn insert(&mut self, index: usize, matcher: &'b dyn Matcher<'a>) {
		while self.matchers.len() <= index {
			self.matchers.push(MaybeUninit::uninit());
		}

		self.matchers[index] = MaybeUninit::new(matcher);
	}

	pub fn get(&self, index: usize) -> &'b dyn Matcher<'a> {
		return unsafe { self.matchers[index].assume_init() };
	}
}
