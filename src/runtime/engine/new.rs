use crate::runtime::data::{ Array, Class, Function, FunctionImplementation, Generic, GenericImplementation, Method, Nullable, Object };
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcRef;
use crate::runtime::reference::GcReference;
use crate::runtime::utilities::parameters::Parameters;
use crate::runtime::value::Value;

impl<'a> Engine<'a> {
    pub fn new_boolean_value(&mut self, boolean: bool) -> Value<'a> {
        Value::primitive(self, boolean)
    }

    pub fn new_float_value(&mut self, float: f64) -> Value<'a> {
        Value::primitive(self, float)
    }

    pub fn new_integer_value(&mut self, integer: isize) -> Value<'a> {
        Value::primitive(self, integer)
    }

    pub fn new_array_value(&mut self, class: GcRef<Class<'a>>, elements: Vec<GcReference<'a>>) -> Value<'a> {
        Value::alloc(self, class, Array::new(elements))
    }

    pub fn new_array_any_value(&mut self, elements: Vec<GcReference<'a>>) -> Value<'a> {
        self.new_array_value(self.primitives.array_any, elements)
    }

    pub fn new_class_value(&mut self, name: Option<&str>, parent: Option<GcRef<Class<'a>>>, gc: bool) -> Value<'a> {
        let tag = self.taggers.classes.generate(name);
        self.run_source_scope("__class__", |engine, scope| {
            Value::alloc_primitive(engine, Class::new(tag, scope, parent, gc))
        })
    }

    pub fn new_function_value(&mut self,
        name: Option<&str>, parameters: Parameters<'a>, r#return: Option<GcRef<Class<'a>>>, implementation: impl FunctionImplementation<'a> + 'a
    ) -> Value<'a> {
        let tag = self.taggers.functions.generate(name);
        self.run_source_scope("__function__", |engine, scope| {
            Value::alloc_primitive(engine, Function::new(tag, scope, parameters, r#return, implementation))
        })
    }

    pub fn new_generic_value(&mut self, name: Option<&str>, parameters: Box<[Box<str>]>, implementation: impl GenericImplementation<'a> + 'a) -> Value<'a> {
        let tag = self.taggers.generics.generate(name);
        self.run_source_scope("__generic__", |engine, scope| {
            Value::alloc_primitive(engine, Generic::new(tag, scope, parameters, implementation))
        })
    }

    pub fn new_method_value(&mut self, function: Value<'a>, this: Value<'a>) -> Value<'a> {
        Value::alloc_primitive(self, Method::new(function, this))
    }

    pub fn new_object_value(&mut self, parent: GcRef<Class<'a>>) -> Value<'a> {
        Value::alloc(self, parent, Object::new())
    }

    pub fn new_nullable_value(&mut self, class: GcRef<Class<'a>>, option: Option<Value<'a>>) -> Value<'a> {
        Value::alloc(self, class, Nullable::new(option))
    }

    pub fn new_string_value(&mut self, string: String) -> Value<'a> {
        Value::alloc_primitive(self, string)
    }
}

impl<'a> Engine<'a> {
    pub fn new_boolean(&mut self, boolean: bool) -> GcReference<'a> {
        let value = self.new_boolean_value(boolean);
        self.new_constant(value)
    }

    pub fn new_float(&mut self, float: f64) -> GcReference<'a> {
        let value = self.new_float_value(float);
        self.new_constant(value)
    }

    pub fn new_integer(&mut self, integer: isize) -> GcReference<'a> {
        let value = self.new_integer_value(integer);
        self.new_constant(value)
    }

    pub fn new_array(&mut self, class: GcRef<Class<'a>>, elements: Vec<GcReference<'a>>) -> GcReference<'a> {
        let value = self.new_array_value(class, elements);
        self.new_constant(value)
    }

    pub fn new_array_any(&mut self, elements: Vec<GcReference<'a>>) -> GcReference<'a> {
        self.new_array(self.primitives.array_any, elements)
    }

    pub fn new_class(&mut self, name: Option<&str>, parent: Option<GcRef<Class<'a>>>, gc: bool) -> GcReference<'a> {
        let value = self.new_class_value(name, parent, gc);
        self.new_constant(value)
    }

    pub fn new_function(&mut self,
        name: Option<&str>, parameters: Parameters<'a>, r#return: Option<GcRef<Class<'a>>>, implementation: impl FunctionImplementation<'a> + 'a
    ) -> GcReference<'a> {
       let value = self.new_function_value(name, parameters, r#return, implementation);
        self.new_constant(value)
    }

    pub fn new_generic(&mut self, name: Option<&str>, parameters: Box<[Box<str>]>, implementation: impl GenericImplementation<'a> + 'a) -> GcReference<'a> {
        let value = self.new_generic_value(name, parameters, implementation);
        self.new_constant(value)
    }

    pub fn new_method(&mut self, function: Value<'a>, this: Value<'a>) -> GcReference<'a> {
        let value = self.new_method_value(function, this);
        self.new_constant(value)
    }

    pub fn new_object(&mut self, parent: GcRef<Class<'a>>) -> GcReference<'a> {
        let value = self.new_object_value(parent);
        self.new_constant(value)
    }

    pub fn new_nullable(&mut self, class: GcRef<Class<'a>>, option: Option<Value<'a>>) -> GcReference<'a> {
        let value = self.new_nullable_value(class, option);
        self.new_constant(value)
    }

    pub fn new_string(&mut self, string: String) -> GcReference<'a> {
        let value = self.new_string_value(string);
        self.new_constant(value)
    }
}
