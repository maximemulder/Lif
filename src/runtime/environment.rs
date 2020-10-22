use crate::runtime::ReturnReference;
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::gc::GcTraceable;
use crate::runtime::value::GcValue;

pub struct Environment<'a, 'b> {
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

impl<'a, 'b> Environment<'a, 'b> {
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

impl GcTraceable for Environment<'_, '_> {
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
        self.environment.class = self.new_class_primitive_value("Class");
        self.environment.any   = self.new_class_primitive_value("Any");

        self.environment.array    = self.new_class_primitive_value("Array");
        self.environment.boolean  = self.new_class_primitive_value("Boolean");
        self.environment.function = self.new_class_primitive_value("Function");
        self.environment.generic  = self.new_class_primitive_value("Generic");
        self.environment.integer  = self.new_class_primitive_value("Integer");
        self.environment.method   = self.new_class_primitive_value("Method");
        self.environment.object   = self.new_class_primitive_value("Object");
        self.environment.string   = self.new_class_primitive_value("String");

        self.environment.class.class = self.environment.class;
        self.environment.class.data_class_mut().parent = Some(self.environment.any);
        self.environment.any.data_class_mut().parent = None;

        let any      = self.environment.any;
        let array    = self.environment.array;
        let boolean  = self.environment.boolean;
        let class    = self.environment.class;
        let function = self.environment.function;
        let generic  = self.environment.generic;
        let integer  = self.environment.integer;
        let method   = self.environment.method;
        let object   = self.environment.object;
        let string   = self.environment.string;

        self.add_constant_primitive("assert", [any],     &primitive_assert);
        self.add_constant_primitive("error",  [any],     &primitive_error);
        self.add_constant_primitive("exit",   [integer], &primitive_exit);
        self.add_constant_primitive("new",    [class],   &primitive_new);
        self.add_constant_primitive("print",  [any],     &primitive_print);

        self.add_constant_value("Any",      any);
        self.add_constant_value("Array",    array);
        self.add_constant_value("Boolean",  boolean);
        self.add_constant_value("Class",    class);
        self.add_constant_value("Function", function);
        self.add_constant_value("Integer",  integer);
        self.add_constant_value("Object",   object);
        self.add_constant_value("String",   string);

        self.add_method_primitive(any, ".",  [any, string], &any_chain);
        self.add_method_primitive(any, "==", [any, any],    &any_comparison);
        self.add_method_primitive(any, "!=", [any, any],    &any_difference);
        self.add_method_primitive(any, ">",  [any, any],    &any_greater);
        self.add_method_primitive(any, "<=", [any, any],    &any_lesser_equal);
        self.add_method_primitive(any, ">=", [any, any],    &any_greater_equal);

        self.add_method_primitive(array, "to_string", [array],               &array_to_string);
        self.add_method_primitive(array, "copy",      [array],               &array_copy);
        self.add_method_primitive(array, "append",    [array, any],          &array_append);
        self.add_method_primitive(array, "prepend",   [array, any],          &array_prepend);
        self.add_method_primitive(array, "insert",    [array, integer, any], &array_insert);
        self.add_method_primitive(array, "remove",    [array, integer],      &array_remove);
        self.add_method_primitive(array, "[]",        [array, array],        &array_access);

        self.add_method_primitive(boolean, "to_string", [boolean],      &boolean_to_string);
        self.add_method_primitive(boolean, "==",        [boolean, any], &boolean_comparison);

        self.add_method_primitive(class, "to_string", [class],         &class_to_string);
        self.add_method_primitive(class, ".",         [class, string], &class_chain);

        self.add_method_primitive(function, "to_string", [function],        &function_to_string);
        self.add_method_primitive(function, "()",        [function, array], &function_call);

        self.add_method_primitive(generic, "to_string", [generic],        &generic_to_string);
        self.add_method_primitive(generic, "<>",        [generic, array], &generic_apply);

        self.add_method_primitive(integer, "to_string", [integer],          &integer_to_string);
        self.add_method_primitive(integer, "==",        [integer, any],     &integer_comparison);
        self.add_method_primitive(integer, "<",         [integer, integer], &integer_lesser);
        self.add_method_primitive(integer, "+",         [integer, integer], &integer_addition);
        self.add_method_primitive(integer, "-",         [integer, integer], &integer_subtraction);
        self.add_method_primitive(integer, "*",         [integer, integer], &integer_multiplication);
        self.add_method_primitive(integer, "/",         [integer, integer], &integer_division);
        self.add_method_primitive(integer, "%",         [integer, integer], &integer_remainder);

