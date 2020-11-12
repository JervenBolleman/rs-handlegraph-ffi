use handlegraph::{
    handle::*, handlegraph::*, hashgraph::*, mutablehandlegraph::*, pathgraph::*,
};

use gfa::{
    gfa::GFA,
    optfields::OptFields,
    parser::{GFAParser, GFAResult},
};

pub struct CGraph {
    graph: HashGraph,
}

#[repr(C)]
pub struct EdgeHolder{
    left: u64,
    right: u64,
}

#[repr(C)]
pub struct HandlesIter<'a> {
    finished: bool,
    iter:  Box<dyn Iterator<Item = Handle> + 'a>,
}

#[repr(C)]
pub struct EdgesIter<'a> {
    finished: bool,
    iter:  Box<dyn Iterator<Item = Edge> + 'a>,
}

#[repr(C)]
pub struct PathsIter<'a> {
    finished: bool,
    iter:  Box<dyn Iterator<Item = &'a i64> + 'a>,
}

impl CGraph {
    pub fn parse_gfa(file: &str) -> Self {
        let parser = GFAParser::new();
        let gfa: GFA<usize, ()> = parser.parse_file(file).unwrap();
        let hashgraph = HashGraph::from_gfa(&gfa);
        CGraph { graph: hashgraph }
    }

    pub fn handles(&self) -> HandlesIter<'_> {
        let iter = self.graph.handles_iter();
        HandlesIter {
            finished: false,
            iter,
        }
    }

    pub fn edges(&self) -> EdgesIter<'_> {
        let iter = self.graph.edges_iter();
        EdgesIter {
            finished: false,
            iter,
        }
    }

    pub fn paths(&self) -> PathsIter<'_> {
        let iter = self.graph.paths.keys();
        PathsIter {
            finished: false,
            iter: Box::new(iter),
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn load_hashgraph(file_name: *const u8, file_name_len: usize) -> *mut CGraph {
    let file_name_slice = std::slice::from_raw_parts(file_name, file_name_len);
    let file_str = std::str::from_utf8(file_name_slice).unwrap();
    Box::into_raw(Box::new(CGraph::parse_gfa(file_str)))
}

#[no_mangle]
pub unsafe extern "C" fn free_hashgraph(graph: *mut CGraph) {
    let _ = Box::from_raw(graph);
}

#[no_mangle]
pub unsafe extern "C" fn graph_handles<'a>(graph: *const CGraph) -> *mut HandlesIter<'a> {
    let g = &(*graph);
    let iter = g.handles();
    Box::into_raw(Box::new(iter))
}

#[no_mangle]
pub unsafe extern "C" fn free_handles_iter<'a>(handles: *mut HandlesIter<'a>) {
    let _ = Box::from_raw(handles);
}

#[no_mangle]
pub unsafe extern "C" fn handles_next<'a>(handles: *mut HandlesIter<'a>) -> u64 {
    let handles = &mut (*handles);
    let next = handles.iter.next();
    match next {
        None => {
            handles.finished = true;
            0
        }
        Some(h) => u64::from(h.id()),
    }
}

#[no_mangle]
pub unsafe extern "C" fn handles_has_next<'a>(handles: *mut HandlesIter<'a>) -> bool {
    let handles = &mut (*handles);
    return handles.finished;
}

#[no_mangle]
pub unsafe extern "C" fn graph_edges<'a>(graph: *const CGraph) -> *mut EdgesIter<'a> {
    let g = &(*graph);
    let iter = g.edges();
    Box::into_raw(Box::new(iter))
}

#[no_mangle]
pub unsafe extern "C" fn free_edges_iter<'a>(handles: *mut EdgesIter<'a>) {
    let _ = Box::from_raw(handles);
}

#[no_mangle]
pub unsafe extern "C" fn edges_next<'a>(edges: *mut EdgesIter<'a>) -> EdgeHolder {
    let edges = &mut (*edges);
    let next = edges.iter.next();
    match next {
        None => {
            edges.finished = true;
            return EdgeHolder{left:0, right:0};
        }
        Some(e) => EdgeHolder{left: u64::from(e.0.id()) , right: u64::from(e.1.id())},
    }
}

#[no_mangle]
pub unsafe extern "C" fn edges_has_next<'a>(edges: *mut EdgesIter<'a>) -> bool {
    let edges = &mut (*edges);
    return edges.finished;
}

#[no_mangle]
pub unsafe extern "C" fn graph_paths<'a>(graph: *const CGraph) -> *mut PathsIter<'a> {
    let g = &(*graph);
    let iter = g.paths();
    Box::into_raw(Box::new(iter))
}

#[no_mangle]
pub unsafe extern "C" fn free_paths_iter<'a>(paths: *mut PathsIter<'a>) {
    let _ = Box::from_raw(paths);
}

#[no_mangle]
pub unsafe extern "C" fn paths_next<'a>(paths: *mut PathsIter<'a>) -> i64 {
    let paths = &mut (*paths);
    let next = paths.iter.next();
    match next {
        None => {
            paths.finished = true;
            return 0;
        }
        Some(e) => *e
    }
}

#[no_mangle]
pub unsafe extern "C" fn paths_has_next<'a>(paths: *mut PathsIter<'a>) -> bool {
    let paths = &mut (*paths);
    return paths.finished;
}