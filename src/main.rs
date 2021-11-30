use std::rc::Rc;

mod graph;

fn main() {
    println!("Hello, world!");
    let mut gs = graph::Graph::new();
    gs.add_vertex("A", 50.0, 49.0);
    gs.add_vertex("B", 45.5, 24.5);
    gs.add_edges(&Rc::clone(gs.get_vertex("A").unwrap()), &Rc::clone(gs.get_vertex("B").unwrap()));
}
