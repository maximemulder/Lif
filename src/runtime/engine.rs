use crate::nodes::{ Executable, Node };
use crate::runtime::ReturnReference;
use crate::runtime::data::{ Data, Tagger };
use crate::runtime::environment::Environment;
use crate::runtime::error::Error;
use crate::runtime::gc::{ GC_THRESHOLD, Gc, GcTraceable };
use crate::runtime::reference::{ GcReference, Reference };
use crate::runtime::scope::{ GcScope, Scope };
use crate::runtime::value::{ GcValue, Value };

#[derive(PartialEq, Eq)]
pub enum Control {
    Return,
    Break,
    Continue,
}

pub struct Taggers {
	generics:  Tagger,
	classes:   Tagger,
	functions: Tagger,
}

impl Taggers {
	pub fn new() -> Self {
		Self {
			generics:  Tagger::new(),
			classes:   Tagger::new(),
			functions: Tagger::new(),
		}
	}
}

pub struct Engine<'a, 'b> where 'a: 'b {
	pub environment: Environment<'a, 'b>,
	pub taggers:     Taggers,
    scopes:          Gc<Scope<'a, 'b>>,
    references:      Gc<Reference<'a, 'b>>,
    values:          Gc<Value<'a, 'b>>,
    registries:      Vec<Vec<GcReference<'a, 'b>>>,
    frames:          Vec<GcScope<'a, 'b>>,
    scope:           GcScope<'a, 'b>,
    undefined:       GcReference<'a, 'b>,
    control:         Option<Control>,
    allocations:     usize,
}

impl<'a, 'b> Engine<'a, 'b> {
    pub fn new() -> Self {
        let mut engine = Self {
			environment: Environment::new(),
			taggers:     Taggers::new(),
            scopes:      Gc::new(),
            references:  Gc::new(),
            values:      Gc::new(),
            registries:  Vec::new(),
            frames:      Vec::new(),
            scope:       GcScope::null(),
            undefined:   GcReference::null(),
            control:     None,
            allocations: 0,
        };

        engine.scope = engine.alloc_scope(Scope::new());
        engine.undefined = engine.alloc_reference(Reference::new_constant(None));
        engine.registries.push(Vec::new());
        engine.populate();
        engine
    }
}

impl<'a, 'b> Engine<'a, 'b> {
    pub fn alloc_value(&mut self, value: Value<'a, 'b>) -> GcValue<'a, 'b> {
        let value = self.values.alloc(value);
        self.allocations += 1;
        value
    }

    pub fn alloc_reference(&mut self, reference: Reference<'a, 'b>) -> GcReference<'a, 'b> {
        let reference = self.references.alloc(reference);
        self.allocations += 1;
        reference
    }

    pub fn alloc_scope(&mut self, scope: Scope<'a, 'b>) -> GcScope<'a, 'b> {
        let scope = self.scopes.alloc(scope);
        self.allocations += 1;
        scope
    }
}

impl<'a, 'b> Engine<'a, 'b> {
    pub fn new_value(&mut self, class: GcValue<'a, 'b>, data: Data<'a, 'b>) -> GcValue<'a, 'b> {
        self.alloc_value(Value::new(class, data))
    }

    pub fn new_reference(&mut self, value: GcValue<'a, 'b>) -> GcReference<'a, 'b> {
        self.alloc_reference(Reference::new_variable(Some(value), self.environment.any))
    }

    pub fn new_variable(&mut self, value: Option<GcValue<'a, 'b>>, r#type: GcValue<'a, 'b>) -> GcReference<'a, 'b> {
        self.alloc_reference(Reference::new_variable(value, r#type))
    }

    pub fn new_constant(&mut self, value: GcValue<'a, 'b>) -> GcReference<'a, 'b> {
        self.alloc_reference(Reference::new_constant(Some(value)))
    }

    pub fn undefined(&mut self) -> GcReference<'a, 'b> {
        self.undefined
    }
}

impl<'a, 'b> Engine<'a, 'b> {
    pub fn push_scope(&mut self) {
        self.scope = self.alloc_scope(Scope::new_child(self.scope));
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
                return Err(Error::new_undeclared_variable(name));
            }
        }
    }

    pub fn collect(&mut self) {
        self.trace();
        self.scopes.collect();
        self.references.collect();
        self.values.collect();
        self.allocations = 0;
    }

    pub fn execute(&mut self, node: &'b Node<'a>) -> ReturnReference<'a, 'b> {
        self.registries.push(Vec::new());
        let reference = match node.sem.execute(self) {
            Ok(reference) => reference,
            Err(mut error) => {
                if error.node.is_none() {
                    error.node = Some(node.syn);
                }

                return Err(error);
            },
        };

        let index = self.registries.len() - 2;
        self.registries[index].push(reference);
        self.registries.pop();
        if self.allocations > GC_THRESHOLD {
            self.collect();
        }

        Ok(reference)
    }
}

impl<'a, 'b> Engine<'a, 'b> {
    pub fn control_new(&mut self, control: Control, node: &'b Option<Node<'a>>) -> ReturnReference<'a, 'b> {
        let reference = if let Some(node) = node {
            let value = self.execute(node)?.read()?;
            self.new_constant(value)
        } else {
            self.undefined()
        };

        if self.control.is_none() {
            self.control = Some(control);
        }

        Ok(reference)
    }

    pub fn control_none(&mut self) -> bool {
        self.control.is_none()
    }

    pub fn control_is(&mut self, other: Control) -> bool {
        if let Some(control) = &self.control {
            if *control == other {
                return true;
            }
        }

        false
    }

    pub fn control_consume(&mut self, control: Control) -> bool {
        if self.control_is(control) {
            self.control = None;
            return true;
        }

        false
    }
}

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
