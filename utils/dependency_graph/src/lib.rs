#![allow(dead_code)]

use node::Node;
use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::{Rc, Weak};
use thiserror::Error;
use AddNodeError::{CyclicRelation, SameNode};

mod node;

type RefNode<T> = Rc<RefCell<Node<T>>>;
type WeakRefNode<T> = Weak<RefCell<Node<T>>>;

/// A dependency graph implementation.
///
/// `DependencyGraph<T>` represents a directed graph where nodes contain values of type `T`.
/// It allows for adding nodes and edges, as well as querying the graph structure.
pub struct DependencyGraph<T> {
    nodes: Vec<RefNode<T>>,
}

#[derive(Error, Debug)]
pub enum AddNodeError {
    #[error("Failed to add node as it is already referenced in its ancestry: {0}")]
    CyclicRelation(String),

    #[error("Can't add edge to itself: {0}")]
    SameNode(String),
}

impl<T> DependencyGraph<T> {
    /// Creates a new, empty `DependencyGraph<T>`.
    ///
    /// # Returns
    ///
    /// A new `DependencyGraph<T>` instance with no nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use dependency_graph::DependencyGraph;
    /// let graph: DependencyGraph<i32> = DependencyGraph::new();
    /// ```
    pub fn new() -> DependencyGraph<T> {
        DependencyGraph { nodes: Vec::new() }
    }

    /// Retrieves an existing node with the given value or adds a new node if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to search for or add to the graph.
    ///
    /// # Returns
    ///
    /// A reference-counted, mutable reference to the node (`RefNode<T>`).
    ///
    /// # Type Constraints
    ///
    /// The type `T` must implement the `Eq` trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use dependency_graph::DependencyGraph;
    /// let mut graph = DependencyGraph::new(); 
    /// 
    /// let node = graph.get_or_add_node(42);
    /// ```
    pub fn get_or_add_node(&mut self, value: T) -> RefNode<T>
    where
        T: Eq,
    {
        if let Some(node) = self.fetch_existing(&value) {
            return node;
        }

        let node = Node::new(value);

        let ref_node = Rc::new(RefCell::new(node));

        self.nodes.push(Rc::clone(&ref_node));

        ref_node
    }

    fn fetch_existing(&self, value: &T) -> Option<RefNode<T>>
    where
        T: Eq,
    {
        self.nodes
            .iter()
            .find(|node_ref| node_ref.borrow().value == *value)
            .map(Rc::clone)
    }

    /// Adds an edge between two nodes in the graph.
    ///
    /// # Arguments
    ///
    /// * `parent_ref` - A reference to the parent node.
    /// * `child_ref` - A reference to the child node.
    ///
    /// # Returns
    ///
    /// A `Result<(), AddNodeError>` indicating success or containing an error if the operation failed.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The child node already exists in the parent's ancestry (to prevent cycles).
    /// - An attempt is made to add an edge from a node to itself.
    ///
    /// # Type Constraints
    ///
    /// The type `T` must implement both the `Eq` and `Display` traits.
    ///
    /// # Examples
    ///
    /// ```
    /// use dependency_graph::DependencyGraph;
    /// let mut graph = DependencyGraph::new();
    /// 
    /// let parent = graph.get_or_add_node(1);
    /// let child = graph.get_or_add_node(2);
    /// 
    /// DependencyGraph::add_edge(&parent, &child).expect("Failed to add edge");
    /// ```
    pub fn add_edge(parent_ref: &RefNode<T>, child_ref: &RefNode<T>) -> Result<(), AddNodeError>
    where
        T: Eq + Display,
    {
        if Rc::ptr_eq(parent_ref, child_ref) {
            return Err(SameNode(
                parent_ref.borrow().value.to_string(),
            ));
        }

        verify_if_exists_in_parents(parent_ref, child_ref)?;

        parent_ref.borrow_mut().add_child(child_ref);
        child_ref.borrow_mut().add_parent(parent_ref);

        Ok(())
    }
}

impl<T> Default for DependencyGraph<T> {
    fn default() -> Self {
        DependencyGraph::new()
    }
}

fn verify_if_exists_in_parents<T: Eq + Display>(
    parent_ref: &RefNode<T>,
    child_ref: &RefNode<T>,
) -> Result<(), AddNodeError> {
    let parent_node = parent_ref.borrow();

    if Rc::ptr_eq(parent_ref, child_ref) {
        return Err(CyclicRelation(parent_node.value.to_string()));
    }

    parent_node.parents.iter().try_for_each(|parent_weak_ref| {
        if let Some(parent_ref) = parent_weak_ref.upgrade() {
            verify_if_exists_in_parents(&parent_ref, child_ref)
        } else {
            Ok(())
        }
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dep_graph() {
        let mut graph = DependencyGraph::new();
        let node1 = graph.get_or_add_node(1);
        let node2 = graph.get_or_add_node(2);
        let node3 = graph.get_or_add_node(3);
        let node4 = graph.get_or_add_node(4);

        assert!(DependencyGraph::add_edge(&node1, &node2).is_ok());
        assert!(DependencyGraph::add_edge(&node1, &node3).is_ok());
        assert!(DependencyGraph::add_edge(&node2, &node4).is_ok());
        assert!(DependencyGraph::add_edge(&node3, &node4).is_ok());

        let node1 = node1.borrow();
        let node2 = node2.borrow();
        let node3 = node3.borrow();
        let node4 = node4.borrow();

        assert_eq!(node1.childs.len(), 2);
        assert_eq!(node1.parents.len(), 0);

        assert_eq!(node2.childs.len(), 1);
        assert_eq!(node2.parents.len(), 1);

        assert_eq!(node3.childs.len(), 1);
        assert_eq!(node3.parents.len(), 1);

        assert_eq!(node4.childs.len(), 0);
        assert_eq!(node4.parents.len(), 2);
    }

    #[test]
    fn test_cyclic_graph_error() {
        let mut graph = DependencyGraph::new();
        let node1 = graph.get_or_add_node(1);
        let node2 = graph.get_or_add_node(2);
        let node3 = graph.get_or_add_node(3);

        let _ = DependencyGraph::add_edge(&node1, &node2);
        let _ = DependencyGraph::add_edge(&node2, &node3);

        assert!(DependencyGraph::add_edge(&node3, &node1).is_err());
    }

    #[test]
    fn test_find_same_node() {
        let mut graph = DependencyGraph::new();
        let node1 = graph.get_or_add_node(1);
        let node1bis = graph.get_or_add_node(1);

        assert!(Rc::ptr_eq(&node1, &node1bis));
    }
}
