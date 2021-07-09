use crate::runtime::data::FunctionPrimitive;
use crate::runtime::engine::Engine;
use crate::runtime::utilities::Callable;
use crate::runtime::utilities::parameters::Parameters;
use crate::runtime::utilities::variable::Variable;
use crate::runtime::value::GcValue;

fn create_variable<'a>(parameter: (&str, GcValue<'a>)) -> Variable<'a> {
    Variable::new_unchecked(Box::from(parameter.0), Some(parameter.1))
}

fn create_parameters<'a>(parameters: &[(&str, GcValue<'a>)], rest: Option<(&str, GcValue<'a>)>) -> Parameters<'a> {
    Parameters::new(parameters.iter().copied().map(create_variable).collect(), rest.map(create_variable))
}

impl<'a> Engine<'a> {
    pub fn primitive_function<const N: usize>(
        &mut self, name: &str, parameters: [(&str, GcValue<'a>); N], rest: Option<(&str, GcValue<'a>)>, r#return: Option<GcValue<'a>>, callback: &'a Callable<'a>
    ) {
        let primitive = self.new_function(Some(name), create_parameters(&parameters, rest), r#return, FunctionPrimitive::new(callback));
        self.set_variable(name, primitive);
    }

    pub fn primitive_method<const N: usize>(
        &mut self, mut class: GcValue<'a>, name: &str, parameters: [(&str, GcValue<'a>); N], rest: Option<(&str, GcValue<'a>)>, r#return: Option<GcValue<'a>>, callback: &'a Callable<'a>
    ) {
        let mut elements = Vec::<(&str, GcValue<'a>)>::new();
        elements.push(("self", class));
        elements.extend_from_slice(&parameters);
        let primitive = self.run_frame(class.data_class().scope(), |engine| {
            engine.new_function_value(Some(&name), create_parameters(&elements, rest), r#return, FunctionPrimitive::new(callback))
        });

        class.data_class_mut().set_method(name, primitive);
    }

    pub fn primitive_static<const N: usize>(
        &mut self, mut class: GcValue<'a>, name: &str, parameters: [(&str, GcValue<'a>); N], rest: Option<(&str, GcValue<'a>)>, r#return: Option<GcValue<'a>>, callback: &'a Callable<'a>
    ) {
        let primitive = self.run_frame(class.data_class().scope(), |engine| {
            engine.new_function_value(Some(&name), create_parameters(&parameters, rest), r#return, FunctionPrimitive::new(callback))
        });

        class.data_class_mut().set_static(name, self.new_constant(primitive));
    }
}
