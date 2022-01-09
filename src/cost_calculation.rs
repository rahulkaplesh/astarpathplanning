use crate::graph::Vertex;

use geo::prelude::*;
use geo::point;


pub enum CostCalculator {
    SimpleCalculation,
    HaversineCalculation
}

impl CostCalculator {
    pub fn calculate_cost(&self ,start: &Vertex, current: &Vertex, target: &Vertex) -> u32 {
        match self {
            CostCalculator::SimpleCalculation => {
                let distance_start_vertex: f64 = f64::sqrt((start.x - current.x).powi(2) + (start.y - current.y).powi(2));
                let distance_target_vertex: f64 = f64::sqrt((target.x - current.x).powi(2) + (target.y - current.y).powi(2));
                ((distance_start_vertex + distance_target_vertex) * 10.0).round() as u32
            },
            CostCalculator::HaversineCalculation => {
                let start_point = point!(x: start.x, y: start.y);
                let current_point = point!(x: current.x, y: current.y);
                let target_point = point!(x: target.x, y: target.y);
                ((start_point.haversine_distance(&current_point) + target_point.haversine_distance(&current_point)) * 10.0).round() as u32
            }
        }
    }
}