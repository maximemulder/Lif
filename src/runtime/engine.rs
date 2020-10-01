use crate::nodes::Node;
use crate::nodes::block::Block;
use crate::nodes::declaration::Declaration;
use crate::nodes::expression::Expression;
use crate::runtime::ReturnReference;
use crate::runtime::data::{ Class, Data, Function, Generic, Instance, Primitive };
use crate::runtime::environment::Environment;
use crate::runtime::error::Error;
use crate::runtime::gc::{ Gc, GcRef, GcTraceable };
use crate::runtime::reference::{ GcReference, Reference };
use crate::runtime::scope::{ GcScope, Scope };
use crate::runtime::value::{ GcValue, Value };

#[derive(PartialEq, Eq)]
pub enum Control {
	Return,
	Break,
	Continue,
}

pub struct Engine<'a, 'b> where 'a: 'b {
	pub environment: Environment<'a, 'b>,
	scopes:          Gc<Scope<'a, 'b>>,
	references:      Gc<Reference<'a, 'b>>,
	values:          Gc<Value<'a, 'b>>,
	registries:      Vec<Vec<GcReference<'a, 'b>>>,
	frames:          Vec<GcScope<'a, 'b>>,
	scope:           GcScope<'a, 'b>,
	undefined:       GcReference<'a, 'b>,
	control:         Option<Control>,
}

impl<'a, 'b> Engine<'a, 'b> {
	pub fn new() -> Self {
		let mut engine = Self {
			environment: Environment::new(),
			scopes:      Gc::new(),
			references:  Gc::new(),
			values:      Gc::new(),
			registries:  Vec::new(),
			frames:      Vec::new(),
			scope:       GcRef::null(),
			undefined:   GcRef::null(),
			control:     None,
		};

		engine.scope = engine.scopes.alloc(Scope::new());
		engine.undefined = engine.references.alloc(Reference::new_constant(None));
		engine.registries.push(Vec::new());
		engine.populate();
		return engine;
	}
}

impl<'a, 'b> Engine<'a, 'b> {
	pub fn push_scope(&mut self) {
		self.scope = self.scopes.alloc(Scope::new_child(self.scope));
	}

	pub fn pop_scope(&mut self) {
		self.scope = self.scope.parent.unwrap();
	}

	pub fn push_frame(&mut self, frame: GcScope<'a, 'b>) {
		self.frames.push(self.scope);
		self.scope = frame;
	}

	pub fn pop_frame(&mut self) {
		self.scope = self.frames.pop().unwrap();
	}
}

impl<'a, 'b> Engine<'a, 'b> {
	pub fn add_variable(&mut self, name: &str, reference: GcReference<'a, 'b>) {
		self.scope.add_variable(name, reference);
	}

	pub fn get_variable(&self, name: &str) -> ReturnReference<'a, 'b> {
		let mut scope = self.scope;
		loop {
			if let Some(object) = scope.get_variable(name) {
				return Ok(object);
			}

			if let Some(parent) = scope.parent {
				scope = parent;
			} else {
				return Err(Error::new_runtime("Variable does not exist."));
			}
		}
	}

	pub fn call_method(&mut self, value: GcValue<'a, 'b>, name: &str, mut arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
		arguments.insert(0, value);
		return self.call(value.get_method(self, name).unwrap(), arguments);
	}

	pub fn call_method_self(&mut self, value: GcValue<'a, 'b>, name: &str, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
		return self.call(value.get_method(self, name).unwrap(), arguments);
	}

	pub fn call(&mut self, value: GcValue<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
		let callable = value.data_callable().duplicate();
		return callable.execute(self, arguments);
	}

	pub fn collect(&mut self) {
		self.trace();
		self.scopes.collect();
		self.references.collect();
		self.values.collect();
	}

	pub fn execute(&mut self, node: &'b dyn Node<'a>) -> ReturnReference<'a, 'b> {
		self.registries.push(Vec::new());
		let reference = match node.execute(self) {
			Ok(reference) => reference,
			Err(mut error) => {
				if error.node.is_none() {
					error.node = Some(node.get_syntax_node());
				}

				return Err(error);
			},
		};

		let index = self.registries.len() - 2;
		self.registries[index].push(reference);
		self.registries.pop();
		return Ok(reference);
	}
}

