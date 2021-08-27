use crate::memory::Ref;
use crate::parser::CNode;
use crate::walker::ANode;

pub struct AGenerics {
    generics: Option<Box<[Ref<str>]>>,
}


impl AGenerics {
    pub fn new(generics: Option<Box<[Ref<str>]>>) -> Self {
        Self {
            generics,
        }
    }

    pub fn build(&self) -> Option<Box<[Box<str>]>> {
        self.generics.as_ref().map(|generics| generics.iter()
            .map(|generic| Box::from(generic.as_ref()))
            .collect()
        )
    }
}

impl ANode for AGenerics {
    fn build(node: Ref<CNode>) -> Self {
        Self::new(node.children().get(1).map(|child| child.children().iter()
            .step_by(2)
            .map(|child| child.text())
            .collect()
        ))
    }
}
