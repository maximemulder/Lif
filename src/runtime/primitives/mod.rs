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

use crate::code::Code;
use crate::memory::Ref;
use crate::nodes::build;
use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcTrace;
use crate::runtime::value::GcValue;

use std::process;

pub struct Primitives<'a> {
    pub any:      GcValue<'a>,
    pub array:    GcValue<'a>,
    pub boolean:  GcValue<'a>,
    pub class:    GcValue<'a>,
    pub function: GcValue<'a>,
    pub generic:  GcValue<'a>,
    pub method:   GcValue<'a>,
    pub object:   GcValue<'a>,
    pub integer:  GcValue<'a>,
    pub string:   GcValue<'a>,
}

impl<'a> Primitives<'a> {
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

impl GcTrace for Primitives<'_> {
    fn trace(&mut self) {
        for class in [self.any, self.array, self.boolean, self.class, self.function, self.generic, self.integer, self.method, self.object, self.string].iter_mut() {
            class.trace();
        }
    }
}

impl<'a> Engine<'a> {
    pub fn add_constant_value(&mut self, name: &str, value: GcValue<'a>) {
        let reference = self.new_constant(value);
        self.add_variable(name, reference);
    }

    fn add_constant_primitive<const N: usize>(&mut self, name: &str, parameters: [GcValue<'a>; N], callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> ReturnReference<'a>) {
        let primitive = self.new_primitive(Ref::from_ref(name), Box::new(parameters), callback);
        self.add_variable(name, primitive);
    }

    fn add_method_primitive<const N: usize>(&mut self, mut value: GcValue<'a>, name: &str, parameters: [GcValue<'a>; N], callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> ReturnReference<'a>) {
        let primitive = self.new_primitive(Ref::from_ref(&name), Box::new(parameters), callback).get_value();
        value.data_class_mut().methods.insert(name.to_string(), primitive);
    }

