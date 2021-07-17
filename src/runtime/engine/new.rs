use crate::runtime::data::{ Data, FunctionImplementation, GenericImplementation };
use crate::runtime::engine::Engine;
use crate::runtime::reference::GcReference;
use crate::runtime::utilities::parameters::Parameters;
use crate::runtime::value::{ Value, GcValue };

impl<'a> Engine<'a> {
    pub fn new_array_value(&mut self, class: GcValue<'a>, elements: Vec<GcReference<'a>>) -> GcValue<'a> {
        self.new_value(class, Data::array(elements))
    }

    pub fn new_array_any_value(&mut self, elements: Vec<GcReference<'a>>) -> GcValue<'a> {
        self.new_array_value(self.primitives.array_any, elements)
    }

    pub fn new_boolean_value(&mut self, boolean: bool) -> GcValue<'a> {
        self.alloc(Value::new(self.primitives.boolean, boolean))
    }

    pub fn new_class_value(&mut self, name: Option<&str>, parent: Option<GcValue<'a>>) -> GcValue<'a> {
        let tag = self.taggers.classes.generate(name);
        self.run_source_scope("__class__", |engine, scope| {
            engine.new_value(engine.primitives.class, Data::class(tag, scope, parent))
        })
    }

    pub fn new_float_value(&mut self, float: f64) -> GcValue<'a> {
        self.alloc(Value::new(self.primitives.float, float))
    }

    pub fn new_function_value(&mut self,
        name: Option<&str>, parameters: Parameters<'a>, r#return: Option<GcValue<'a>>, implementation: impl FunctionImplementation<'a> + 'a
    ) -> GcValue<'a> {
        let tag = self.taggers.functions.generate(name);
        self.run_source_scope("__function__", |engine, scope| {
            engine.new_value(engine.primitives.function, Data::function(tag, scope, parameters, r#return, implementation))
        })
    }

    pub fn new_generic_value(&mut self, name: Option<&str>, parameters: Box<[Box<str>]>, implementation: impl GenericImplementation<'a> + 'a) -> GcValue<'a> {
        let tag = self.taggers.generics.generate(name);
        self.run_source_scope("__generic__", |engine, scope| {
            engine.new_value(engine.primitives.generic, Data::generic(tag, scope, parameters, implementation))
        })
    }

    pub fn new_integer_value(&mut self, integer: isize) -> GcValue<'a> {
        self.alloc(Value::new(self.primitives.integer, integer))
    }

    pub fn new_method_value(&mut self, function: GcValue<'a>, this: GcValue<'a>) -> GcValue<'a> {
        self.new_value(self.primitives.method, Data::method(function, this))
    }

    pub fn new_object_value(&mut self, parent: GcValue<'a>) -> GcValue<'a> {
        self.new_value(parent, Data::object())
    }

    pub fn new_nullable_value(&mut self, class: GcValue<'a>, option: Option<GcValue<'a>>) -> GcValue<'a> {
        self.new_value(class, Data::nullable(option))
    }

    pub fn new_string_value(&mut self, string: String) -> GcValue<'a> {
        self.new_value(self.primitives.string, Data::string(string))
    }
}

impl<'a> Engine<'a> {
    pub fn new_array(&mut self, class: GcValue<'a>, elements: Vec<GcReference<'a>>) -> GcReference<'a> {
        let value = self.new_array_value(class, elements);
        self.new_constant(value)
    }

    pub fn new_array_any(&mut self, elements: Vec<GcReference<'a>>) -> GcReference<'a> {
        self.new_array(self.primitives.array_any, elements)
    }

    pub fn new_boolean(&mut self, boolean: bool) -> GcReference<'a> {
        let value = self.new_boolean_value(boolean);
        self.new_constant(value)
    }

    pub fn new_class(&mut self, name: Option<&str>, parent: Option<GcValue<'a>>) -> GcReference<'a> {
        let value = self.new_class_value(name, parent);
        self.new_constant(value)
    }

    pub fn new_float(&mut self, float: f64) -> GcReference<'a> {
        let value = self.new_float_value(float);
        self.new_constant(value)
    }

    pub fn new_function(&mut self,
        name: Option<&str>, parameters: Parameters<'a>, r#type: Option<GcValue<'a>>, implementation: impl FunctionImplementation<'a> + 'a
    ) -> GcReference<'a> {
       let value = self.new_function_value(name, parameters, r#type, implementation);
        self.new_constant(value)
    }

    pub fn new_generic(&mut self, name: Option<&str>, parameters: Box<[Box<str>]>, implementation: impl GenericImplementation<'a> + 'a) -> GcReference<'a> {
        let value = self.new_generic_value(name, parameters, implementation);
        self.new_constant(value)
    }

    pub fn new_integer(&mut self, integer: isize) -> GcReference<'a> {
        let value = self.new_integer_value(integer);
        self.new_constant(value)
    }

    pub fn new_method(&mut self, function: GcValue<'a>, this: GcValue<'a>) -> GcReference<'a> {
        let value = self.new_method_value(function, this);
        self.new_constant(value)
    }

    pub fn new_object(&mut self, parent: GcValue<'a>) -> GcReference<'a> {
        let value = self.new_object_value(parent);
        self.new_constant(value)
    }

    pub fn new_nullable(&mut self, class: GcValue<'a>, option: Option<GcValue<'a>>) -> GcReference<'a> {
        let value = self.new_nullable_value(class, option);
        self.new_constant(value)
    }

    pub fn new_string(&mut self, string: String) -> GcReference<'a> {
        let value = self.new_string_value(string);
        self.new_constant(value)
    }
}
