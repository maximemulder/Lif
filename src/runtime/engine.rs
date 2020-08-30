use crate::nodes::Node;
use crate::nodes::block::Block;
use crate::nodes::expression::Expression;
use crate::runtime::data::{ Class, Data, Function, Instance, Primitive };
use crate::runtime::environment::Environment;
use crate::runtime::gc::{ GcRef, GcTraceable };
use crate::runtime::reference::Reference;
use crate::runtime::scope::Scope;
use crate::runtime::value::Value;

pub enum Control {
	Return,
	Break,
	Continue,
}

pub struct Engine<'a> {
	pub environment: Environment<'a>,
	pub scopes:      Vec<GcRef<Scope<'a>>>,
	pub references:  Vec<GcRef<Reference<'a>>>,
	pub values:      Vec<GcRef<Value<'a>>>,
	pub scope:       GcRef<Scope<'a>>,
	pub registries:  Vec<Vec<GcRef<Reference<'a>>>>,
	pub this:        Option<GcRef<Value<'a>>>,
	pub control:     Option<Control>,
}

impl<'a> Engine<'a> {
	pub fn new() -> Self {
		let mut engine = Self {
			environment: Environment::new(),
			scopes:      Vec::new(),
			references:  Vec::new(),
			values:      Vec::new(),
			scope:       GcRef::alloc(Scope::new()),
			registries:  Vec::new(),
			this:        None,
			control:     None,
		};

		engine.scopes.push(engine.scope);
		engine.registries.push(Vec::new());
		engine.populate();

		return engine;
	}

	pub fn push_scope(&mut self) {
		self.scopes.push(GcRef::alloc(Scope::new_child(self.scope)));
		self.scope = self.scopes[self.scopes.len() - 1];
	}

	pub fn pop_scope(&mut self) {
		if let Some(parent) = self.scope.parent {
			self.scope = parent;
		} else {
			panic!();
		}
	}

	pub fn push_frame(&mut self, frame: GcRef<Scope<'a>>) -> GcRef<Scope<'a>> {
		let scope = self.scope;
		self.scope = frame;
		return scope;
	}

	pub fn pop_frame(&mut self, frame: GcRef<Scope<'a>>) {
		self.scope = frame;
	}

	pub fn new_variable(&mut self, name: &str, reference: GcRef<Reference<'a>>) {
		self.scope.add_variable(name, reference);
	}

	pub fn get_variable(&self, name: &str) -> GcRef<Reference<'a>> {
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

	pub fn call_method(&mut self, value: GcRef<Value<'a>>, name: &str, mut arguments: Vec<GcRef<Value<'a>>>) -> GcRef<Reference<'a>> {
		arguments.insert(0, value);
		return self.call(value.get_method(self, name).unwrap().read(), arguments);
	}

	pub fn call_method_self(&mut self, value: GcRef<Value<'a>>, name: &str, arguments: Vec<GcRef<Value<'a>>>) -> GcRef<Reference<'a>> {
		return self.call(value.get_method(self, name).unwrap().read(), arguments);
	}

	pub fn call(&mut self, value: GcRef<Value<'a>>, mut arguments: Vec<GcRef<Value<'a>>>) -> GcRef<Reference<'a>> {
		if let Some(this) = self.this {
			arguments.insert(0, this);
			self.this = None;
		}

		let callable = value.data_callable().duplicate();
		return callable.call(self, arguments);
	}

	pub fn collect(&mut self) {
		self.trace();
		self.scopes.drain_filter(|scope| !scope.collect());
		self.references.drain_filter(|reference| !reference.collect());
		self.values.drain_filter(|value| !value.collect());
	}

	pub fn execute(&mut self, node: &'a dyn Node) -> GcRef<Reference<'a>> {
		self.registries.push(Vec::new());
		let reference = node.execute(self);
		let index = self.registries.len() - 2;
		self.registries[index].push(reference);
		self.registries.pop();
		return reference;
	}

	pub fn new_control(&mut self, control: Control, node: &'a Option<Expression>) -> GcRef<Reference<'a>> {
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
	pub fn new_value(&mut self, class: GcRef<Value<'a>>, data: Data<'a>) -> GcRef<Value<'a>> {
		let value = GcRef::alloc(Value::new(class, data));
		self.values.push(value);
		return value;
	}

	pub fn new_undefined(&mut self) -> GcRef<Reference<'a>> {
		let reference = GcRef::alloc(Reference::new_undefined());
		self.references.push(reference);
		return reference;
	}

	pub fn new_reference(&mut self, value: GcRef<Value<'a>>) -> GcRef<Reference<'a>> {
		let reference = GcRef::alloc(Reference::new(value));
		self.references.push(reference);
		return reference;
	}

	pub fn new_reference_value(&mut self, class: GcRef<Value<'a>>, data: Data<'a>) -> GcRef<Reference<'a>> {
		let value = self.new_value(class, data);
		return self.new_reference(value);
	}
}

impl<'a> Engine<'a> {
	pub fn new_array(&mut self, elements: Vec<GcRef<Reference<'a>>>) -> GcRef<Reference<'a>> {
		return self.new_reference_value(self.environment.class, Data::Array(elements));
	}

	pub fn new_boolean(&mut self, boolean: bool) -> GcRef<Reference<'a>> {
		return self.new_reference_value(self.environment.boolean, Data::Boolean(boolean));
	}

	pub fn new_class(&mut self) -> GcRef<Reference<'a>> {
		return self.new_reference_value(self.environment.class, Data::Class(Class::new(Some(self.environment.object))));
	}

	pub fn new_instance(&mut self, parent: GcRef<Value<'a>>) -> GcRef<Reference<'a>> {
		return self.new_reference_value(parent, Data::Instance(Instance::new()));
	}

	pub fn new_integer(&mut self, integer: usize) -> GcRef<Reference<'a>> {
		return self.new_reference_value(self.environment.integer, Data::Integer(integer));
	}

	pub fn new_function(&mut self, parameters: &'a Vec<Box<str>>, block: &'a Block) -> GcRef<Reference<'a>> {
		return self.new_reference_value(self.environment.function, Data::Callable(Box::new(Function::new(self.scope, parameters, block))));
	}

	pub fn new_primitive(&mut self, callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcRef<Value<'a>>>) -> GcRef<Reference<'a>>) -> GcRef<Reference<'a>> {
		return self.new_reference_value(self.environment.function, Data::Callable(Box::new(Primitive::new(callback))));
	}

	pub fn new_string(&mut self, string: String) -> GcRef<Reference<'a>> {
		return self.new_reference_value(self.environment.string, Data::String(string));
	}
}
