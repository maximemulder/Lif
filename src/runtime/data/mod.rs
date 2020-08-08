mod array;
mod boolean;
mod function;
mod instance;
mod integer;
mod text;
mod r#type;

pub use array::Array;
pub use boolean::Boolean;
pub use function::Function;
pub use instance::Instance;
pub use integer::Integer;
pub use text::Text;
pub use r#type::Type;

pub enum Data {
	Array(Array),
	Boolean(Boolean),
	Instance(Instance),
	Integer(Integer),
	Function(Function),
	String(Text),
	Type(Type),
	Null,
}
