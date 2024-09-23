#![allow(dead_code)]

use std::error::Error;
use std::fmt::{Debug, Display};
use std::sync::{Arc, RwLock, Weak};

use rayon::prelude::*;
use thiserror::Error;

type RefNode<T> = Arc<RwLock<Node<T>>>;
type WeakRefNode<T> = Weak<RwLock<Node<T>>>;

pub struct Node<T> {
    value: T,
    childs: Vec<RefNode<T>>,
    parents: Vec<WeakRefNode<T>>,
}

/// The equality is based on the rule that the `DepGraph` will return the same node if the value is the same.
impl<T: Eq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq> Eq for Node<T> {}

/// A dependency graph implementation.
///
/// `DepGraph<T>` represents a directed graph where nodes contain values of type `T`.
/// It allows for adding nodes and edges, as well as querying the graph structure.
pub struct DepGraph<T> {
    nodes: Vec<RefNode<T>>,
}

#[derive(Error, Debug)]
pub enum AddNodeError {
    #[error("Failed to add node as it is already referenced in its ancestry: {0}")]
    CyclicRelation(String),

    #[error("Can't add edge to itself: {0}")]
    SameNode(String),

    #[error("A thread has paniced because of a fallacious access to an RwLock")]
    RwLockPanic
}

impl<T> DepGraph<T> {
    /// Creates a new, empty `DepGraph<T>`.
    ///
    /// # Returns
    ///
    /// A new `DepGraph<T>` instance with no nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use dependency_graph::DepGraph;
    /// let graph: DepGraph<i32> = DepGraph::new();
    /// ```
    pub fn new() -> DepGraph<T> {
        DepGraph { nodes: Vec::new() }
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
    /// use dependency_graph::DepGraph;
    /// let mut graph = DepGraph::new(); ///
    /// let node = graph.get_or_add_node(42);
    /// ```
    pub fn get_or_add_node(&mut self, value: T) -> Result<RefNode<T>, AddNodeError>
    where
        T: Eq + Send + Sync,
    {
        if let Some(node) = self.fetch_existing(&value)? {
            return Ok(node);
        }

        let node = Node {
            value,
            childs: Vec::new(),
            parents: Vec::new(),
        };

        let ref_node = Arc::new(RwLock::new(node));

        self.nodes.push(Arc::clone(&ref_node));

        Ok(ref_node)
    }

    fn fetch_existing(&mut self, value: &T) -> Result<Option<RefNode<T>>, AddNodeError>
    where
        T: Eq + Send + Sync,
    {
        let node = self.nodes
            .par_iter()
            .find_first(|node_ref| {
                node_ref.read().expect("Couldn't unwrap the value in fetch_existing").value == *value
            })
            .map(Arc::clone);

        Ok(node)
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
    /// A `Result<(), Box<dyn Error>>` indicating success or containing an error if the operation failed.
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
    /// use dependency_graph::DepGraph;
    /// let mut graph = DepGraph::new();
    /// let parent = graph.get_or_add_node(1);
    /// let child = graph.get_or_add_node(2);
    /// DepGraph::add_edge(&parent, &child).expect("Failed to add edge");
    /// ```
    pub fn add_edge<'a>(parent_ref: &'a RefNode<T>, child_ref: &'a RefNode<T>) -> Result<(), AddNodeError>
    where
        T: Eq + Display + Send + Sync,
    {
        verify_if_exists_in_parents(parent_ref, child_ref)?;

        let mut child = child_ref.write().map_err(|_| AddNodeError::RwLockPanic)?;
        let mut parent = parent_ref.write().map_err(|_| AddNodeError::RwLockPanic)?;

        if parent.value == child.value {
            return Err(AddNodeError::SameNode(parent.value.to_string()));
        }

        parent.childs.push(Arc::clone(child_ref));
        child.parents.push(Arc::downgrade(parent_ref));

        Ok(())
    }
}

impl<T> Default for DepGraph<T> {
    fn default() -> Self {
        DepGraph::new()
    }
}

fn verify_if_exists_in_parents<'a, T>(
    parent_ref: &'a RefNode<T>,
    child_ref: &'a RefNode<T>,
) -> Result<(), AddNodeError>
where
    T: Eq + Display + Send + Sync,
{
    let parent_node = parent_ref
        .read().map_err(|_| AddNodeError::RwLockPanic)?;

    if Arc::ptr_eq(parent_ref, child_ref) {
        return Err(AddNodeError::CyclicRelation(
            parent_node.value.to_string(),
        ));
    }

    parent_node
        .parents
        .par_iter()
        .try_for_each(|parent_weak_ref| -> Result<(), AddNodeError> {
            if let Some(parent_ref) = parent_weak_ref.upgrade() {
                let clone = parent_ref.clone();
                verify_if_exists_in_parents(&clone, child_ref)?;
            }

            Ok(())
        })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dep_graph() -> Result<(), AddNodeError> {
        let mut graph = DepGraph::new();
        let node1 = graph.get_or_add_node(1)?;
        let node2 = graph.get_or_add_node(2)?;
        let node3 = graph.get_or_add_node(3)?;
        let node4 = graph.get_or_add_node(4)?;

        assert!(DepGraph::add_edge(&node1, &node2).is_ok());
        assert!(DepGraph::add_edge(&node1, &node3).is_ok());
        assert!(DepGraph::add_edge(&node2, &node4).is_ok());
        assert!(DepGraph::add_edge(&node3, &node4).is_ok());

        assert_eq!(node1.read().unwrap().childs.len(), 2);
        assert_eq!(node1.read().unwrap().parents.len(), 0);

        assert_eq!(node2.read().unwrap().childs.len(), 1);
        assert_eq!(node2.read().unwrap().parents.len(), 1);

        assert_eq!(node3.read().unwrap().childs.len(), 1);
        assert_eq!(node3.read().unwrap().parents.len(), 1);

        assert_eq!(node4.read().unwrap().childs.len(), 0);
        assert_eq!(node4.read().unwrap().parents.len(), 2);

        Ok(())
    }

    #[test]
    fn test_cyclic_graph_error() -> Result<(), AddNodeError> {
        let mut graph = DepGraph::new();
        let node1 = graph.get_or_add_node(1)?;
        let node2 = graph.get_or_add_node(2)?;
        let node3 = graph.get_or_add_node(3)?;

        let _ = DepGraph::add_edge(&node1, &node2);
        let _ = DepGraph::add_edge(&node2, &node3);

        assert!(DepGraph::add_edge(&node3, &node1).is_err());
        
        Ok(())
    }

    #[test]
    fn test_find_same_node() -> Result<(), AddNodeError> {
        let mut graph = DepGraph::new();
        let node1 = graph.get_or_add_node(1)?;
        let node1bis = graph.get_or_add_node(1)?;

        assert!(Arc::ptr_eq(&node1, &node1bis));
        
        Ok(())
    }
}
