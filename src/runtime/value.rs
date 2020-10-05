use crate::runtime::Return;
use crate::runtime::data::{ Callable, Class, Data, Generic, Object };
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::gc::{ GcRef, GcTraceable };
use crate::runtime::reference::GcReference;

pub type GcValue<'a, 'b> = GcRef<Value<'a, 'b>>;

pub struct Value<'a, 'b> {
	pub class: GcValue<'a, 'b>,
	data: Data<'a, 'b>,
}

impl<'a, 'b> Value<'a, 'b> {
	pub fn new(class: GcValue<'a, 'b>, data: Data<'a, 'b>) -> Self {
		return Self {
			class,
			data,
		};
	}

	pub fn isa(&self, other: GcValue<'a, 'b>) -> bool {
		let mut class = self.class;
		loop {
			if class == other {
				return true;
			}

			if let Some(parent) = class.data_class().parent {
				class = parent;
			} else {
				break;
			}
		}

		return false;
	}

	pub fn cast(&self, other: GcValue<'a, 'b>) -> Return<'a, ()> {
		return if self.isa(other) {
			Ok(())
		} else {
			Err(Error::new_cast(self, other))
		};
	}

	pub fn get_cast_array(&self, engine: &Engine<'a, 'b>) -> Return<'a, &Vec<GcReference<'a, 'b>>> {
		self.cast(engine.environment.array)?;
		return Ok(self.data_array());
	}

	pub fn get_cast_boolean(&self, engine: &Engine<'a, 'b>) -> Return<'a, &bool> {
		self.cast(engine.environment.boolean)?;
		return Ok(self.data_boolean());
	}

	pub fn get_cast_callable(&self, engine: &Engine<'a, 'b>) -> Return<'a, &dyn Callable<'a, 'b>> {
		self.cast(engine.environment.function)?;
		return Ok(self.data_callable());
	}

	pub fn get_cast_string(&self, engine: &Engine<'a, 'b>) -> Return<'a, &String> {
		self.cast(engine.environment.string)?;
		return Ok(self.data_string());
	}

	pub fn get_method(&self, engine: &Engine<'a, 'b>, name: &str) -> Option<GcValue<'a, 'b>> {
		return self.class.data_class().get_method(engine, name);
	}
}

impl GcTraceable for Value<'_, '_> {
	fn trace(&mut self) {
		self.class.trace();
		self.data.trace();
	}
}

macro_rules! data {
	( $this:expr, $variant:ident ) => {
		if let Data::$variant(variant) = &$this.data {
			return variant;
		}

		panic!();
	};
}

macro_rules! data_mut {
	( $this:expr, $variant:ident ) => {
		if let Data::$variant(variant) = &mut $this.data {
			return variant;
		}

		panic!();
	};
}

impl<'a, 'b> Value<'a, 'b> {
	pub fn data_array(&self) -> &Vec<GcReference<'a, 'b>> {
		data!(self, Array);
	}

	pub fn data_array_mut(&mut self) -> &mut Vec<GcReference<'a, 'b>> {
		data_mut!(self, Array);
	}

	pub fn data_boolean(&self) -> &bool {
		data!(self, Boolean);
	}

	pub fn data_boolean_mut(&mut self) -> &mut bool {
		data_mut!(self, Boolean);
	}

	pub fn data_class(&self) -> &Class<'a, 'b> {
		data!(self, Class);
	}

	pub fn data_class_mut(&mut self) -> &mut Class<'a, 'b> {
		data_mut!(self, Class);
	}

	pub fn data_generic(&self) -> &Generic<'a, 'b> {
		data!(self, Generic);
	}

	pub fn data_generic_mut(&mut self) -> &mut Generic<'a, 'b> {
		data_mut!(self, Generic);
	}

	pub fn data_object(&self) -> &Object<'a, 'b> {
		data!(self, Object);
	}

	pub fn data_object_mut(&mut self) -> &mut Object<'a, 'b> {
		data_mut!(self, Object);
	}

	pub fn data_integer(&self) -> &usize {
		data!(self, Integer);
	}

	pub fn data_integer_mut(&mut self) -> &mut usize {
		data_mut!(self, Integer);
	}

	pub fn data_string(&self) -> &String {
		data!(self, String);
	}

	pub fn data_string_mut(&mut self) -> &mut String {
		data_mut!(self, String);
	}

	pub fn data_callable(&self) -> &dyn Callable<'a, 'b> {
		if let Data::Callable(callable) = &self.data {
			return callable.as_ref();
		}

		panic!();
	}

	pub fn data_callable_mut(&mut self) -> &mut dyn Callable<'a, 'b> {
		if let Data::Callable(callable) = &mut self.data {
			return callable.as_mut();
		}

		panic!();
	}
}
