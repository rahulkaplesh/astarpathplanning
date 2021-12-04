use crate::graph;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    edge_id: Rc<String>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn shortest_path(operation_graph: &graph::Graph, start_vertex: &str, goal_vertex: &str) {
    let mut dist: HashMap<_, _> = HashMap::new();
    if let Some(vertices) = operation_graph.get_vertex_list() {
        for vert in vertices.iter() {
            dist.insert(vert.clone().get_id(), usize::MAX);
        };
    }
    let mut heap = BinaryHeap::new();

    dist.entry(start_vertex.to_string()).or_insert(0);
    heap.push(State { cost: 0, edge_id: Rc::new(start_vertex.to_string()) });

    while let Some(State{ cost, edge_id}) = heap.pop() {
        if *edge_id == goal_vertex {
            println!("Cost of the entire route : {}", cost);
        }
        // We have already found a shorter way ...
        if cost > dist[&*edge_id] { continue; }

        let adjacent_vertices = operation_graph.get_adjacent_vertices_list(&Rc::clone(operation_graph.get_vertex(&*&edge_id).unwrap()));

        println!("{:?}",adjacent_vertices.unwrap());

        //TODO ! Further implementation to be completed !
    }
}