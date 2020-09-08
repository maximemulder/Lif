use crate::nodes::Node;
use crate::nodes::block::Block;
use crate::nodes::declaration::Declaration;
use crate::runtime::engine::{ Control, Engine };
use crate::runtime::gc::GcTraceable;
use crate::runtime::reference::GcReference;
use crate::runtime::scope::GcScope;
use crate::runtime::value::GcValue;

pub trait Callable<'a>: GcTraceable {
	fn execute(&self, engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> GcReference<'a>;
	fn duplicate(&self) -> Box<dyn Callable<'a> + 'a>;
}

impl<'a> dyn Callable<'a> {
	pub fn call(&mut self, engine: &mut Engine<'a>, mut arguments: Vec<GcValue<'a>>) -> GcReference<'a> {
		if let Some(this) = engine.get_this() {
			arguments.insert(0, this);
		}

		return self.duplicate().execute(engine, arguments);
	}
}

#[derive(Clone)]
pub struct Primitive<'a> {
	callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> GcReference<'a>,
}

impl<'a> Primitive<'a> {
	pub fn new(callback: &'a dyn Fn(&mut Engine<'a>, Vec<GcValue<'a>>) -> GcReference<'a>) -> Self {
		return Self {
			callback,
		};
	}
}

impl<'a> Callable<'a> for Primitive<'a> {
	fn execute(&self, engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> GcReference<'a> {
		return (self.callback)(engine, arguments);
	}

	fn duplicate(&self) -> Box<dyn Callable<'a> + 'a> {
		return Box::new(self.clone());
	}
}

impl GcTraceable for Primitive<'_> {
	fn trace(&mut self) {}
}

#[derive(Clone)]
pub struct Function<'a> {
	scope: GcScope<'a>,
	parameters: &'a Vec<Declaration>,
	r#type: Option<GcValue<'a>>,
	block: &'a Block,
}

impl<'a> Function<'a> {
	pub fn new(scope: GcScope<'a>, parameters: &'a Vec<Declaration>, r#type: Option<GcValue<'a>>, block: &'a Block) -> Self {
		return Self {
			scope,
			parameters,
			r#type,
			block,
		};
	}
}

impl<'a> Callable<'a> for Function<'a> {
	fn execute(&self, engine: &mut Engine<'a>, arguments: Vec<GcValue<'a>>) -> GcReference<'a> {
		engine.push_frame(self.scope);
		for (parameter, argument) in self.parameters.iter().zip(arguments) {
			let mut reference = parameter.execute(engine);
			reference.write(argument);
		}

		let reference = self.block.execute(engine);
		engine.pop_frame();

		return match &engine.control {
			Some(control) => match control {
				Control::Break | Control::Continue => panic!(),
				Control::Return => {
					engine.control = None;
					let value = reference.read();
					if let Some(r#type) = self.r#type {
						value.cast(r#type);
					}

					engine.new_constant(Some(value))
				},
			},
			None => engine.new_undefined(),
		};
	}

	fn duplicate(&self) -> Box<dyn Callable<'a> + 'a> {
		return Box::new(self.clone());
	}
}

impl GcTraceable for Function<'_> {
	fn trace(&mut self) {
		self.scope.trace();
	}
}
