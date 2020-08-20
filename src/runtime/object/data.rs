use crate::runtime::{ Reference };
use super::class::Class;
use super::callable::Callable;
use super::instance::Instance;

pub enum Data<'a> {
	Array(Vec<Reference<'a>>),
	Boolean(bool),
	Callable(Box<dyn Callable<'a> + 'a>),
	Class(Class<'a>),
	Instance(Instance<'a>),
	Integer(usize),
	String(String),
	Null,
}
