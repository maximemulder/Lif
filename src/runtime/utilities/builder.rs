use crate::runtime::engine::Engine;
use crate::runtime::utilities::Callable;
use crate::runtime::value::GcValue;

pub fn function<'a, const N: usize>(engine: &mut Engine<'a>, name: &str, parameters: [GcValue<'a>; N], callback: &'a Callable<'a>) {
    let primitive = engine.new_function_primitive(name, Box::new(parameters), callback);
    engine.add_variable(name, primitive);
}

pub fn r#static<'a, const N: usize>(engine: &mut Engine<'a>, mut value: GcValue<'a>, name: &str, parameters: [GcValue<'a>; N], callback: &'a Callable<'a>) {
    let primitive = engine.new_function_primitive(&name, Box::new(parameters), callback).get_value();
    value.data_class_mut().statics.insert(name.to_string(), engine.new_constant(primitive));
}

pub fn method<'a, const N: usize>(engine: &mut Engine<'a>, mut value: GcValue<'a>, name: &str, parameters: [GcValue<'a>; N], callback: &'a Callable<'a>) {
    let primitive = engine.new_function_primitive(&name, Box::new(parameters), callback).get_value();
    value.data_class_mut().methods.insert(name.to_string(), primitive);
}
