use crate::nodes::block::Block;
use super::scope::Scope;
use super::{ Object, Reference, Value };
use super::object::callable::Callable;
use super::engine_data::EngineData;
use std::cell::{ Ref, RefCell, RefMut };

pub struct Engine<'a> {
	pub data: RefCell<EngineData<'a>>,
}

impl<'a> Engine<'a> {
	pub fn new() -> Self {
		let mut data = EngineData::new();

		data.scopes.push(Scope::new());
		data.populate();

		return Self {
			data: RefCell::new(data),
		};
	}

	/*fn get_value(&self, reference: Reference) -> &mut Value {
		return &mut self.data.borrow_mut().references[reference.0];
	}*/

	pub fn new_undefined(&self) -> Reference {
		return self.new_reference(Value::new_undefined());
	}

	pub fn new_reference(&self, value: Value) -> Reference {
		let mut data = self.data.borrow_mut();
		let reference = Reference(data.references.len());
		data.references.push(value);
		return reference;
	}

	pub fn new_value(&self, object: Object<'a>) -> Value {
		let mut data = self.data.borrow_mut();
		let value = Value(data.objects.len());
		data.objects.push(object);
		return value;
	}

	pub fn new_object(&self, object: Object<'a>) -> Reference {
		let value = self.new_value(object);
		return self.new_reference(value);
	}

	pub fn get_object(&self, value: Value) -> Ref<Object<'a>> {
		return Ref::map(self.data.borrow(), |data| data.get_object(value));
	}

	pub fn get_object_mut(&self, value: Value) -> RefMut<Object<'a>> {
		return RefMut::map(self.data.borrow_mut(), |data| data.get_object_mut(value));
	}

	/* pub fn get_scope(&self) -> &mut Scope {
		let mut data = self.data.borrow_mut();
		return &mut data.scopes[data.scope];
	} */

	pub fn push_scope(&self) {
		let mut data = self.data.borrow_mut();
		let index = data.scope;
		data.scopes.push(Scope::new_child(index));
	}

	pub fn pop_scope(&self) {
		let mut data = self.data.borrow_mut();
		if let Some(parent) = data.scopes[data.scope].parent {
			data.scope = parent;
		} else {
			panic!();
		}
	}

	pub fn push_frame(&self, frame: usize) -> usize {
		let mut data = self.data.borrow_mut();
		let scope = data.scope;
		data.scope = frame;
		return scope;
	}

	pub fn pop_frame(&self, frame: usize) {
		self.data.borrow_mut().scope = frame;
	}

	pub fn new_variable(&self, name: &str, reference: Reference) {
		let mut data = self.data.borrow_mut();
		let index = data.scope;
		data.scopes[index].add_variable(name, reference);
	}

	pub fn get_variable(&self, name: &str) -> Reference {
		let mut data = self.data.borrow_mut();
		let index = data.scope;
		let mut scope = &mut data.scopes[index];
		loop {
			if let Some(object) = scope.get_variable(name) {
				return object;
			}

			if let Some(parent) = scope.parent {
				scope = &mut data.scopes[parent];
			} else {
				panic!();
			}
		}
	}

	pub fn get_cast_array(&self, value: Value) -> Ref<Vec<Reference>> {
		return Ref::map(self.data.borrow(), |data| {
			let object = data.get_object(value);
			object.cast(data.primitives.function);
			object.data.as_array()
		});
	}

	pub fn get_cast_boolean(&self, value: Value) -> Ref<bool> {
		return Ref::map(self.data.borrow(), |data| {
			let object = data.get_object(value);
			object.cast(data.primitives.function);
			object.data.as_boolean()
		});
	}

	pub fn get_cast_callable(&self, value: Value) -> Ref<Box<dyn Callable<'a> + 'a>> {
		return Ref::map(self.data.borrow(), |data| {
			let object = data.get_object(value);
			object.cast(data.primitives.function);
			object.data.as_callable()
		});
	}

	pub fn get_cast_string(&self, value: Value) -> Ref<String> {
		return Ref::map(self.data.borrow(), |data| {
			let object = data.get_object(value);
			object.cast(data.primitives.string);
			object.data.as_string()
		});
	}

	pub fn read(&self, reference: Reference) -> Value {
		let value: Value = self.data.borrow().get_value(reference).clone();
		if value == Value::new_undefined() {
			panic!();
		}

		return value;
	}

	pub fn write(&self, reference: Reference, value: Value) {
		*self.data.borrow_mut().get_value_mut(reference) = value;
	}

	pub fn call_method(&self, reference: Reference, name: &str, mut arguments: Vec<Reference>) -> Reference {
		arguments.insert(0, reference);
		return self.get_object(self.read(self.get_object(self.read(reference)).get_method(self, name).unwrap())).data.as_callable().call(self, arguments);
	}

	pub fn call_method_self(&self, reference: Reference, name: &str, arguments: Vec<Reference>) -> Reference {
		return self.get_object(self.read(self.get_object(self.read(reference)).get_method(self, name).unwrap())).data.as_callable().call(self, arguments);
	}

	pub fn call(&self, value: Value, arguments: Vec<Reference>) {
		/* let callable: &Box<dyn Callable> = &self.get_cast_callable(value);
		callable.call(self, arguments); */
	}
}

impl<'a> Engine<'a> {
	pub fn new_array(&self, elements: Vec<Reference>) -> Reference {
		return self.data.borrow_mut().new_array(elements);
	}

	pub fn new_boolean(&self, boolean: bool) -> Reference {
		return self.data.borrow_mut().new_boolean(boolean);
	}

	pub fn new_class(&self) -> Reference {
		return self.data.borrow_mut().new_class();
	}

	pub fn new_instance(&self, parent: Value) -> Reference {
		return self.data.borrow_mut().new_instance(parent);
	}

	pub fn new_integer(&self, integer: usize) -> Reference {
		return self.data.borrow_mut().new_integer(integer);
	}

	pub fn new_function(&self, parameters: &'a Vec<Box<str>>, block: &'a Block) -> Reference {
		return self.data.borrow_mut().new_function(parameters, block);
	}

	pub fn new_primitive(&self, callback: &'static dyn Fn(&Engine<'a>, Vec<Reference>) -> Reference) -> Reference {
		return self.data.borrow_mut().new_primitive(callback);
	}

	pub fn new_string(&self, string: String) -> Reference {
		return self.data.borrow_mut().new_string(string);
	}
}
