use crate::nodes::Node;
use crate::nodes::block::Block;
use crate::nodes::expression::Expression;
use crate::runtime::data::{ Class, Data, Function, Instance, Primitive };
use crate::runtime::environment::Environment;
use crate::runtime::gc::{ Gc, GcRef, GcTraceable };
use crate::runtime::reference::{ GcReference, Reference };
use crate::runtime::scope::{ GcScope, Scope };
use crate::runtime::value::{ GcValue, Value };

pub enum Control {
	Return,
	Break,
	Continue,
}

pub struct Engine<'a> {
	pub environment: Environment<'a>,
	pub scopes:      Gc<Scope<'a>>,
	pub references:  Gc<Reference<'a>>,
	pub values:      Gc<Value<'a>>,
	pub scope:       GcScope<'a>,
	pub registries:  Vec<Vec<GcReference<'a>>>,
	pub this:        Option<GcValue<'a>>,
	pub control:     Option<Control>,
}

impl<'a> Engine<'a> {
	pub fn new() -> Self {
		let mut engine = Self {
			environment: Environment::new(),
			scopes:      Gc::new(),
			references:  Gc::new(),
			values:      Gc::new(),
			scope:       GcRef::null(),
			registries:  Vec::new(),
			this:        None,
			control:     None,
		};

		engine.scope = engine.scopes.alloc(Scope::new());
		engine.registries.push(Vec::new());
		engine.populate();

		return engine;
	}

	pub fn push_scope(&mut self) {
		self.scope = self.scopes.alloc(Scope::new_child(self.scope));
	}

	pub fn pop_scope(&mut self) {
		if let Some(parent) = self.scope.parent {
			self.scope = parent;
		} else {
			panic!();
		}
	}

	pub fn push_frame(&mut self, frame: GcScope<'a>) -> GcScope<'a> {
		let scope = self.scope;
		self.scope = frame;
		return scope;
	}

	pub fn pop_frame(&mut self, frame: GcScope<'a>) {
		self.scope = frame;
	}

	pub fn add_variable(&mut self, name: &str, reference: GcReference<'a>) {
		self.scope.add_variable(name, reference);
	}

	pub fn get_variable(&self, name: &str) -> GcReference<'a> {
		let mut scope = self.scope;
		loop {
			if let Some(object) = scope.get_variable(name) {
				return object;
			}

			if let Some(parent) = scope.parent {
				scope = parent;
			} else {
				panic!();
			}
		}
	}

	pub fn call_method(&mut self, value: GcValue<'a>, name: &str, mut arguments: Vec<GcValue<'a>>) -> GcReference<'a> {
		arguments.insert(0, value);
		return self.call(value.get_method(self, name).unwrap().read(), arguments);
	}

	pub fn call_method_self(&mut self, value: GcValue<'a>, name: &str, arguments: Vec<GcValue<'a>>) -> GcReference<'a> {
		return self.call(value.get_method(self, name).unwrap().read(), arguments);
	}

	pub fn call(&mut self, value: GcValue<'a>, mut arguments: Vec<GcValue<'a>>) -> GcReference<'a> {
		if let Some(this) = self.this {
			arguments.insert(0, this);
			self.this = None;
		}

		let callable = value.data_callable().duplicate();
		return callable.call(self, arguments);
	}

	pub fn collect(&mut self) {
		self.trace();
		self.scopes.collect();
		self.references.collect();
		self.values.collect();
	}

	pub fn execute(&mut self, node: &'a dyn Node) -> GcReference<'a> {
		self.registries.push(Vec::new());
		let reference = node.execute(self);
		let index = self.registries.len() - 2;
		self.registries[index].push(reference);
		self.registries.pop();
		return reference;
	}

	pub fn new_control(&mut self, control: Control, node: &'a Option<Expression>) -> GcReference<'a> {
		self.control = Some(control);
		return if let Some(node) = node {
			self.execute(node)
		} else {
			self.new_undefined()
		};
	}
}

impl GcTraceable for Engine<'_> {
	fn trace(&mut self) {
		self.environment.trace();
		self.scope.trace();
		for registries in self.registries.iter_mut() {
			for registry in registries.iter_mut() {
				registry.trace();
			}
		}

		if let Some(this) = &mut self.this {
			this.trace();
		}
	}
}

impl<'a> Engine<'a> {
	pub fn new_value(&mut self, class: GcValue<'a>, data: Data<'a>) -> GcValue<'a> {
		return self.values.alloc(Value::new(class, data));
	}

	pub fn new_undefined(&mut self) -> GcReference<'a> {
		return self.new_reference(None, false);
	}

	pub fn new_reference(&mut self, value: Option<GcValue<'a>>, variable: bool) -> GcReference<'a> {
		return self.references.alloc(Reference::new(value, variable));
	}

	pub fn new_constant_value(&mut self, class: GcValue<'a>, data: Data<'a>) -> GcReference<'a> {
		let value = self.new_value(class, data);
		return self.new_reference(Some(value), false);
	}
}

impl<'a> Engine<'a> {
	pub fn new_array(&mut self, elements: Vec<GcReference<'a>>) -> GcReference<'a> {
		return self.new_constant_value(self.environment.class, Data::Array(elements));
	}

	pub fn new_boolean(&mut self, boolean: bool) -> GcReference<'a> {
		return self.new_constant_value(self.environment.boolean, Data::Boolean(boolean));
	}

	pub fn new_class(&mut self) -> GcReference<'a> {
		return self.new_constant_value(self.environment.class, Data::Class(Class::new(Some(self.environment.object))));
	}

	pub fn new_instance(&mut self, parent: GcValue<'a>) -> GcReference<'a> {
		return self.new_constant_value(parent, Data::Instance(Instance::new()));
	}

	pub fn new_integer(&mut self, integer: usize) -> GcReference<'a> {
		return self.new_constant_value(self.environment.integer, Data::Integer(integer));
	}

	pub fn new_function(&mut self, parameters: &'a Vec<Box<str>>, block: &'a Block) -> GcReference<'a> {
		return self.new_constant_value(self.environment.function, Data::Callable(Box::new(Function::new(self.scope, parameters, block))));
	}

	pub fn new_primitive(&mut self, callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> GcReference<'a>) -> GcReference<'a> {
		return self.new_constant_value(self.environment.function, Data::Callable(Box::new(Primitive::new(callback))));
	}

	pub fn new_string(&mut self, string: String) -> GcReference<'a> {
		return self.new_constant_value(self.environment.string, Data::String(string));
	}
}
