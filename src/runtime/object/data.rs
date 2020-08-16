use crate::runtime::{ Reference };
use super::class::Class;
use super::callable::Callable;
use super::instance::Instance;

pub enum Data<'a> {
	Array(Vec<Reference>),
	Boolean(bool),
	Callable(Box<dyn Callable<'a> + 'a>),
	Class(Class),
	Instance(Instance),
	Integer(usize),
	String(String),
	Null,
}
