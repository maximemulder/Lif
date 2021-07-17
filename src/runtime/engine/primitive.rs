use crate::runtime::data::{ Class, FunctionPrimitive, Generic, GenericImplementation };
use crate::runtime::engine::Engine;
use crate::runtime::gc::GcRef;
use crate::runtime::utilities::Callable;
use crate::runtime::utilities::parameters::Parameters;
use crate::runtime::utilities::variable::Variable;
use crate::runtime::value::Value;

fn create_variable<'a>(parameter: (&str, GcRef<Class<'a>>)) -> Variable<'a> {
    Variable::new(Box::from(parameter.0), Some(parameter.1))
}

fn create_parameters<'a>(parameters: &[(&str, GcRef<Class<'a>>)], rest: Option<(&str, GcRef<Class<'a>>)>) -> Parameters<'a> {
    Parameters::new(parameters.iter().copied().map(create_variable).collect(), rest.map(create_variable))
}

impl<'a> Engine<'a> {
    pub fn primitive_class(&mut self, name: &str, parent: Option<GcRef<Class<'a>>>) -> GcRef<Class<'a>> {
        self.new_class_value(Some(name), parent).get::<GcRef<Class>>(self)
    }

    pub fn primitive_generic(&mut self, name: &str, parameters: Box<[Box<str>]>, implementation: impl GenericImplementation<'a> + 'a) -> GcRef<Generic<'a>> {
        self.new_generic_value(Some(name), parameters, implementation).get::<GcRef<Generic>>(self)
    }

    pub fn populate_class(&mut self, name: &str, class: GcRef<Class<'a>>) {
        let reference = self.new_constant(Value::new(self.primitives.class, class));
        self.set_variable(name, reference);
    }

    pub fn populate_generic(&mut self, name: &str, generic: GcRef<Generic<'a>>) {
        let reference = self.new_constant(Value::new(self.primitives.generic, generic));
        self.set_variable(name, reference);
    }

    pub fn primitive_function<const N: usize>(
        &mut self, name: &str, parameters: [(&str, GcRef<Class<'a>>); N], rest: Option<(&str, GcRef<Class<'a>>)>, r#return: Option<GcRef<Class<'a>>>, callback: &'a Callable<'a>
    ) {
        let primitive = self.new_function(Some(name), create_parameters(&parameters, rest), r#return, FunctionPrimitive::new(callback));
        self.set_variable(name, primitive);
    }

    pub fn primitive_method<const N: usize>(
        &mut self, mut class: GcRef<Class<'a>>, name: &str, parameters: [(&str, GcRef<Class<'a>>); N], rest: Option<(&str, GcRef<Class<'a>>)>, r#return: Option<GcRef<Class<'a>>>, callback: &'a Callable<'a>
    ) {
        let mut elements = Vec::<(&str, GcRef<Class<'a>>)>::new();
        elements.push(("self", class));
        elements.extend_from_slice(&parameters);
        let primitive = self.run_frame(class.scope(), |engine| {
            engine.new_function_value(Some(&name), create_parameters(&elements, rest), r#return, FunctionPrimitive::new(callback))
        });

        class.set_method(name, primitive);
    }

    pub fn primitive_static<const N: usize>(
        &mut self, mut class: GcRef<Class<'a>>, name: &str, parameters: [(&str, GcRef<Class<'a>>); N], rest: Option<(&str, GcRef<Class<'a>>)>, r#return: Option<GcRef<Class<'a>>>, callback: &'a Callable<'a>
    ) {
        let primitive = self.run_frame(class.scope(), |engine| {
            engine.new_function_value(Some(&name), create_parameters(&parameters, rest), r#return, FunctionPrimitive::new(callback))
        });

        class.set_static(name, self.new_constant(primitive));
    }
}
