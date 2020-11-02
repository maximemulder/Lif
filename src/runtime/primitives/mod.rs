mod any;
mod array;
mod boolean;
mod class;
mod function;
mod generic;
mod integer;
mod method;
mod object;
mod string;

use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTraceable;
use crate::runtime::value::GcValue;

pub struct Primitives<'a, 'b> {
    pub any:      GcValue<'a, 'b>,
    pub array:    GcValue<'a, 'b>,
    pub boolean:  GcValue<'a, 'b>,
    pub class:    GcValue<'a, 'b>,
    pub function: GcValue<'a, 'b>,
    pub generic:  GcValue<'a, 'b>,
    pub method:   GcValue<'a, 'b>,
    pub object:   GcValue<'a, 'b>,
    pub integer:  GcValue<'a, 'b>,
    pub string:   GcValue<'a, 'b>,
}

impl<'a, 'b> Primitives<'a, 'b> {
    pub fn new() -> Self {
        Self {
            any:      GcValue::null(),
            array:    GcValue::null(),
            boolean:  GcValue::null(),
            class:    GcValue::null(),
            function: GcValue::null(),
            generic:  GcValue::null(),
            method:   GcValue::null(),
            object:   GcValue::null(),
            integer:  GcValue::null(),
            string:   GcValue::null(),
        }
    }
}

impl GcTraceable for Primitives<'_, '_> {
    fn trace(&mut self) {
        for class in [self.any, self.array, self.boolean, self.class, self.function, self.generic, self.integer, self.method, self.object, self.string].iter_mut() {
            class.trace();
        }
    }
}

impl<'a, 'b> Engine<'a, 'b> {
    pub fn add_constant_value(&mut self, name: &str, value: GcValue<'a, 'b>) {
        let reference = self.new_constant(value);
        self.add_variable(name, reference);
    }

    fn add_constant_primitive<const N: usize>(&mut self, name: &str, parameters: [GcValue<'a, 'b>; N], callback: &'b dyn Fn(&mut Engine<'a, 'b>, Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b>) {
        let primitive = self.new_primitive(name, Box::new(parameters), callback);
        self.add_variable(name, primitive);
    }

    fn add_method_primitive<const N: usize>(&mut self, mut value: GcValue<'a, 'b>, name: &str, parameters: [GcValue<'a, 'b>; N], callback: &'b dyn Fn(&mut Engine<'a, 'b>, Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b>) {
        let primitive = self.new_primitive(name, Box::new(parameters), callback).get_value();
        value.data_class_mut().methods.insert(name.to_string(), primitive);
    }

