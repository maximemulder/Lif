pub mod build;
pub mod nodes;

use crate::memory::Ref;
use crate::parser::SNode;

pub struct ANode<T: ?Sized> {
    pub concrete: Ref<SNode>,
    r#abstract: T,
}

impl<T> ANode<T> {
    pub fn new(concrete: Ref<SNode>, r#abstract: T) -> Self {
        Self {
            concrete,
            r#abstract,
        }
    }
}

impl<T: ?Sized> ANode<T> {
    pub fn get(&self) -> &T {
        &self.r#abstract
    }
}
