mod nodes;
use ContextPtr;
use JsonResponse;
use std::collections::HashMap;
extern crate imageflow_serde as s;
extern crate serde;
extern crate serde_json;

use ::Context;
pub struct BuildRequestHandler {

}

impl BuildRequestHandler{
    pub fn new () -> BuildRequestHandler{
        BuildRequestHandler{}
    }

    fn steps_to_graph(steps: Vec<s::Node>) -> s::Graph {
        //TODO: implement
       s:: Graph {
           nodes: HashMap::new(),
           edges: vec![]
        }
    }

   unsafe fn create_node(ctx: *mut ::ffi::Context, g: *mut *mut ::ffi::Graph, node: s::Node )-> i32{
        match node {
            s::Node::FlipV => ::ffi::flow_node_create_primitive_flip_vertical(ctx, g, -1),
            _ => -10
        }
    }

    unsafe fn create_edge(ctx: *mut ::ffi::Context, g: *mut *mut ::ffi::Graph, from_node: i32, to_node: i32, edge_kind: ::ffi::EdgeKind )-> i32{
        ::ffi::flow_edge_create(ctx, g, from_node,to_node,edge_kind)
    }

    pub fn do_and_respond<'a, 'b, 'c, 'd>(&'a self, ctx: &'d mut ContextPtr, json: &'b [u8])  -> JsonResponse<'c> {

        let parsed : s::Build001 = serde_json::from_slice(json).unwrap();
        let cfg = parsed.builder_config;
        let io_vec = parsed.io;
        let graph = match parsed.framewise{
            s::Framewise::Graph(g) => g,
            s::Framewise::Steps(s) => BuildRequestHandler::steps_to_graph(s)
        };

        unsafe {
            let p = ctx.ptr.unwrap();

            //create nodes, develop a map of desired vs. actual node ids.

            let mut g = ::ffi::flow_graph_create(p, 10, 10, 3000, 2.0f32);

            let mut node_id_map = HashMap::new();

            for (old_id, node) in graph.nodes {
                let new_id = BuildRequestHandler::create_node(p, &mut g, node);
                if new_id < 0 {
                    panic!("node creation failed");
                }
                node_id_map.insert(old_id, new_id);
            }

            for edge in graph.edges {
               let edge_id = BuildRequestHandler::create_edge(p, &mut g, node_id_map[edge.from], node_id_map[edge.to], edge.kind);
                if edge_id < 0 {
                    panic!("node creation failed");
                }
            }
            //Create edges, using map

            let job = ::ffi::flow_job_create(p);


            /*
                pub io_id: i32,
    pub direction: IoDirection,
    pub io: IoEnum,
    pub checksum: Option<IoChecksum>
            */
            for io_obj in parsed.io {
                match io_obj.io {
                    s::IoEnum::BytesHex(hex_string) => {

                        let io_ptr = ::ffi::flow_io_create_from_memory(p, ::ffi::IoMode::read_seekable,
                    }
                }
            }


        }

        //Create job
        //Add i/o

        //Build

        //TODO: Question, should JSON endpoints populate the Context error stacktrace when something goes wrong? Or populate both (except for OOM).

//        ::ffi::flow_node_create_canvas
//
//        ::ffi::flow_context_

    }



}

#[test]
fn test_handler(){

    let nodes = HashMap::new();
    nodes.insert(0, s::Node::FlipV);

    let build = s::Build001{
        builder_config: None,
        io: vec![],
        framewise: s::Framewise::Graph( s::Graph {
            nodes: nodes,
            edges: vec![]
        })
    };

    let json_str = serde_json::to_string_pretty(&build).unwrap();

    let handler = BuildRequestHandler::new();


    let response = handler.do_and_respond(Context::create().unsafe_borrow_mut_context_pointer(), json_str.into_bytes().as_slice());



}