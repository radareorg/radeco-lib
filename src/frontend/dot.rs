//! Module to parse and write dot files for graphs.

// TODO:
//   * Add colors to different elements of the graph.

pub use super::cfg::*;
pub use super::esil::*;

use std::io::prelude::*;
use std::fs::File;

macro_rules! add_strings {
    ( $( $x: expr ),* ) => {
        {
            let mut s = String::new();
            $(
                s = format!("{}{}", s, $x);
                )*
                s
        }
    };
}

pub trait Dot {
    fn to_dot(&self) -> String;
}

impl Dot for CFG {
    fn to_dot(&self) -> String {
        let mut result = String::new();
        result = add_strings!(result, "digraph cfg {\n");
        // Graph configurations
        // Node configurations
        for node in self.g.raw_nodes().iter() {
            result = add_strings!(result, node.weight.to_dot());
        }

        // Connect BasicBlocks by edges.
        for edge in self.g.raw_edges() {
            let src_node = self.g.node_weight(edge.source()).unwrap();
            let dst_node = self.g.node_weight(edge.target()).unwrap();
            result = add_strings!(result, src_node.label, " -> ", dst_node.label,
                                  "[color=blue];\n");
        }

        add_strings!(result, "}")
    }
}

impl Dot for BasicBlock {
    fn to_dot(&self) -> String {
        let mut result = String::new();
        result = add_strings!(result, "<<table border=\"0\" cellborder=\"0\" cellpadding=\"1\">");
        for (_, inst) in &self.instructions {
            result = add_strings!(result, inst.to_dot());
        }
        result = add_strings!(result, "</table>>");
        add_strings!(self.label, "[label=", result, " shape=box];\n")
    }
}

impl Dot for Instruction {
    fn to_dot(&self) -> String {
        format!("<tr><td>{}</td></tr>", self)
    }
}

pub fn make_dot(g: CFG) {
    let mut dot_file = File::create("cfg.dot").ok().expect("Error. Cannot
                                                           create file!\n");
    dot_file.write_all(g.to_dot().as_bytes()).ok().expect("Error. Cannot write
                                                          file!\n");
    println!("* File written!");
}
