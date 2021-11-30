use std::rc::Rc;

pub struct Vertex {
    id: String,
    x: f64,
    y: f64
}

pub struct Edge {
    source: Rc<Vertex>,
    target: Rc<Vertex>,
}

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

    pub fn add_edges(&mut self, source: Rc<Vertex>, target: Rc<Vertex>) {
        self.edges.push(Edge{source, target});
    }
}