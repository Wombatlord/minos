use crate::graph::{Edge, Gridlike, Node};

#[allow(dead_code)]
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
        // Ensures we satisfy the heap invariant (child > parent).
        // Determine the index of the parent of the element at index k.
        // If k is not the root (idx 0), and the element at index k (child) 
        // is less than the element at the index of its parent,
        // swap the child and parent.
        // Update the value of k to reflect the new index of the swapped element,
        // find its new parent and repeat the process until either
        // the element at k becomes the root, or is greater than its parent.
        let mut parent = (k.saturating_sub(1)) / 2;

        while k > 0 && self.heap[k] < self.heap[parent] {
            self.swap(k, parent);
            k = parent;

            parent = (k.saturating_sub(1)) / 2;
        }
        k
    }

    pub fn sink(&mut self, mut k: usize) -> usize {
        // Ensures we satisfy the heap invariant (child > parent).
        // Determine the indices of the left and right child of the element at index k.
        // Assume the element at k is less than its children.
        // Check if the left child exists, and if so, check if the child is less than the parent.
        // If so, update to reflect the new smallest element.
        // Check the right child exists and against the current assumed smallest element and update if necessary.
        // If the element at index k turns out not to be less than a child, swap it, preferentially with the right
        // child if it was greater than both children.
        // update k to reflect the new index of the sunk element.

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

impl BinaryHeap<Edge<Node>> {
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

    pub fn lowest_cost_edge(&mut self) -> Option<Edge<Node>> {
        self.poll()
    }
}

impl BinaryHeap<Edge<usize>> {
    pub fn add_edges(&mut self, idx: usize, visited: &mut Vec<bool>, grid: &Gridlike<Node>) {
        visited[idx] = true;
    
        // let edges = node.node_edges(width, height);
        let edges = grid.edges(idx);
    
        for edge in edges {
            if !visited[edge.destination] {
                self.insert(edge);
            }
        }
    }

    pub fn lowest_cost_edge(&mut self) -> Option<Edge<usize>> {
        self.poll()
    }
}