impl<'a, 'b> Engine<'a, 'b> {
	pub fn control_new(&mut self, control: Control, node: &'b Option<Expression<'a>>) -> ReturnReference<'a, 'b> {
		let reference = if let Some(node) = node {
			let value = self.execute(node)?.read()?;
			self.new_constant(value)
		} else {
			self.undefined()
		};

		if self.control.is_none() {
			self.control = Some(control);
		}

		return Ok(reference);
	}

	pub fn control_none(&mut self) -> bool {
		return self.control.is_none();
	}

	pub fn control_is(&mut self, other: Control) -> bool {
		if let Some(control) = &self.control {
			if *control == other {
				return true;
			}
		}

		return false;
	}

	pub fn control_consume(&mut self, control: Control) -> bool {
		if self.control_is(control) {
			self.control = None;
			return true;
		}

		return false;
	}
}

impl GcTraceable for Engine<'_, '_> {
	fn trace(&mut self) {
		self.environment.trace();
		self.scope.trace();
		self.undefined.trace();
		for registries in self.registries.iter_mut() {
			for registry in registries.iter_mut() {
				registry.trace();
			}
		}

		for frame in self.frames.iter_mut() {
			frame.trace();
		}
	}
}

impl<'a, 'b> Engine<'a, 'b> {
	pub fn new_value(&mut self, class: GcValue<'a, 'b>, data: Data<'a, 'b>) -> GcValue<'a, 'b> {
		return self.values.alloc(Value::new(class, data));
	}

	pub fn new_reference(&mut self, value: GcValue<'a, 'b>) -> GcReference<'a, 'b> {
		return self.references.alloc(Reference::new_variable(Some(value), self.environment.object));
	}

	pub fn new_variable(&mut self, value: Option<GcValue<'a, 'b>>, r#type: GcValue<'a, 'b>) -> GcReference<'a, 'b> {
		return self.references.alloc(Reference::new_variable(value, r#type));
	}

	pub fn new_constant(&mut self, value: GcValue<'a, 'b>) -> GcReference<'a, 'b> {
		return self.references.alloc(Reference::new_constant(Some(value)));
	}

	pub fn new_constant_value(&mut self, class: GcValue<'a, 'b>, data: Data<'a, 'b>) -> GcReference<'a, 'b> {
		let value = self.new_value(class, data);
		return self.new_constant(value);
	}

	pub fn undefined(&mut self) -> GcReference<'a, 'b> {
		return self.undefined;
	}
}

impl<'a, 'b> Engine<'a, 'b> {
	pub fn new_array(&mut self, elements: Vec<GcReference<'a, 'b>>) -> GcReference<'a, 'b> {
		return self.new_constant_value(self.environment.array, Data::Array(elements));
	}

	pub fn new_boolean(&mut self, boolean: bool) -> GcReference<'a, 'b> {
		return self.new_constant_value(self.environment.boolean, Data::Boolean(boolean));
	}

	pub fn new_class(&mut self) -> GcReference<'a, 'b> {
		return self.new_constant_value(self.environment.class, Data::Class(Class::new(Some(self.environment.object))));
	}

	pub fn new_instance(&mut self, parent: GcValue<'a, 'b>) -> GcReference<'a, 'b> {
		return self.new_constant_value(parent, Data::Instance(Instance::new()));
	}

	pub fn new_integer(&mut self, integer: usize) -> GcReference<'a, 'b> {
		return self.new_constant_value(self.environment.integer, Data::Integer(integer));
	}

	pub fn new_function(&mut self, parameters: &'b Vec<Declaration<'a>>, r#type: Option<GcValue<'a, 'b>>, block: &'b Block<'a>) -> GcReference<'a, 'b> {
		return self.new_constant_value(self.environment.function, Data::Callable(Box::new(Function::new(self.scope, parameters, r#type, block))));
	}

	pub fn new_generic(&mut self, generics: &'b Vec<&'a str>, node: &'b dyn Node<'a>) -> GcReference<'a, 'b> {
		return self.new_constant_value(self.environment.generic, Data::Generic(Generic::new(generics, node)));
	}

	pub fn new_primitive(&mut self, callback: &'b dyn Fn(&mut Engine<'a, 'b>, Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b>) -> GcReference<'a, 'b> {
		return self.new_constant_value(self.environment.function, Data::Callable(Box::<Primitive<'a, 'b>>::new(Primitive::new(callback))));
	}

	pub fn new_string(&mut self, string: String) -> GcReference<'a, 'b> {
		return self.new_constant_value(self.environment.string, Data::String(string));
	}
}
