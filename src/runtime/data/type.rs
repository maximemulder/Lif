use std::collections::HashMap;

pub struct Type {
	parent: usize,
	statics: HashMap<String, usize>,
	methods: HashMap<String, usize>,
}
