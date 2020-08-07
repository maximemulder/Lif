mod array;
mod boolean;
mod function;
mod instance;
mod integer;
mod text;
mod r#type;

use array::Array;
use boolean::Boolean;
use function::Function;
use instance::Instance;
use integer::Integer;
use text::Text;
use r#type::Type;

pub enum Data {
	Array(Array),
	Boolean(Boolean),
	Instance(Instance),
	Integer(Integer),
	Function(Function),
	Text(Text),
	Type(Type),
	Null,
}
