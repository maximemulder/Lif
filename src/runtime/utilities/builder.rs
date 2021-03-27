use crate::runtime::engine::Engine;
use crate::runtime::utilities::Callable;
use crate::runtime::value::GcValue;

pub fn function<'a, const N: usize>(engine: &mut Engine<'a>, name: &str, parameters: [GcValue<'a>; N], callback: &'a Callable<'a>) {
    let primitive = engine.new_function_primitive(name, Box::new(parameters), None, callback);
    engine.add_variable(name, primitive);
}

pub fn r#static<'a, const N: usize>(engine: &mut Engine<'a>, mut value: GcValue<'a>, name: &str, parameters: [GcValue<'a>; N], callback: &'a Callable<'a>) {
    let primitive = engine.new_function_primitive_value(&name, Box::new(parameters), None, callback);
    value.data_class_mut().statics.insert(Box::from(name), engine.new_constant(primitive));
}

pub fn method<'a, const N: usize>(engine: &mut Engine<'a>, mut value: GcValue<'a>, name: &str, parameters: [GcValue<'a>; N], callback: &'a Callable<'a>) {
    let primitive = engine.new_function_primitive_value(&name, Box::new(parameters), None, callback);
    value.data_class_mut().methods.insert(Box::from(name), primitive);
}

pub fn method_rest<'a, const N: usize>(engine: &mut Engine<'a>, mut value: GcValue<'a>, name: &str, parameters: [GcValue<'a>; N], callback: &'a Callable<'a>) {
    let primitive = engine.new_function_primitive_value(&name, Box::new(parameters), Some(engine.primitives.any), callback);
    value.data_class_mut().methods.insert(Box::from(name), primitive);
}
