mod draw;
mod graph;
mod priority_queue;

use draw::draw;
use graph::{Cardinal, Edge, Edges, Node};
use priority_queue::PriorityQueue;
use std::collections::HashMap;
struct Maze {
    width: usize,
    height: usize,
    walls: HashMap<Node, Vec<Cardinal>>,
}

impl Maze {
    fn wall_edges(&self, grid: Vec<Node>, edges: Edges) -> HashMap<Node, Vec<Cardinal>> {
        let mut edge_walls: HashMap<Node, Vec<Cardinal>> = HashMap::new();
        let walls = vec![
            Cardinal::North,
            Cardinal::South,
            Cardinal::East,
            Cardinal::West,
        ];
    
        for node in &grid {
            edge_walls.entry(node.clone()).or_insert(walls.clone());
        }
    
        self.remove_walls(edges, &mut edge_walls);
    
        return edge_walls;
    }
    
    fn remove_walls(&self, edges: Vec<Edge>, edge_walls: &mut HashMap<Node, Vec<Cardinal>>) {
        for edge in edges.clone() {
            match edge.origin.cardinal_adjacency(edge.destination) {
                Some(cardinal) => {
                    edge_walls
                        .entry(edge.origin)
                        .and_modify(|w| w.retain(|&c| c != cardinal));
    
                    edge_walls
                        .entry(edge.destination)
                        .and_modify(|w| w.retain(|&c| c != cardinal.opposite()));
                }
                None => {}
            }
        }
    }
}

fn main() {
    let mut m = Maze {
        width: 100,
        height: 100,
        walls: HashMap::new(),
    };

    let g = nodes(m.width, m.height);

    // cost, minimum spanning tree as Vec<Node>
    let (_, mst) = lazy_prims(g.clone(), m.width, m.height);

    // println!("{cost}");
    // println!("{mst:#?}");

    m.walls = m.wall_edges(g, mst);
    // println!("{:#?}", m.walls);
    let document = draw(&m);
    let _ = svg::save("image.svg", &document);
}

fn nodes(width: usize, height: usize) -> Vec<Node> {
    let mut nodes: Vec<Node> = vec![];
    for y in 0..width {
        for x in 0..height {
            nodes.push(Node { x, y })
        }
    }

    nodes
}

fn lazy_prims(grid: Vec<Node>, width: usize, height: usize) -> (usize, Edges) {
    let mst_len = grid.len() - 1;
    let mut edge_count = 0;
    let mut mst_cost = 0;
    let mut mst_edges: Edges = vec![];

    let mut priority_queue: PriorityQueue = PriorityQueue::new();
    let mut visited: Vec<Node> = vec![];
    priority_queue.add_edges(grid[0], &mut visited, width, height);

    while !priority_queue.queue.is_empty() && edge_count != mst_len {
        let edge = priority_queue.lowest_cost_edge();

        if visited.contains(&edge.destination) {
            continue;
        }

        mst_edges.push(edge);
        edge_count += 1;
        mst_cost += edge.cost as usize;

        priority_queue.add_edges(
            edge.destination,
            &mut visited,
            width,
            height,
        );
    }

    if edge_count != mst_len {
        panic!("No MST found");
    }

    return (mst_cost, mst_edges);
}
