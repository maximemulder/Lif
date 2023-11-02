use crate::runtime::gc::{GcRef, GcTrace};
use crate::runtime::bis::{Value, Variable};
use crate::runtime::bis::data::{Ref, GcClass};

pub type GcList<'a> = GcRef<List<'a>>;

pub struct List<'a>(Vec<Variable<'a>>);

impl<'a> List<'a> {
    pub fn new(class: GcClass<'a>, values: &[Value<'a>]) -> Self {
        let variables = values.iter()
            .copied()
            .map(|value| Variable::value(class, value))
            .collect();

        Self(variables)
    }

    pub fn values(&self) -> Box<[Value<'a>]> {
        self.0.iter()
            .map(|variable| variable.content().unwrap())
            .collect()
    }

    pub fn get(&self, index: usize) -> Value<'a> {
        self.0[index].content().unwrap()
    }

    pub fn get_ref(&mut self, index: usize) -> Ref<'a> {
        self.0[index].get_ref()
    }

    pub fn insert(&mut self, class: GcClass<'a>, index: usize, value: Value<'a>) {
        self.0.insert(index, Variable::value(class, value));
    }

    pub fn append(&mut self, class: GcClass<'a>, value: Value<'a>) {
        self.0.push(Variable::value(class, value));
    }

    pub fn prepend(&mut self, class: GcClass<'a>, value: Value<'a>) {
        self.0.insert(0, Variable::value(class, value));
    }

    pub fn remove(&mut self, index: usize) {
        self.0.remove(index);
    }
}

impl GcTrace for List<'_> {
    fn trace(&mut self) {
        for element in self.0.iter_mut() {
            element.trace()
        }
    }
}
