use crate::runtime::data::{ Callable, Class, Data, Instance };
use crate::runtime::engine::Engine;
use crate::runtime::gc::{ GcRef, GcTraceable };
use crate::runtime::reference::Reference;

pub struct Value<'a> {
	pub class: GcRef<Value<'a>>,
	data: Data<'a>,
}

impl<'a> Value<'a> {
	pub fn new(class: GcRef<Value<'a>>, data: Data<'a>) -> Self {
		return Self {
			class,
			data,
		};
	}

	pub fn cast(&self, class: GcRef<Value<'a>>) {
		if self.class != class {
			panic!();
		}
	}

	pub fn get_cast_array(&self, engine: &Engine<'a>) -> &Vec<GcRef<Reference<'a>>> {
		self.cast(engine.environment.array);
		return self.data_array();
	}

	pub fn get_cast_boolean(&self, engine: &Engine<'a>) -> &bool {
		self.cast(engine.environment.boolean);
		return self.data_boolean();
	}

	pub fn get_cast_callable(&self, engine: &Engine<'a>) -> &Box<dyn Callable<'a> + 'a> {
		self.cast(engine.environment.function);
		return self.data_callable();
	}

	pub fn get_cast_string(&self, engine: &Engine<'a>) -> &String {
		self.cast(engine.environment.string);
		return self.data_string();
	}

	pub fn get_method(&self, engine: &Engine<'a>, name: &str) -> Option<GcRef<Reference<'a>>> {
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
	pub fn data_array(&self) -> &Vec<GcRef<Reference<'a>>> {
		data!(self, Array);
	}

	pub fn data_array_mut(&mut self) -> &mut Vec<GcRef<Reference<'a>>> {
		data_mut!(self, Array);
	}

	pub fn data_boolean(&self) -> &bool {
		data!(self, Boolean);
	}

	pub fn data_boolean_mut(&mut self) -> &mut bool {
		data_mut!(self, Boolean);
	}

	pub fn data_callable(&self) -> &Box<dyn Callable<'a> + 'a> {
		data!(self, Callable);
	}

	pub fn data_callable_mut(&mut self) -> &mut Box<dyn Callable<'a> + 'a> {
		data_mut!(self, Callable);
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
}