    pub fn populate(&mut self) {
        self.primitives.class = self.new_class_primitive_value(Ref::from_ref("Class"));
        self.primitives.any   = self.new_class_primitive_value(Ref::from_ref("Any"));

        self.primitives.array    = self.new_class_primitive_value(Ref::from_ref("Array"));
        self.primitives.boolean  = self.new_class_primitive_value(Ref::from_ref("Boolean"));
        self.primitives.function = self.new_class_primitive_value(Ref::from_ref("Function"));
        self.primitives.generic  = self.new_class_primitive_value(Ref::from_ref("Generic"));
        self.primitives.integer  = self.new_class_primitive_value(Ref::from_ref("Integer"));
        self.primitives.method   = self.new_class_primitive_value(Ref::from_ref("Method"));
        self.primitives.object   = self.new_class_primitive_value(Ref::from_ref("Object"));
        self.primitives.string   = self.new_class_primitive_value(Ref::from_ref("String"));

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

        self.add_constant_primitive("assert",  [any],     &assert);
        self.add_constant_primitive("error",   [any],     &error);
        self.add_constant_primitive("eval",    [string],  &eval);
        self.add_constant_primitive("exec",    [string],  &exec);
        self.add_constant_primitive("exit",    [integer], &exit);
        self.add_constant_primitive("include", [string],  &include);
        self.add_constant_primitive("new",     [class],   &new);
        self.add_constant_primitive("print",   [any],     &print);

        self.add_constant_value("Any",      any);
        self.add_constant_value("Array",    array);
        self.add_constant_value("Boolean",  boolean);
        self.add_constant_value("Class",    class);
        self.add_constant_value("Function", function);
        self.add_constant_value("Integer",  integer);
        self.add_constant_value("Object",   object);
        self.add_constant_value("String",   string);

        self.add_method_primitive(any, "__cn__", [any, string], &any::cn);
        self.add_method_primitive(any, "__eq__", [any, any],    &any::eq);
        self.add_method_primitive(any, "__ne__", [any, any],    &any::ne);
        self.add_method_primitive(any, "__gt__", [any, any],    &any::gt);
        self.add_method_primitive(any, "__le__", [any, any],    &any::le);
        self.add_method_primitive(any, "__ge__", [any, any],    &any::ge);

        self.add_method_primitive(array, "to_string", [array],               &array::to_string);
        self.add_method_primitive(array, "copy",      [array],               &array::copy);
        self.add_method_primitive(array, "append",    [array, any],          &array::append);
        self.add_method_primitive(array, "prepend",   [array, any],          &array::prepend);
        self.add_method_primitive(array, "insert",    [array, integer, any], &array::insert);
        self.add_method_primitive(array, "remove",    [array, integer],      &array::remove);
        self.add_method_primitive(array, "__id__",    [array, array],        &array::id);

        self.add_method_primitive(boolean, "to_string", [boolean],      &boolean::to_string);
        self.add_method_primitive(boolean, "__eq__",    [boolean, any], &boolean::eq);
        self.add_method_primitive(boolean, "__not__",   [boolean],      &boolean::not);

        self.add_method_primitive(class, "to_string", [class],         &class::to_string);
        self.add_method_primitive(class, "__cn__",    [class, string], &class::cn);
        self.add_method_primitive(class, "__id__",    [class],         &class::id);

        self.add_method_primitive(function, "to_string", [function],        &function::to_string);
        self.add_method_primitive(function, "__cl__",    [function, array], &function::cl);

        self.add_method_primitive(generic, "to_string", [generic],        &generic::to_string);
        self.add_method_primitive(generic, "__gn__",    [generic, array], &generic::gn);

        self.add_method_primitive(integer, "to_string", [integer],          &integer::to_string);
        self.add_method_primitive(integer, "__eq__",    [integer, any],     &integer::eq);
        self.add_method_primitive(integer, "__lt__",    [integer, integer], &integer::lt);
        self.add_method_primitive(integer, "__pos__",   [integer],          &integer::pos);
        self.add_method_primitive(integer, "__neg__",   [integer],          &integer::neg);
        self.add_method_primitive(integer, "__add__",   [integer, integer], &integer::add);
        self.add_method_primitive(integer, "__sub__",   [integer, integer], &integer::sub);
        self.add_method_primitive(integer, "__mul__",   [integer, integer], &integer::mul);
        self.add_method_primitive(integer, "__div__",   [integer, integer], &integer::div);
        self.add_method_primitive(integer, "__rem__",   [integer, integer], &integer::rem);
        self.add_method_primitive(integer, "__bnot__",  [integer],          &integer::bnot);
        self.add_method_primitive(integer, "__band__",  [integer, integer], &integer::band);
        self.add_method_primitive(integer, "__bor__",   [integer, integer], &integer::bor);
        self.add_method_primitive(integer, "__bxor__",  [integer, integer], &integer::bxor);
        self.add_method_primitive(integer, "__bls__",   [integer, integer], &integer::bls);
        self.add_method_primitive(integer, "__brs__",   [integer, integer], &integer::brs);
        self.add_method_primitive(integer, "__bcls__",  [integer, integer], &integer::bcls);
        self.add_method_primitive(integer, "__bcrs__",  [integer, integer], &integer::bcrs);

        self.add_method_primitive(method, "to_string", [method],        &method::to_string);
        self.add_method_primitive(method, "__gn__",    [method, array], &method::gn);
        self.add_method_primitive(method, "__cl__",    [method, array], &method::cl);

        self.add_method_primitive(object, "to_string", [object],         &object::to_string);
        self.add_method_primitive(object, "__cn__",    [object, string], &object::cn);

        self.add_method_primitive(string, "to_string", [string],      &string::to_string);
        self.add_method_primitive(string, "__eq__",    [string, any], &string::eq);
        self.add_method_primitive(string, "__add__",   [string, any], &string::add);
    }
}

fn assert<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    if !arguments[0].data_boolean() {
        panic!();
    }

    Ok(engine.undefined())
}

fn error<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let message = arguments[0].call_to_string(engine)?;
    writeln!(engine.error, "{}", message).unwrap();
    Ok(engine.undefined())
}

fn eval<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let code = Code::from_string(&engine.parser, 1, &build::expression, &arguments[0].data_string());
    Ok(match engine.run(code) {
        Some(reference) => reference,
        None => engine.undefined(),
    })
}

fn exec<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let code = Code::from_string(&engine.parser, 0, &build::program, &arguments[0].data_string());
    engine.run(code);
    Ok(engine.undefined())
}

fn exit<'a>(_: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    process::exit(*arguments[0].data_integer() as i32);
}

fn include<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let code = Code::from_file(&engine.parser, 0, &build::program, &arguments[0].data_string()).unwrap();
    engine.run(code);
    Ok(engine.undefined())
}

fn new<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    Ok(engine.new_object(arguments[0]))
}

fn print<'a>(engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> ReturnReference<'a> {
    let message = arguments[0].call_to_string(engine)?;
    writeln!(engine.output, "{}", message).unwrap();
    Ok(engine.undefined())
}
