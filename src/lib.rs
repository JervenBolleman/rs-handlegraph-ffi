use handlegraph::handle::Handle;
use handlegraph::handle::Edge;
use handlegraph::handle::Direction;
use handlegraph::handle::NodeId;
use handlegraph::handlegraph::HandleGraph;
use handlegraph::hashgraph::HashGraph;
use handlegraph::pathgraph::PathHandleGraph;
use handlegraph::mutablehandlegraph::MutableHandleGraph;
use handlegraph;

use gfa::{
    gfa::{Link, Segment, GFA},
    optfields::OptFields,
    parser::{GFAParser, GFAResult},
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
//     return edge.;
// }

// #[no_mangle]
// pub extern fn edge_right(edge: &Edge)-> NodeId {
//     return edge.right;
// }

#[no_mangle]
pub extern fn hash_graph_new() -> HashGraph {
    return HashGraph::new();
}

#[no_mangle]
pub extern fn parse_gfa_into_hash_graph(file: &str) -> HashGraph {
    let parser = GFAParser::new();
    let gfa: GFA<usize, ()> = parser.parse_file(file).unwrap();
    let hashgraph = HashGraph::from_gfa(&gfa);
    return hashgraph;
}

// #[no_mangle]
// pub extern fn parse_gfa_into_path_graph(file: &str) -> HashGraph {
//     let parser = GFAParser::new();
//     let gfa: GFA<usize, ()> = parser.parse_file(file).unwrap();
//     let hashgraph = conversion::from_gfa::<PathGraph, _>(&gfa);
//     return hashgraph;
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
pub extern fn handles_iterator_next(mut iterator: Box<dyn Iterator<Item = Handle>>) -> Option<Handle> {
    return iterator.next();
}

#[no_mangle] 
pub extern fn handles_iterator_isdone(handle: Option<Handle>) -> bool {
    return None != handle;
}

#[no_mangle] 
pub extern fn handles_from_option(option: Option<Handle>) -> Handle {
    match option {
        Some(p) => return p,
        None => panic!("has no value"),
    }
}

#[no_mangle]
pub extern fn handle_graph_edges_iterator<'a>(graph: &'a dyn HandleGraph) -> Box<dyn Iterator<Item = Edge> + 'a> {
    return graph.edges_iter();
}

#[no_mangle] 
pub extern fn edges_iterator_next(mut iterator: Box<dyn  Iterator<Item = Edge>>) -> Option<Edge> {
    return iterator.next();
}

#[no_mangle] 
pub extern fn edges_iterator_isdone(edge: Option<Edge>) -> bool {
    return None != edge;
}

#[no_mangle] 
pub extern fn edge_from_option(option: Option<Edge>) -> Edge {
    match option {
        Some(p) => return p,
        None => panic!("has no value"),
    }
}

#[no_mangle]
pub extern fn handle_edges_iter_right<'a>(graph: &'a dyn HandleGraph, handle: Handle) -> Box<dyn Iterator<Item = Handle> + 'a> {
    return graph.handle_edges_iter(handle, Direction::Right);
}

#[no_mangle]
pub extern fn handle_edges_iter_left<'a>(graph: &'a dyn HandleGraph, handle: Handle) -> Box<dyn Iterator<Item = Handle> + 'a> {
    return graph.handle_edges_iter(handle, Direction::Left);
}