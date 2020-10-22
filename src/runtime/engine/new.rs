use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::data::Data;
use crate::runtime::engine::Engine;
use crate::runtime::reference::GcReference;
use crate::runtime::value::GcValue;


impl<'a, 'b> Engine<'a, 'b> {
	pub fn new_array_value(&mut self, elements: Vec<GcReference<'a, 'b>>) -> GcValue<'a, 'b> {
		self.new_value(self.environment.array, Data::new_array(elements))
	}

	pub fn new_boolean_value(&mut self, boolean: bool) -> GcValue<'a, 'b> {
        self.new_value(self.environment.boolean, Data::new_boolean(boolean))
	}

	pub fn new_class_value(&mut self, parent: GcValue<'a, 'b>) -> GcValue<'a, 'b> {
		let tag = self.taggers.classes.generate(None);
        self.new_value(self.environment.class, Data::new_class(tag, Some(parent)))
	}

	pub fn new_class_primitive_value(&mut self, name: &str) -> GcValue<'a, 'b> {
		let tag = self.taggers.classes.generate(Some(Box::from(name)));
        self.new_value(self.environment.class, Data::new_class(tag, Some(self.environment.any)))
	}

    pub fn new_function_value(&mut self, name: Option<&'a str>, parameters: &'b [Node<'a>], r#type: Option<GcValue<'a, 'b>>, block: &'b Node<'a>) -> GcValue<'a, 'b> {
		let tag = self.taggers.functions.generate(name.map(Box::from));
        self.new_value(self.environment.function, Data::new_function(tag, self.scope, parameters, r#type, block))
    }

	pub fn new_generic_value(&mut self, name: Option<&'a str>, generics: &'b [&'a str], node: &'b dyn Executable<'a>) -> GcValue<'a, 'b> {
		let tag = self.taggers.generics.generate(name.map(Box::from));
        self.new_value(self.environment.generic, Data::new_generic(tag, generics, node))
	}

	pub fn new_integer_value(&mut self, integer: usize) -> GcValue<'a, 'b> {
        self.new_value(self.environment.integer, Data::new_integer(integer))
	}

	pub fn new_method_value(&mut self, function: GcValue<'a, 'b>, this: GcValue<'a, 'b>) -> GcValue<'a, 'b> {
		self.new_value(self.environment.method, Data::new_method(function, this))
	}

	pub fn new_object_value(&mut self, parent: GcValue<'a, 'b>) -> GcValue<'a, 'b> {
		self.new_value(parent, Data::new_object())
	}

    pub fn new_primitive_value(&mut self, name: &str, parameters: Box<[GcValue<'a, 'b>]>, callback: &'b dyn Fn(&mut Engine<'a, 'b>, Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b>) -> GcValue<'a, 'b> {
		let tag = self.taggers.functions.generate(Some(Box::from(name)));
        self.new_value(self.environment.function, Data::new_primitive(tag, parameters, callback))
    }

	pub fn new_string_value(&mut self, string: String) -> GcValue<'a, 'b> {
        self.new_value(self.environment.string, Data::new_string(string))
	}
}

impl<'a, 'b> Engine<'a, 'b> {
    pub fn new_array(&mut self, elements: Vec<GcReference<'a, 'b>>) -> GcReference<'a, 'b> {
		let value = self.new_array_value(elements);
        self.new_constant(value)
    }

    pub fn new_boolean(&mut self, boolean: bool) -> GcReference<'a, 'b> {
		let value = self.new_boolean_value(boolean);
        self.new_constant(value)
	}

	pub fn new_class(&mut self, parent: GcValue<'a, 'b>) -> GcReference<'a, 'b> {
		let value = self.new_class_value(parent);
        self.new_constant(value)
    }

    pub fn new_function(&mut self, name: Option<&'a str>, parameters: &'b [Node<'a>], r#type: Option<GcValue<'a, 'b>>, block: &'b Node<'a>) -> GcReference<'a, 'b> {
       let value = self.new_function_value(name, parameters, r#type, block);
        self.new_constant(value)
    }

	pub fn new_generic(&mut self, name: Option<&'a str>, generics: &'b [&'a str], node: &'b dyn Executable<'a>) -> GcReference<'a, 'b> {
		let value = self.new_generic_value(name, generics, node);
        self.new_constant(value)
    }

	pub fn new_integer(&mut self, integer: usize) -> GcReference<'a, 'b> {
		let value = self.new_integer_value(integer);
        self.new_constant(value)
    }

	pub fn new_method(&mut self, function: GcValue<'a, 'b>, this: GcValue<'a, 'b>) -> GcReference<'a, 'b> {
		let value = self.new_method_value(function, this);
        self.new_constant(value)
	}

	pub fn new_object(&mut self, parent: GcValue<'a, 'b>) -> GcReference<'a, 'b> {
		let value = self.new_object_value(parent);
        self.new_constant(value)
    }

    pub fn new_primitive(&mut self, name: &str, parameters: Box<[GcValue<'a, 'b>]>, callback: &'b dyn Fn(&mut Engine<'a, 'b>, Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b>) -> GcReference<'a, 'b> {
		let value = self.new_primitive_value(name, parameters, callback);
        self.new_constant(value)
    }

    pub fn new_string(&mut self, string: String) -> GcReference<'a, 'b> {
		let value = self.new_string_value(string);
        self.new_constant(value)
    }
}
