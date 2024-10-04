use crate::graph::{Edge, Node};

pub trait PriorityQueue<T: PartialOrd> {
    fn with_capacity(sz: usize) -> Self;
    fn contains(&self, el: &T) -> bool;
    fn remove(&mut self, el: &T);
    fn insert(&mut self, el: T);
    fn poll(&mut self) -> Option<T>;
}

pub struct BinaryHeap<T: PartialOrd> {
    pub heap: Vec<T>,
}

impl<T: PartialOrd> PriorityQueue<T> for BinaryHeap<T> {
    fn with_capacity(sz: usize) -> Self {
        Self {
            heap: Vec::with_capacity(sz),
        }
    }

    fn contains(&self, el: &T) -> bool {
        self.heap.contains(el)
    }

    fn remove(&mut self, el: &T) {
        if let Some(idx) = self.heap.iter().position(|x| x == el) {
            self.remove_at(idx);
        }
    }

    fn insert(&mut self, el: T) {
        self.heap.push(el);
        let end = self.heap.len() - 1;
        self.swim(end);
    }

    fn poll(&mut self) -> Option<T> {
        self.remove_at(0)
    }
}

impl<T: PartialOrd> BinaryHeap<T> {
    pub fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
    }

    pub fn remove_at(&mut self, i: usize) -> Option<T> {
        let end = self.heap.len() - 1;
        self.heap.swap(i, end);
        let removed = self.heap.pop();

        let i_ = self.sink(i);

        if i_ == i {
            self.swim(i);
        }

        return removed;
    }

    pub fn swim(&mut self, mut k: usize) -> usize {
        let mut parent = (k.saturating_sub(1)) / 2;

        while k > 0 && self.heap[k] < self.heap[parent] {
            self.swap(k, parent);
            k = parent;

            parent = (k.saturating_sub(1)) / 2;
        }
        k
    }

    pub fn sink(&mut self, mut k: usize) -> usize {
        let heap_size = self.heap.len();
        loop {
            let left = k * 2 + 1;
            let right = k * 2 + 2;

            let mut smallest = k;

            if left < heap_size && self.heap[left] < self.heap[smallest] {
                smallest = left;
            }

            if right < heap_size && self.heap[right] < self.heap[smallest] {
                smallest = right;
            }

            if smallest != k {
                self.heap.swap(smallest, k);
                k = smallest
            } else {
                break;
            }
        }
        k
    }
}

impl BinaryHeap<Edge> {
    pub fn add_edges(&mut self, node: Node, visited: &mut Vec<Node>, width:usize, height:usize) {
        visited.push(node);
    
        let edges = node.node_edges(width, height);
    
        for edge in edges {
            if visited.contains(&edge.destination) {
                continue;
            }
            self.insert(edge);
        }
    }

    pub fn lowest_cost_edge(&mut self) -> Option<Edge> {
        self.poll()
    }
}