    pub fn populate(&mut self) {
        self.primitives.class = self.new_class_primitive_value("Class");
        self.primitives.any   = self.new_class_primitive_value("Any");

        self.primitives.array    = self.new_class_primitive_value("Array");
        self.primitives.boolean  = self.new_class_primitive_value("Boolean");
        self.primitives.function = self.new_class_primitive_value("Function");
        self.primitives.generic  = self.new_class_primitive_value("Generic");
        self.primitives.integer  = self.new_class_primitive_value("Integer");
        self.primitives.method   = self.new_class_primitive_value("Method");
        self.primitives.object   = self.new_class_primitive_value("Object");
        self.primitives.string   = self.new_class_primitive_value("String");

        self.primitives.class.class = self.primitives.class;
        self.primitives.class.data_class_mut().parent = Some(self.primitives.any);
        self.primitives.any.data_class_mut().parent = None;

        let any      = self.primitives.any;
        let array    = self.primitives.array;
        let boolean  = self.primitives.boolean;
        let class    = self.primitives.class;
        let function = self.primitives.function;
        let generic  = self.primitives.generic;
        let integer  = self.primitives.integer;
        let method   = self.primitives.method;
        let object   = self.primitives.object;
        let string   = self.primitives.string;

        self.add_constant_primitive("assert", [any],     &assert);
        self.add_constant_primitive("error",  [any],     &error);
        self.add_constant_primitive("exit",   [integer], &exit);
        self.add_constant_primitive("new",    [class],   &new);
        self.add_constant_primitive("print",  [any],     &print);

        self.add_constant_value("Any",      any);
        self.add_constant_value("Array",    array);
        self.add_constant_value("Boolean",  boolean);
        self.add_constant_value("Class",    class);
        self.add_constant_value("Function", function);
        self.add_constant_value("Integer",  integer);
        self.add_constant_value("Object",   object);
        self.add_constant_value("String",   string);

        self.add_method_primitive(any, ".",  [any, string], &any::chain);
        self.add_method_primitive(any, "==", [any, any],    &any::comparison);
        self.add_method_primitive(any, "!=", [any, any],    &any::difference);
        self.add_method_primitive(any, ">",  [any, any],    &any::greater);
        self.add_method_primitive(any, "<=", [any, any],    &any::lesser_equal);
        self.add_method_primitive(any, ">=", [any, any],    &any::greater_equal);

        self.add_method_primitive(array, "to_string", [array],               &array::to_string);
        self.add_method_primitive(array, "copy",      [array],               &array::copy);
        self.add_method_primitive(array, "append",    [array, any],          &array::append);
        self.add_method_primitive(array, "prepend",   [array, any],          &array::prepend);
        self.add_method_primitive(array, "insert",    [array, integer, any], &array::insert);
        self.add_method_primitive(array, "remove",    [array, integer],      &array::remove);
        self.add_method_primitive(array, "[]",        [array, array],        &array::access);

        self.add_method_primitive(boolean, "to_string", [boolean],      &boolean::to_string);
        self.add_method_primitive(boolean, "==",        [boolean, any], &boolean::comparison);

        self.add_method_primitive(class, "to_string",  [class],         &class::to_string);
        self.add_method_primitive(class, ".",          [class, string], &class::chain);
        self.add_method_primitive(class, "[]",         [class],         &class::access);

        self.add_method_primitive(function, "to_string", [function],        &function::to_string);
        self.add_method_primitive(function, "()",        [function, array], &function::call);

        self.add_method_primitive(generic, "to_string", [generic],        &generic::to_string);
        self.add_method_primitive(generic, "<>",        [generic, array], &generic::apply);

        self.add_method_primitive(integer, "to_string", [integer],          &integer::to_string);
        self.add_method_primitive(integer, "==",        [integer, any],     &integer::comparison);
        self.add_method_primitive(integer, "<",         [integer, integer], &integer::lesser);
        self.add_method_primitive(integer, "+",         [integer, integer], &integer::addition);
        self.add_method_primitive(integer, "-",         [integer, integer], &integer::subtraction);
        self.add_method_primitive(integer, "*",         [integer, integer], &integer::multiplication);
        self.add_method_primitive(integer, "/",         [integer, integer], &integer::division);
        self.add_method_primitive(integer, "%",         [integer, integer], &integer::remainder);

        self.add_method_primitive(method, "to_string", [method],        &method::to_string);
        self.add_method_primitive(method, "<>",        [method, array], &method::apply);
        self.add_method_primitive(method, "()",        [method, array], &method::call);

        self.add_method_primitive(object, "to_string", [object],         &object::to_string);
        self.add_method_primitive(object, ".",         [object, string], &object::chain);

        self.add_method_primitive(string, "to_string", [string],      &string::to_string);
        self.add_method_primitive(string, "==",        [string, any], &string::comparison);
        self.add_method_primitive(string, "+",         [string, any], &string::concatenation);
    }
}

fn assert<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    if !arguments[0].data_boolean() {
        panic!();
    }

    Ok(engine.undefined())
}

fn error<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    println!("{}",  arguments[0].call_to_string(engine)?);
    panic!();
}

fn exit<'a, 'b>(_: &mut Engine<'a, 'b>, _: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    panic!();
}

fn new<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_object(arguments[0]))
}

fn print<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    println!("{}", arguments[0].call_to_string(engine)?);
    Ok(engine.undefined())
}
