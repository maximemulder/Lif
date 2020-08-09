mod array;
mod boolean;
mod function;
mod instance;
mod integer;
mod text;
mod class;

pub use array::Array;
pub use boolean::Boolean;
pub use function::Function;
pub use instance::Instance;
pub use integer::Integer;
pub use text::Text;
pub use class::Class;

pub enum Data {
	Array(Array),
	Boolean(Boolean),
	Class(Class),
	Instance(Instance),
	Integer(Integer),
	Function(Function),
	String(Text),
	Undefined(()),
}

impl Data {
	pub fn as_boolean(&self) -> &Boolean {
		if let Data::Boolean(boolean) = self {
			return boolean;
		}

		panic!();
	}
}
