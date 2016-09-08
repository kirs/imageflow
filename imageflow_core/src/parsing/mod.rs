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

    fn create_node(ctx: *mut ::ffi::Context, g: *mut ::ffi::Graph, node: s::Node )-> i32{
        match node {
            s::Node::FlipV => ::ffi::flow_node_create_primitive_flip_vertical(ctx, g, -1),
            _ => -10
        }
    }

    pub fn do_and_respond<'a, 'b, 'c, 'd>(&'a self, ctx: &'d mut ContextPtr, json: &'b [u8])  -> JsonResponse<'c> {

        let parsed : s::Build001 = serde_json::from_slice(json).unwrap();
        let cfg = parsed.builder_config;
        let io_vec = parsed.io;
        let graph = match parsed.framewise{
            s::Framewise::Graph(g) => g,
            s::Framewise::Steps(s) => BuildRequestHandler::steps_to_graph(s)
        };

        let job = ctx.create_job();


        let p = ctx.ptr.unwrap();

        let mut g = ::ffi::flow_graph_create(p, 10, 10, 3000, 2.0f32);

        //create nodes, develop a map of desired vs. actual node ids.

        //Create edges, using map

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