use crate::runtime::Reference;
use std::collections::HashMap;

pub struct Instance {
	attributes: HashMap<String, Reference>,
}
