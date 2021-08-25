use crate::memory::Ref;

pub struct AGenerics {
    generics: Box<[Ref<str>]>,
}


impl AGenerics {
    pub fn new(generics: Box<[Ref<str>]>) -> Self {
        Self {
            generics,
        }
    }

    pub fn build(&self) -> Option<Box<[Box<str>]>> {
        (self.generics.len() > 0).then(|| self.generics.iter()
            .map(|generic| Box::from(generic.as_ref()))
            .collect()
        )
    }
}
