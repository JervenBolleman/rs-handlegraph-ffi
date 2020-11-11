extern crate handlegraph;

use handlegraph::handle::Handle;
use handlegraph::handle::NodeId;
use handlegraph::handlegraph::HandleGraph;
use handlegraph::hashgraph::HashGraph;

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

#[no_mangle]
pub extern fn hashgraph_new() -> HashGraph {
    return HashGraph::new();
}

#[no_mangle]
pub extern fn hashgraph_from_gfa(gfa: &GFA<usize, ?>) -> HashGraph {
    return HashGraph::from_gfa(gfa);
}

#[no_mangle]
pub extern fn handlegraph_max_node_id(graph: &dyn HandleGraph) -> NodeId {
    return graph.max_node_id();
}

#[no_mangle]
pub extern fn handlegraph_min_node_id(graph: &dyn HandleGraph) -> NodeId {
    return graph.min_node_id();
}

#[no_mangle]
pub extern fn handlegraph_has_node(graph: &dyn HandleGraph, node_id: NodeId) -> bool {
    return graph.has_node(node_id);
}