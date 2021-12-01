use core::fmt;
use std::{rc::Rc, fmt::write};

#[derive(Debug)]
pub struct Vertex {
    id: String,
    x: f64,
    y: f64
}

#[derive(Debug)]
pub struct Edge {
    source: Rc<Vertex>,
    target: Rc<Vertex>,
}

#[derive(Debug)]
pub struct Graph {
    vertices: Vec<Rc<Vertex>>,
    edges: Vec<Edge>
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, name: &str, x: f64, y: f64) -> usize {
        let index = self.vertices.len();
        self.vertices.push(Rc::new(Vertex{id: name.to_string() ,x ,y }));
        index
    }

    pub fn add_edges(&mut self, source: &Rc<Vertex>, target: &Rc<Vertex>) {
            let source = Rc::clone(source);
            let target=Rc::clone(target);
            self.edges.push(Edge{source, target});
    }

    pub fn get_vertex(&self, name: &str) -> Option<&Rc<Vertex>> {
        self.vertices.iter().find(|&vertex| {
            let Vertex{id, x, y} = &*Rc::clone(vertex);
            id == name
        })
    }
}