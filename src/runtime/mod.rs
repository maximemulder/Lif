pub mod engine;
pub mod engine_data;
pub mod object;
pub mod primitives;
pub mod scope;

pub use engine::Engine;
pub use engine_data::EngineData;
pub use object::Object;

#[derive(Clone,Copy,Eq,PartialEq)]
pub struct Reference(pub usize);

#[derive(Clone,Copy,Eq,PartialEq)]
pub struct Value(pub usize);

impl Value {
	pub fn new_undefined() -> Self {
		return Self(usize::MAX);
	}
}