        self.add_method_primitive(method, "to_string", [method],        &method_to_string);
        self.add_method_primitive(method, "<>",        [method, array], &method_apply);
        self.add_method_primitive(method, "()",        [method, array], &method_call);

        self.add_method_primitive(object, "to_string", [object],         &object_to_string);
        self.add_method_primitive(object, ".",         [object, string], &object_chain);

        self.add_method_primitive(string, "to_string", [string],      &string_to_string);
        self.add_method_primitive(string, "==",        [string, any], &string_comparison);
        self.add_method_primitive(string, "+",         [string, any], &string_concatenation);
    }
}

fn primitive_assert<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    if !arguments[0].data_boolean() {
        panic!();
    }

    Ok(engine.undefined())
}

fn primitive_error<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    println!("{}",  arguments[0].call_to_string(engine)?);
    panic!();
}

fn primitive_exit<'a, 'b>(_: &mut Engine<'a, 'b>, _: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    panic!();
}

fn primitive_new<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_object(arguments[0]))
}

fn primitive_print<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    println!("{}", arguments[0].call_to_string(engine)?);
    Ok(engine.undefined())
}

fn any_chain<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let this = arguments[0];
    let name = arguments[1].data_string().clone();
    if let Some(method) = this.get_method(&name) {
        return Ok(engine.new_method(method, this));
    }

    Err(Error::new_undefined_method(&name, this))
}

fn any_comparison<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_boolean(arguments[0] == arguments[1]))
}

fn any_difference<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let reference = arguments[0].call_method_self(engine, "==", arguments)?;
    Ok(engine.new_boolean(!reference.read()?.data_boolean()))
}

fn any_greater<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let left  = arguments[0].call_method_self(engine, "<", arguments.clone())?;
    let right = arguments[0].call_method_self(engine, "==", arguments.clone())?;
    Ok(engine.new_boolean(!left.read()?.data_boolean() && !right.read()?.data_boolean()))
}

fn any_greater_equal<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let reference = arguments[0].call_method_self(engine, "<", arguments)?;
    Ok(engine.new_boolean(!reference.read()?.data_boolean()))
}

fn any_lesser_equal<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let left  = arguments[0].call_method_self(engine, "<", arguments.clone())?;
    let right = arguments[0].call_method_self(engine, "==", arguments.clone())?;
    Ok(engine.new_boolean(*left.read()?.data_boolean() || *right.read()?.data_boolean()))
}

fn array_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let mut string = String::from("[");
    let elements = arguments[0].data_array().clone();
    for element in elements.iter() {
        string.push_str(&element.read()?.call_to_string(engine)?);
        string.push_str(", ");
    }

    if !elements.is_empty() {
        string.truncate(string.len() - 2);
    }

    string.push(']');
    Ok(engine.new_string(string))
}

fn array_copy<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_array(arguments[0].data_array().clone()))
}

fn array_append<'a, 'b>(engine: &mut Engine<'a, 'b>, mut arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let reference = engine.new_reference(arguments[1]);
    arguments[0].data_array_mut().push(reference);
    Ok(engine.undefined())
}

fn array_prepend<'a, 'b>(engine: &mut Engine<'a, 'b>, mut arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let reference = engine.new_reference(arguments[1]);
    arguments[0].data_array_mut().insert(0, reference);
    Ok(engine.undefined())
}

fn array_insert<'a, 'b>(engine: &mut Engine<'a, 'b>, mut arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let reference = engine.new_reference(arguments[1]);
    let index = *arguments[1].data_integer();
    arguments[0].data_array_mut().insert(index, reference);
    Ok(engine.undefined())
}

fn array_remove<'a, 'b>(engine: &mut Engine<'a, 'b>, mut arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let index = *arguments[1].data_integer();
    arguments[0].data_array_mut().remove(index);
    Ok(engine.undefined())
}

fn array_access<'a, 'b>(_: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(arguments[0].data_array()[*arguments[1].data_integer()])
}

fn boolean_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_string(arguments[0].data_boolean().to_string()))
}

fn boolean_comparison<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_boolean(arguments[0].data_boolean() == arguments[1].data_boolean()))
}

