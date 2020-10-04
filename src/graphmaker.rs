
use crate::nandgraph::*;
use crate::nameregister::*;

use std::iter::Iterator;

pub struct Maker {
    register : NameRegister,
    graph : Graph,
    inputs : Vec<String>,
    outputs : Vec<String>
}

impl Maker {
    pub fn new() -> Self {
        Maker { register: NameRegister::new()
            , graph: Graph::new() 
            , inputs : Vec::new()
            , outputs : Vec::new()
        }
    }

    pub fn add_wire(&mut self, x : &str) -> usize {
        let v = self.graph.create_vertex();
        self.register.register(x.to_string(), v);
        v
    }
    
    pub fn add_input(&mut self, x : &str) -> usize {
        self.inputs.push(x.to_string());
        self.add_wire(x)
    }

    pub fn add_output(&mut self, x : &str) -> usize {
        self.outputs.push(x.to_string());
        self.add_wire(x)
    }

    pub fn get_wire(&mut self, x: &str) -> usize {
        if !self.register.registered(x.to_string()) {
            return self.add_wire(x)
        }
        else {
            return self.register.resolve_rev(x.to_string())
        }
    }

    pub fn connect_str(&mut self, x : &str, y: &str, p: bool) {
        let xi = self.get_wire(x);
        let yi = self.get_wire(y);
        self.graph.add_edge(xi, yi, p)
    }

    pub fn connect(&mut self, x :usize, y: usize, p:bool) {
        self.graph.add_edge(x, y, p)
    }

    pub fn connect_and<I>(&mut self, out: usize, inputs: I)
    where
        I: Iterator<Item = usize>
    {
        for i in inputs {
            self.graph.add_edge(i, out, true)
        }
    }

    pub fn connect_or<I>(&mut self, out: usize, inputs: I)
    where
        I: Iterator<Item = usize>
    {
        let x = self.graph.create_vertex();
        for i in inputs {
            self.graph.add_edge(i, x, false);
        }
        self.graph.add_edge(x, out, false);
    }

}


pub fn make_graph() -> Graph {
    Graph::new()
}