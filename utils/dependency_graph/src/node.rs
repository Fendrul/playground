use crate::{RefNode, WeakRefNode};
use std::rc::Rc;

pub struct Node<T> {
    pub(crate) value: T,
    pub(crate) childs: Vec<RefNode<T>>,
    pub(crate) parents: Vec<WeakRefNode<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value,
            childs: Vec::new(),
            parents: Vec::new(),
        }
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn get_childs(&self) -> &Vec<RefNode<T>> {
        &self.childs
    }

    pub fn get_parents(&self) -> &Vec<WeakRefNode<T>> {
        &self.parents
    }
    
    // The methods here are not exposed as pub so the verification logic can be keeped in the `DependencyGraph` struct.
    pub(crate) fn add_child(&mut self, child: &RefNode<T>) {
        self.childs.push(Rc::clone(child));
    }

    pub(crate) fn add_parent(&mut self, parent: &RefNode<T>) {
        self.parents.push(Rc::downgrade(parent));
    }
}

// The equality is based on the rule that the `DependencyGraph` will return the same node if the value is the same.
impl<T: Eq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq> Eq for Node<T> {}
