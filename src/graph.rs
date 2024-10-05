use rand::random;

pub type Edges = Vec<Edge<usize>>;

#[derive(Debug, Eq, Hash, Clone, Copy)]
pub struct Edge<T>
where
    Edge<T>: PartialEq,
{
    pub origin: T,
    pub destination: T,
    pub cost: u8,
}

impl PartialOrd for Edge<usize> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge<usize> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialEq for Edge<usize> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd for Edge<Node> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge<Node> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialEq for Edge<Node> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Edge<usize> {
    pub fn default() -> Edge<usize> {
        Edge {
            origin: 0,
            destination: 0,
            cost: 0
        }
    }
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

    pub fn node_edges(&self, width: usize, height: usize) -> Vec<Edge<Node>> {
        let mut edges: Vec<Edge<Node>> = vec![];
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

#[derive(Debug, Clone)]
pub struct Gridlike<T> {
    array: Vec<T>,
    width: usize,
    height: usize,
    adjacency: Vec<Vec<usize>>,
}

impl Gridlike<Node> {
    pub fn new(width: usize, height: usize) -> Self {
        let mut nodes: Vec<Node> = vec![];
        for y in 0..width {
            for x in 0..height {
                nodes.push(Node { x, y })
            }
        }

        let mut gridlike = Gridlike {
            width,
            height,
            array: nodes,
            adjacency: vec![vec![]],
        };

        gridlike.adjacency = gridlike.populate_adjacency_list();
        gridlike
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn nodes(&self) -> &Vec<Node> {
        &self.array
    }

    fn get_node_at(&self, x: usize, y: usize) -> Node {
        // Basically, the element stored at index i is (i%width, i/width).
        let n = &self.array[y * self.width + x];
        n.clone()
    }

    fn get_node_index(&self, node: &Node) -> usize {
        node.y * self.width + node.x
    }

    fn get_node_adjacency_indices(&self, node: &Node) -> Vec<usize> {
        let north = if node.y > 0 {
            Some((node.y - 1) * self.width + node.x)
        } else {
            None
        };
        let east = if node.x < self.width - 1 {
            Some(node.y * self.width + (node.x + 1))
        } else {
            None
        };
        let south = if node.y < self.height - 1 {
            Some((node.y + 1) * self.width + node.x)
        } else {
            None
        };
        let west = if node.x > 0 {
            Some(node.y * self.width + (node.x - 1))
        } else {
            None
        };

        vec![north, east, south, west]
            .iter()
            .filter_map(|i| *i)
            .collect()
    }

    pub fn edges(&self, idx: usize) -> Edges {
        let mut edges = vec![];
        for destination in &self.adjacency[idx] {
            edges.push(Edge {
                origin: idx,
                destination: *destination,
                cost: random::<u8>(),
            })
        }

        edges
    }

    pub fn adjacency_list(&self) -> &Vec<Vec<usize>> {
        &self.adjacency
    }

    fn populate_adjacency_list(&self) -> Vec<Vec<usize>> {
        let mut adj_list = vec![];
        for node in &self.array {
            adj_list.push(self.get_node_adjacency_indices(node))
        }
        // println!("{adj_list:#?}");
        adj_list
    }

    fn get_adjacencies(&self, node: &Node) -> Vec<Node> {
        let north = self.get_node_at(node.x, node.y - 1);
        let east = self.get_node_at(node.x + 1, node.y);
        let south = self.get_node_at(node.x, node.y + 1);
        let west = self.get_node_at(node.x - 1, node.y);

        vec![north, west, east, south]
    }
}
