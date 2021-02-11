use crate::memory::Ref;
use crate::nodes::{ Executable, Node };
use crate::runtime::data::Data;
use crate::runtime::engine::Engine;
use crate::runtime::reference::GcReference;
use crate::runtime::utilities::Callable;
use crate::runtime::value::GcValue;

impl<'a> Engine<'a> {
    pub fn new_array_value(&mut self, elements: Vec<GcReference<'a>>) -> GcValue<'a> {
        self.new_value(self.primitives.array, Data::new_array(elements))
    }

    pub fn new_boolean_value(&mut self, boolean: bool) -> GcValue<'a> {
        self.new_value(self.primitives.boolean, Data::new_boolean(boolean))
    }

    pub fn new_class_value(&mut self, name: Option<&str>, parent: GcValue<'a>) -> GcValue<'a> {
        let tag = self.taggers.classes.generate(name.map(Box::from));
        self.new_value(self.primitives.class, Data::new_class(tag, Some(parent)))
    }

    pub fn new_class_primitive_value(&mut self, parent: Option<GcValue<'a>>, name: &str) -> GcValue<'a> {
        let tag = self.taggers.classes.generate(Some(Box::from(name)));
        self.new_value(self.primitives.class, Data::new_class(tag, parent))
    }

    pub fn new_function_value(&mut self, name: Option<&str>, parameters: Ref<[Node]>, r#type: Option<GcValue<'a>>, block: Ref<Node>) -> GcValue<'a> {
        let tag = self.taggers.functions.generate(name.map(Box::from));
        self.new_value(self.primitives.function_code, Data::new_function(tag, self.scope, parameters, r#type, block))
    }

    pub fn new_function_primitive_value(&mut self, name: &str, parameters: Box<[GcValue<'a>]>, callback: &'a Callable<'a>) -> GcValue<'a> {
        let tag = self.taggers.functions.generate(Some(Box::from(name)));
        self.new_value(self.primitives.function_primitive, Data::new_function_primitive(tag, parameters, callback))
    }

    pub fn new_generic_value(&mut self, name: Option<&str>, parameters: Ref<[Ref<str>]>, node: Ref<dyn Executable>) -> GcValue<'a> {
        let tag = self.taggers.generics.generate(name.map(Box::from));
        self.new_value(self.primitives.generic_code, Data::new_generic(tag, self.scope, parameters, node))
    }

    pub fn new_generic_primitive_value(&mut self, name: &str, parameters: Vec<Box<str>>, callback: &'a Callable<'a>) -> GcValue<'a> {
        let tag = self.taggers.generics.generate(Some(Box::from(name)));
        self.new_value(self.primitives.generic_primitive, Data::new_generic_primitive(tag, self.scope, parameters, callback))
    }

    pub fn new_integer_value(&mut self, integer: isize) -> GcValue<'a> {
        self.new_value(self.primitives.integer, Data::new_integer(integer))
    }

    pub fn new_method_value(&mut self, function: GcValue<'a>, this: GcValue<'a>) -> GcValue<'a> {
        self.new_value(self.primitives.method, Data::new_method(function, this))
    }

    pub fn new_object_value(&mut self, parent: GcValue<'a>) -> GcValue<'a> {
        self.new_value(parent, Data::new_object())
    }

    pub fn new_nullable_value(&mut self, class: GcValue<'a>, option: Option<GcValue<'a>>) -> GcValue<'a> {
        self.new_value(class, Data::new_nullable(option))
    }

    pub fn new_string_value(&mut self, string: String) -> GcValue<'a> {
        self.new_value(self.primitives.string, Data::new_string(string))
    }
}

impl<'a> Engine<'a> {
    pub fn new_array(&mut self, elements: Vec<GcReference<'a>>) -> GcReference<'a> {
        let value = self.new_array_value(elements);
        self.new_constant(value)
    }

    pub fn new_boolean(&mut self, boolean: bool) -> GcReference<'a> {
        let value = self.new_boolean_value(boolean);
        self.new_constant(value)
    }

    pub fn new_class(&mut self, name: Option<&str>, parent: GcValue<'a>) -> GcReference<'a> {
        let value = self.new_class_value(name, parent);
        self.new_constant(value)
    }

    pub fn new_function(&mut self, name: Option<&str>, parameters: Ref<[Node]>, r#type: Option<GcValue<'a>>, block: Ref<Node>) -> GcReference<'a> {
       let value = self.new_function_value(name, parameters, r#type, block);
        self.new_constant(value)
    }

    pub fn new_function_primitive(&mut self, name: &str, parameters: Box<[GcValue<'a>]>, callback: &'a Callable<'a>) -> GcReference<'a> {
        let value = self.new_function_primitive_value(name, parameters, callback);
        self.new_constant(value)
    }

    pub fn new_generic(&mut self, name: Option<&str>, parameters: Ref<[Ref<str>]>, node: Ref<dyn Executable>) -> GcReference<'a> {
        let value = self.new_generic_value(name, parameters, node);
        self.new_constant(value)
    }

    pub fn new_generic_primitive(&mut self, name: &str, parameters: Vec<Box<str>>, callback: &'a Callable<'a>) -> GcReference<'a> {
        let value = self.new_generic_primitive_value(name, parameters, callback);
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