fn class_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let mut string = String::new();
    string += "Class";
    if let Some(name) = &arguments[0].data_class().tag.get_name() {
        string += "(";
        string += name;
        string += ")";
    }

    Ok(engine.new_string(string))
}

fn class_chain<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let mut this = arguments[0];
    let name = arguments[1].data_string().clone();
    if let Some(method) = this.get_method(&name) {
        return Ok(engine.new_method(method, this));
    }

    let member = engine.undefined();
    let class = this.data_class_mut();
    Ok(if let Some(&member) = class.statics.get(&name) {
        member
    } else {
        class.statics.insert(name.clone(), member);
        member
    })
}

fn class_access<'a, 'b>(engine: &mut Engine<'a, 'b>, _: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_constant(engine.environment.array))
}

fn function_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, _: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_string("FUNCTION".to_string()))
}

fn function_call<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let mut array = Vec::new();
    for argument in arguments[1].data_array().iter() {
        array.push(argument.read()?);
    }

    arguments[0].data_callable().duplicate().execute(engine, array)
}

fn generic_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, _: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_string("GENERIC".to_string()))
}

fn generic_apply<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    engine.push_scope();
    let value = arguments[0];
    let generic = value.data_generic();
    for (parameter, argument) in generic.generics.iter().zip(arguments[1].data_array()) {
        let reference = engine.new_reference(argument.read()?);
        engine.add_variable(parameter, reference);
    }

    let reference = generic.node.execute(engine)?;
    engine.pop_scope();
    Ok(reference)
}

fn integer_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_string(arguments[0].data_integer().to_string()))
}

fn integer_comparison<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_boolean(*arguments[0].data_integer() == *arguments[1].data_integer()))
}

fn integer_lesser<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_boolean(*arguments[0].data_integer() < *arguments[1].data_integer()))
}

fn integer_addition<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(*arguments[0].data_integer() + *arguments[1].data_integer()))
}

fn integer_subtraction<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(*arguments[0].data_integer() - *arguments[1].data_integer()))
}

fn integer_multiplication<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(*arguments[0].data_integer() * *arguments[1].data_integer()))
}

fn integer_division<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(*arguments[0].data_integer() / *arguments[1].data_integer()))
}

fn integer_remainder<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_integer(*arguments[0].data_integer() % *arguments[1].data_integer()))
}

fn object_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let mut string = String::from("{");
    let attributes = &arguments[0].data_object().attributes.clone();
    for (name, attribute) in attributes {
        string.push_str(&name);
        string.push_str(": ");
        string.push_str(&attribute.read()?.call_to_string(engine)?);
        string.push_str(", ");
    }

    if !attributes.is_empty() {
        string.truncate(string.len() - 2);
    }

    string.push('}');
    Ok(engine.new_string(string))
}

fn method_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, _: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_string("METHOD".to_string()))
}

fn method_apply<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let method = arguments[0].data_method();
    let mut elements = Vec::new();
    for argument in arguments[1].data_array().iter() {
        elements.push(*argument);
    }

    let array = engine.new_array_value(elements);
    let function = method.function.call_method(engine, "<>", vec![array])?.read()?;
    Ok(engine.new_method(function, method.this))
}

fn method_call<'a, 'b>(engine: &mut Engine<'a, 'b>, mut arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let this = arguments[0].data_method().this;
    arguments[1].data_array_mut().insert(0, engine.new_constant(this));
    let method = arguments[0].data_method();
    method.function.call_method(engine, "()", vec![arguments[1]])
}

fn object_chain<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let mut this = arguments[0];
    let name = arguments[1].data_string().clone();
    if let Some(method) = this.get_method(&name) {
        return Ok(engine.new_method(method, this));
    }

    let member = engine.undefined();
    let object = this.data_object_mut();
    Ok(if let Some(&member) = object.attributes.get(&name) {
        member
    } else {
        object.attributes.insert(name.clone(), member);
        member
    })
}

fn string_to_string<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_constant(arguments[0]))
}

fn string_comparison<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    Ok(engine.new_boolean(arguments[0].data_string() == arguments[1].data_string()))
}

fn string_concatenation<'a, 'b>(engine: &mut Engine<'a, 'b>, arguments: Vec<GcValue<'a, 'b>>) -> ReturnReference<'a, 'b> {
    let right = arguments[1].call_to_string(engine)?;
    Ok(engine.new_string(format!("{}{}", arguments[0].data_string(), right)))
}
