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

impl Vertex {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
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

    pub fn get_vertex_list(&self) -> Option<Vec<Rc<Vertex>>> {
        Some(self.vertices.clone())
    }

    pub fn get_adjacent_vertices_list(&self, vertex: &Rc<Vertex>) -> Option<Vec<Rc<Vertex>>> {
        let mut vertices = Vec::new();
        for edge in self.edges.iter() {
            if edge.source.get_id() == vertex.get_id() {
                vertices.push(Rc::clone(&edge.target));
            }
        }
        if vertices.len() > 0 {
            Some(vertices)
        } else {
            None
        }
    }
}