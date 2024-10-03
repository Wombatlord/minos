use std::cmp::min;

use crate::graph::{Edge, Edges, Node};
pub struct PriorityQueue {
    pub queue: Edges,
}

impl PriorityQueue {
    pub fn new() -> PriorityQueue {
        PriorityQueue { queue: vec![] }
    }

    pub fn add_edges(&mut self, node: Node, visited: &mut Vec<Node>, width:usize, height:usize) {
        visited.push(node);
    
        let edges = node.node_edges(width, height);
    
        for edge in edges {
            if visited.contains(&edge.destination) {
                continue;
            }
            self.queue.push(edge);
        }
    }

    pub fn lowest_cost_edge(&mut self) -> Edge {
        let mut lowest_cost_edge: Edge = Edge {
            origin: Node { x: 0, y: 0 },
            destination: Node { x: 0, y: 0 },
            cost: 0,
        };
        let mut lowest_edge_idx: usize = 0;
        
        for (idx, edge) in self.queue.iter().enumerate() {
            if idx == 0 {
                lowest_cost_edge = *edge;
                lowest_edge_idx = idx;
                continue;
            }

            let min_cost_edge = min(lowest_cost_edge.cost, edge.cost);
            if lowest_cost_edge.cost == min_cost_edge {
                continue;
            }

            lowest_cost_edge = *edge;
            lowest_edge_idx = idx;
        }

        self.queue.remove(lowest_edge_idx);
        return lowest_cost_edge;
    }
}
