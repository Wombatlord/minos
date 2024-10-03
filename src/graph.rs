pub type Edges = Vec<Edge>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Edge {
    pub origin: Node,
    pub destination: Node,
    pub cost: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cardinal {
    North,
    East,
    South,
    West,
}

impl Cardinal {
    pub fn opposite(&self) -> Cardinal {
        match &self {
            Cardinal::North => Cardinal::South,
            Cardinal::East => Cardinal::West,
            Cardinal::South => Cardinal::North,
            Cardinal::West => Cardinal::East,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Node {
    pub x: usize,
    pub y: usize,
}

impl Node {
    pub fn north(&self) -> Option<Node> {
        if self.y == 0 {
            None
        } else {
            Some(Node {
                x: self.x,
                y: self.y - 1,
            })
        }
    }


    pub fn cardinal_adjacency(&self, node: Node) -> Option<Cardinal> {
        if self.y != 0 && self.y == node.y + 1 && self.x == node.x {
            return Some(Cardinal::North);
        }

        if node.x != 0 && self.x == node.x - 1 && self.y == node.y {
            return Some(Cardinal::East);
        }

        if node.y != 0 && self.y == node.y - 1 && self.x == node.x {
            return Some(Cardinal::South);
        }

        if self.x != 0 && self.x == node.x + 1 && self.y == node.y {
            return Some(Cardinal::West);
        }

        None
    }

    pub fn n2(&self, grid: &Vec<Node>) -> Option<Node> {
        if self.y == 0 {
            return None;
        };

        let north = Node {
            x: self.x,
            y: self.y - 1,
        };

        match grid.binary_search(&north) {
            Ok(n) => return Some(grid[n]),
            Err(_) => return None,
        }
    }

    pub fn east(&self, grid_width: usize) -> Option<Node> {
        if self.x >= grid_width - 1 {
            None
        } else {
            Some(Node {
                x: self.x + 1,
                y: self.y,
            })
        }
    }

    pub fn south(&self, grid_height: usize) -> Option<Node> {
        if self.y >= grid_height - 1 {
            None
        } else {
            Some(Node {
                x: self.x,
                y: self.y + 1,
            })
        }
    }

    pub fn west(&self) -> Option<Node> {
        if self.x == 0 {
            None
        } else {
            Some(Node {
                x: self.x - 1,
                y: self.y,
            })
        }
    }

    pub fn all_adjacent(&self, grid_width: usize, grid_height: usize) -> Vec<Node> {
        vec![
            self.north(),
            self.east(grid_width),
            self.south(grid_height),
            self.west(),
        ]
        .iter()
        .filter_map(|p| *p)
        .collect()
    }

    pub fn node_edges(&self, width: usize, height: usize) -> Edges {
        let mut edges: Vec<Edge> = vec![];
        let adj = self.all_adjacent(width, height);
        for n in adj {
            edges.push(Edge {
                origin: self.clone(),
                destination: n,
                cost: rand::random::<u8>(),
            });
        }

        edges
    }
}
