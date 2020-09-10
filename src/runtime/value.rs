use crate::runtime::Return;
use crate::runtime::data::{ Callable, Class, Data, Instance };
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::gc::{ GcRef, GcTraceable };
use crate::runtime::reference::GcReference;

pub type GcValue<'a> = GcRef<Value<'a>>;

pub struct Value<'a> {
	pub class: GcValue<'a>,
	data: Data<'a>,
}

impl<'a> Value<'a> {
	pub fn new(class: GcValue<'a>, data: Data<'a>) -> Self {
		return Self {
			class,
			data,
		};
	}

	pub fn isa(&self, other: GcValue<'a>) -> bool {
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

	pub fn cast(&self, other: GcValue<'a>) -> Return<()> {
		return if self.isa(other) {
			Ok(())
		} else {
			Err(Error::new_runtime("Value is not of the required type."))
		};
	}

	pub fn get_cast_array(&self, engine: &Engine<'a>) -> Return<&Vec<GcReference<'a>>> {
		self.cast(engine.environment.array)?;
		return Ok(self.data_array());
	}

	pub fn get_cast_boolean(&self, engine: &Engine<'a>) -> Return<&bool> {
		self.cast(engine.environment.boolean)?;
		return Ok(self.data_boolean());
	}

	pub fn get_cast_callable(&self, engine: &Engine<'a>) -> Return<&dyn Callable<'a>> {
		self.cast(engine.environment.function)?;
		return Ok(self.data_callable());
	}

	pub fn get_cast_string(&self, engine: &Engine<'a>) -> Return<&String> {
		self.cast(engine.environment.string)?;
		return Ok(self.data_string());
	}

	pub fn get_method(&self, engine: &Engine<'a>, name: &str) -> Option<GcReference<'a>> {
		return self.class.data_class().get_method(engine, name);
	}
}

impl GcTraceable for Value<'_> {
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

impl<'a> Value<'a> {
	pub fn data_array(&self) -> &Vec<GcReference<'a>> {
		data!(self, Array);
	}

	pub fn data_array_mut(&mut self) -> &mut Vec<GcReference<'a>> {
		data_mut!(self, Array);
	}

	pub fn data_boolean(&self) -> &bool {
		data!(self, Boolean);
	}

	pub fn data_boolean_mut(&mut self) -> &mut bool {
		data_mut!(self, Boolean);
	}

	pub fn data_class(&self) -> &Class<'a> {
		data!(self, Class);
	}

	pub fn data_class_mut(&mut self) -> &mut Class<'a> {
		data_mut!(self, Class);
	}

	pub fn data_instance(&self) -> &Instance<'a> {
		data!(self, Instance);
	}

	pub fn data_instance_mut(&mut self) -> &mut Instance<'a> {
		data_mut!(self, Instance);
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

	pub fn data_callable(&self) -> &dyn Callable<'a> {
		if let Data::Callable(callable) = &self.data {
			return callable.as_ref();
		}

		panic!();
	}

	pub fn data_callable_mut(&mut self) -> &mut dyn Callable<'a> {
		if let Data::Callable(callable) = &mut self.data {
			return callable.as_mut();
		}

		panic!();
	}
}
