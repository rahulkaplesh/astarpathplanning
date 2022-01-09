use crate::graph;
use crate::graph::Vertex;
use crate::cost_calculation::CostCalculator;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    edge_id: Rc<String>,
    route: Vec<Rc<String>>
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

fn calculate_cost(start: &Vertex, current: &Vertex, target: &Vertex) -> u32 {
    let distance_start_vertex: f64 = f64::sqrt((start.x - current.x).powi(2) + (start.y - current.y).powi(2));
    let distance_target_vertex: f64 = f64::sqrt((target.x - current.x).powi(2) + (target.y - current.y).powi(2));
    ((distance_start_vertex + distance_target_vertex) * 10.0).round() as u32
}

pub fn shortest_path(operation_graph: &graph::Graph, start_vertex: &str, goal_vertex: &str, cost_calc: CostCalculator) -> (Vec<Rc<String>>, usize) {
    let mut dist: HashMap<_, _> = HashMap::new();

    if let Some(vertices) = operation_graph.get_vertex_list() {
        for vert in vertices.iter() {
            dist.insert(vert.clone().get_id(), usize::MAX);
        };
    }
    let mut heap = BinaryHeap::new();
    let mut overall_cost = 0;
    let mut final_route = Vec::new();

    dist.insert(start_vertex.to_string(),0 as usize);
    let route = Vec::new();
    heap.push(State { cost: 0, edge_id: Rc::new(start_vertex.to_string()), route });

    while let Some(State{ cost, edge_id, mut route}) = heap.pop() {
        if *edge_id == goal_vertex {
            route.push(Rc::clone(&edge_id));
            overall_cost = cost;
            final_route = route.clone();
            break;
        }
        
        if let Some(adjacent_vertices) = operation_graph.get_adjacent_vertices_list(&Rc::clone(operation_graph.get_vertex(&*&edge_id).unwrap())) {
            println!("Adjacent Vertices : {:?}",adjacent_vertices);
            route.push(Rc::clone(&edge_id));
            for vertex in &adjacent_vertices {
                let edge_cost = cost_calc.calculate_cost(&Rc::clone(operation_graph.get_vertex(start_vertex).unwrap()), vertex, &Rc::clone(operation_graph.get_vertex(goal_vertex).unwrap()));
                println!("Move to {} Cost : {}", vertex.get_id(), (cost + edge_cost as usize));
                let next_node = State{ cost: cost + (edge_cost as usize), edge_id: Rc::new(vertex.get_id()), route: route.clone()};
                if next_node.cost < *dist.get(&vertex.get_id()).unwrap() {
                    heap.push(next_node);
                    dist.insert(vertex.get_id().to_string(),edge_cost as usize);
                }
            }
        }
    }
    (final_route, overall_cost)
}