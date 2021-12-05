use std::rc::Rc;

mod graph;
mod astar;

fn main() {
    println!("Hello, world!");
    let mut gs = graph::Graph::new();
    gs.add_vertex("A", 0.0, 0.0);
    gs.add_vertex("B", 1.0, 0.0);
    gs.add_edges(&Rc::clone(gs.get_vertex("A").unwrap()), &Rc::clone(gs.get_vertex("B").unwrap()));
    gs.add_vertex("C", 0.0, 1.0);
    gs.add_vertex("D", 1.0, 1.0);
    gs.add_edges(&Rc::clone(gs.get_vertex("B").unwrap()), &Rc::clone(gs.get_vertex("C").unwrap()));
    gs.add_edges(&Rc::clone(gs.get_vertex("A").unwrap()), &Rc::clone(gs.get_vertex("D").unwrap()));
    gs.add_vertex("E", 1.0, 2.0);
    gs.add_edges(&Rc::clone(gs.get_vertex("D").unwrap()), &Rc::clone(gs.get_vertex("E").unwrap()));
    astar::shortest_path(&gs, "A", "E");
    println!("{:?}",gs);
}
