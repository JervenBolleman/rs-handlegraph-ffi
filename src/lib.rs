extern crate handlegraph;

use handlegraph::handle::Handle;
use handlegraph::handle::Edge;
use handlegraph::handle::NodeId;
use handlegraph::handlegraph::HandleGraph;
use handlegraph::hashgraph::HashGraph;
use handlegraph::pathgraph::PathHandleGraph;
use handlegraph::mutablehandlegraph::MutableHandleGraph;

use gfa::{
    gfa::{Link, Segment, GFA},
    optfields::OptFields,
};

#[no_mangle]
pub extern fn node_as_integer(node: &Handle)-> u64 {
    return node.as_integer();
}

#[no_mangle]
pub extern fn node_is_reverse(node: &Handle)-> bool {
    return node.is_reverse();
}

#[no_mangle]
pub extern fn node_id(node: &Handle)-> NodeId {
    return node.id();
}

// #[no_mangle]
// pub extern fn edge_left(edge: &Edge)-> NodeId {
//     return edge.left;
// }

// #[no_mangle]
// pub extern fn edge_right(edge: &Edge)-> NodeId {
//     return edge.right;
// }

#[no_mangle]
pub extern fn hash_graph_new() -> HashGraph {
    return HashGraph::new();
}

// #[no_mangle]
// pub extern fn hash_graph_from_gfa(gfa: &GFA<usize, Any>) -> HashGraph {
//     return HashGraph::from_gfa(gfa);
// }

#[no_mangle]
pub extern fn handle_graph_max_node_id(graph: &dyn HandleGraph) -> NodeId {
    return graph.max_node_id();
}

#[no_mangle]
pub extern fn handle_graph_min_node_id(graph: &dyn HandleGraph) -> NodeId {
    return graph.min_node_id();
}

#[no_mangle]
pub extern fn handle_graph_has_node(graph: &dyn HandleGraph, node_id: NodeId) -> bool {
    return graph.has_node(node_id);
}

#[no_mangle]
pub extern fn handle_graph_sequence_length_of_node(graph: &dyn HandleGraph, handle: Handle) -> usize {
    return graph.length(handle);
}

#[no_mangle]
pub extern fn handle_graph_node_count(graph: &dyn HandleGraph) -> usize {
    return graph.node_count();
}

#[no_mangle]
pub extern fn handle_graph_edge_count(graph: &dyn HandleGraph) -> usize {
    return graph.edge_count();
}

#[no_mangle]
pub extern fn handle_graph_has_edge(graph: &dyn HandleGraph, left: Handle, right: Handle) -> bool {
    return graph.has_edge(left, right);
}

#[no_mangle]
pub extern fn handle_graph_handles_iterator<'a>(graph: &'a dyn HandleGraph) -> Box<dyn Iterator<Item = Handle> + 'a> {
    return graph.handles_iter();
}

#[no_mangle]
pub extern fn handle_graph_edges_iterator<'a>(graph: &'a dyn HandleGraph) -> Box<dyn Iterator<Item = Edge> + 'a> {
    return graph.edges_iter();
}