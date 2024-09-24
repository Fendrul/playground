use dependency_graph::DependencyGraph;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut graph = DependencyGraph::new();

    let node1 = graph.get_or_add_node(1);
    let node2 = graph.get_or_add_node(2);

    DependencyGraph::add_edge(&node1, &node2)?;

    let _child = node1.borrow().get_childs();

    Ok(())
}
