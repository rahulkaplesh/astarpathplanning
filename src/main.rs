mod graph;

fn main() {
    println!("Hello, world!");
    let mut gs = graph::Graph::new();
    gs.add_vertex("A", 50.0, 49.0);
    gs.add_vertex("B", 45.5, 24.5);
}
