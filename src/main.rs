mod bheap;
mod draw;
mod graph;
mod priority_queue;
use bheap::{BinaryHeap, PriorityQueue};
use draw::draw;
use graph::{Cardinal, Edge, Edges, Gridlike, Node};
// use priority_queue::PriorityQueue;
use std::collections::HashMap;
struct Maze {
    width: usize,
    height: usize,
    walls: HashMap<Node, Vec<Cardinal>>,
}

impl Maze {
    fn wall_edges(&self, grid: Gridlike<Node>, edges: Edges) -> HashMap<Node, Vec<Cardinal>> {
        let mut edge_walls: HashMap<Node, Vec<Cardinal>> = HashMap::new();
        let walls = vec![
            Cardinal::North,
            Cardinal::East,
            Cardinal::South,
            Cardinal::West,
        ];

        for node in grid.nodes() {
            edge_walls.entry(node.clone()).or_insert(walls.clone());
        }

        self.remove_walls(edges, &mut edge_walls, &grid);

        return edge_walls;
    }

    fn remove_walls(&self, edges: Vec<Edge<usize>>, edge_walls: &mut HashMap<Node, Vec<Cardinal>>, grid: &Gridlike<Node>) {
        for edge in edges.clone() {
            let origin = grid.nodes()[edge.origin];
            let destination = grid.nodes()[edge.destination];
            match origin.cardinal_adjacency(destination){
                Some(cardinal) => {
                    edge_walls
                        .entry(origin)
                        .and_modify(|w| w.retain(|&c| c != cardinal));

                    edge_walls
                        .entry(destination)
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

    let g = Gridlike::new(m.width, m.height);

    // cost, minimum spanning tree as Vec<Edge>
    let (_, mst) = lazy_prims(0, &g);

    // Assign walls to nodes based on the edges in the MST.
    m.walls = m.wall_edges(g, mst);

    // Generate and save image of the maze.
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

fn lazy_prims(idx: usize, grid: &Gridlike<Node>) -> (usize, Edges) {
    let mst_len = grid.len() - 1;
    let mut edge_count = 0;
    let mut mst_cost = 0;
    let mut mst_edges: Edges = vec![];

    let max_edges = max_edges(grid.width(), grid.height());

    let mut priority_queue = BinaryHeap::<Edge<usize>>::with_capacity(max_edges);
    let mut visited: Vec<bool> = vec![false; grid.len()];
    priority_queue.add_edges(idx, &mut visited, &grid);

    while !priority_queue.heap.is_empty() && edge_count != mst_len {
        let edge = priority_queue.lowest_cost_edge().unwrap();

        if visited[edge.destination] {
            continue;
        }

        mst_edges.push(edge);
        edge_count += 1;
        mst_cost += edge.cost as usize;

        priority_queue.add_edges(edge.destination, &mut visited, &grid);
    }

    println!("{edge_count} : {mst_len}");
    if edge_count != mst_len {
        panic!("No MST found");
    }

    return (mst_cost, mst_edges);
}

// fn lazy_prims(grid: Vec<Node>, width: usize, height: usize) -> (usize, Edges) {
//     let mst_len = grid.len() - 1;
//     let mut edge_count = 0;
//     let mut mst_cost = 0;
//     let mut mst_edges: Edges = vec![];

//     let max_edges = max_edges(width, height);

//     let mut priority_queue = BinaryHeap::with_capacity(max_edges);
//     // let mut priority_queue: PriorityQueue = PriorityQueue::new();
//     let mut visited: Vec<Node> = vec![];
//     priority_queue.add_edges(grid[0], &mut visited, width, height);

//     while !priority_queue.heap.is_empty() && edge_count != mst_len {
//         let edge = priority_queue.lowest_cost_edge().unwrap();

//         if visited.contains(&edge.destination) {
//             continue;
//         }

//         mst_edges.push(edge);
//         edge_count += 1;
//         mst_cost += edge.cost as usize;

//         priority_queue.add_edges(edge.destination, &mut visited, width, height);
//     }

//     if edge_count != mst_len {
//         panic!("No MST found");
//     }

//     return (mst_cost, mst_edges);
// }

fn max_edges(width: usize, height: usize) -> usize {
    let corner_edges = 16;
    let sides_edges_width = (width - 2) * 6 * 2;
    let sides_edges_height = (height - 2) * 6 * 2;
    let inner_edges = (width - 2) ^ 2 * 8;
    let max_edges = corner_edges + sides_edges_width + sides_edges_height + inner_edges;
    max_edges
}
