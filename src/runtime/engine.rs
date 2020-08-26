use crate::nodes::block::Block;
use crate::runtime::data::{ Class, Data, Function, Instance, Primitive };
use crate::runtime::environment::Environment;
use crate::runtime::proxy::{ Proxy, Visitable };
use crate::runtime::reference::Reference;
use crate::runtime::scope::Scope;
use crate::runtime::value::Value;

pub struct Engine<'a> {
	pub environment: Environment<'a>,
	pub scopes:      Vec<Scope<'a>>,
	pub references:  Vec<Reference<'a>>,
	pub values:      Vec<Value<'a>>,
	pub scope:       Scope<'a>,
	pub this:        Option<Reference<'a>>,
}

impl<'a> Engine<'a> {
	pub fn new() -> Self {
		let mut engine = Self {
			environment: Environment::new(),
			scopes:      Vec::new(),
			references:  Vec::new(),
			values:      Vec::new(),
			scope:       Scope::new(),
			this:        None,
		};

		engine.scopes.push(engine.scope);
		engine.populate();

		return engine;
	}

	pub fn push_scope(&mut self) {
		self.scopes.push(Scope::new_child(self.scope));
		self.scope = self.scopes[self.scopes.len() - 1];
	}

	pub fn pop_scope(&mut self) {
		if let Some(parent) = self.scope.parent {
			self.scope = parent;
		} else {
			panic!();
		}
	}

	pub fn push_frame(&mut self, frame: Scope<'a>) -> Scope<'a> {
		let scope = self.scope;
		self.scope = frame;
		return scope;
	}

	pub fn pop_frame(&mut self, frame: Scope<'a>) {
		self.scope = frame;
	}

	pub fn new_variable(&mut self, name: &str, reference: Reference<'a>) {
		self.scope.add_variable(name, reference);
	}

	pub fn get_variable(&self, name: &str) -> Reference<'a> {
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

	pub fn call_method(&mut self, reference: Reference<'a>, name: &str, mut arguments: Vec<Reference<'a>>) -> Reference<'a> {
		arguments.insert(0, reference);
		return self.call(*reference.value_ref().get_method(self, name).unwrap().value_ref(), arguments);
	}

	pub fn call_method_self(&mut self, reference: Reference<'a>, name: &str, arguments: Vec<Reference<'a>>) -> Reference<'a> {
		return self.call(*reference.value_ref().get_method(self, name).unwrap().value_ref(), arguments);
	}

	pub fn call(&mut self, value: Value<'a>, mut arguments: Vec<Reference<'a>>) -> Reference<'a> {
		if let Some(this) = self.this {
			arguments.insert(0, this);
			self.this = None;
		}

		let callable = value.data_callable().duplicate();
		return callable.call(self, arguments);
	}

	pub fn collect(&mut self) {
		self.visit();

		self.scopes.drain_filter(|scope| {
			!Proxy::collect(scope)
		});

		self.references.drain_filter(|reference| {
			!Proxy::collect(reference)
		});

		self.values.drain_filter(|value| {
			!Proxy::collect(value)
		});
	}
}

impl Visitable for Engine<'_> {
	fn visit(&mut self) {
		self.environment.visit();
		self.scope.visit();
		if let Some(this) = &mut self.this {
			this.visit();
		}
	}
}

impl<'a> Engine<'a> {
	pub fn new_value(&mut self, class: Value<'a>, data: Data<'a>) -> Value<'a> {
		let value = Value::new(class, data);
		self.values.push(value);
		return value;
	}

	pub fn new_undefined(&mut self) -> Reference<'a> {
		let reference = Reference::new_undefined();
		self.references.push(reference);
		return reference;
	}

	pub fn new_reference(&mut self, value: Value<'a>) -> Reference<'a> {
		let reference = Reference::new(value);
		self.references.push(reference);
		return reference;
	}

	pub fn new_reference_value(&mut self, class: Value<'a>, data: Data<'a>) -> Reference<'a> {
		let value = self.new_value(class, data);
		return self.new_reference(value);
	}
}

impl<'a> Engine<'a> {
	pub fn new_array(&mut self, elements: Vec<Reference<'a>>) -> Reference<'a> {
		return self.new_reference_value(self.environment.class, Data::Array(elements));
	}

	pub fn new_boolean(&mut self, boolean: bool) -> Reference<'a> {
		return self.new_reference_value(self.environment.boolean, Data::Boolean(boolean));
	}

	pub fn new_class(&mut self) -> Reference<'a> {
		return self.new_reference_value(self.environment.class, Data::Class(Class::new(Some(self.environment.object))));
	}

	pub fn new_instance(&mut self, parent: Value<'a>) -> Reference<'a> {
		return self.new_reference_value(parent, Data::Instance(Instance::new()));
	}

	pub fn new_integer(&mut self, integer: usize) -> Reference<'a> {
		return self.new_reference_value(self.environment.integer, Data::Integer(integer));
	}

	pub fn new_function(&mut self, parameters: &'a Vec<Box<str>>, block: &'a Block) -> Reference<'a> {
		return self.new_reference_value(self.environment.function, Data::Callable(Box::new(Function::new(self.scope, parameters, block))));
	}

	pub fn new_primitive(&mut self, callback: &'a dyn Fn(&mut Engine<'a>, Vec<Reference<'a>>) -> Reference<'a>) -> Reference<'a> {
		return self.new_reference_value(self.environment.function, Data::Callable(Box::new(Primitive::new(callback))));
	}

	pub fn new_string(&mut self, string: String) -> Reference<'a> {
		return self.new_reference_value(self.environment.string, Data::String(string));
	}
}
