pub mod nodes;
pub mod traits;

use crate::memory::Ref;
use crate::parser::CNode;

pub trait ANode {
    fn build(node: Ref<CNode>) -> Self;
}

pub struct SNode<T: ?Sized> {
    pub concrete: Ref<CNode>,
    r#abstract: T,
}

impl<T> SNode<T> {
    pub fn new(concrete: Ref<CNode>, r#abstract: T) -> Self {
        Self {
            concrete,
            r#abstract,
        }
    }
}

impl<T: ANode> SNode<T> {
    pub fn build(node: Ref<CNode>) -> Self {
        SNode::new(node, T::build(node))
    }
}

impl<T: ?Sized> SNode<T> {
    pub fn get(&self) -> &T {
        &self.r#abstract
    }
}
