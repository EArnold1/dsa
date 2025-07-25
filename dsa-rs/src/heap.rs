use std::fmt::Debug;

#[derive(PartialEq)]
pub enum Variant {
    Min,
    Max,
}

/// Priority queue with max & min heap variants
pub struct PriorityQueue<T> {
    heap: Vec<T>,
    variant: Variant,
}

impl<T: PartialOrd + Clone + Debug> PriorityQueue<T> {
    pub fn new(variant: Variant) -> Self {
        PriorityQueue {
            heap: Vec::new(),
            variant,
        }
    }

    fn parent(&self, index: usize) -> usize {
        // floor((i-1)/2)
        let mut parent_index = ((index - 1) / 2) as f64;
        parent_index = parent_index.floor();

        parent_index as usize
    }

    fn left_child(&self, index: usize) -> usize {
        (2 * index) + 1 // 2i + 1
    }

    fn right_child(&self, index: usize) -> usize {
        (2 * index) + 2 // 2i + 2
    }

    pub fn insert(&mut self, key: T) {
        self.heap.push(key);

        match self.variant {
            Variant::Max => self.max_heapify_up(self.heap.len() - 1),
            Variant::Min => self.min_heapify_up(self.heap.len() - 1),
        };
    }

    fn max_heapify_up(&mut self, mut index: usize) {
        while index != 0 && self.heap[self.parent(index)] < self.heap[index] {
            let parent = self.parent(index);

            // swap between parent & child
            self.heap.swap(parent, index);

            index = self.parent(index);
        }
    }

    fn min_heapify_up(&mut self, mut index: usize) {
        while index != 0 && self.heap[self.parent(index)] > self.heap[index] {
            let parent = self.parent(index);

            // swap between parent & child
            self.heap.swap(parent, index);

            index = self.parent(index);
        }
    }

    pub fn extract_root(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }

        let max = self.heap[0].clone();
        let last_child = self.heap[self.heap.len() - 1].clone();
        self.heap[0] = last_child;

        self.heap.pop();

        match self.variant {
            Variant::Max => self.max_heapify_down(0),
            Variant::Min => self.min_heapify_down(0),
        };

        Some(max)
    }

    fn max_heapify_down(&mut self, mut index: usize) {
        let mut largest = index;
        let heap_size = self.heap.len();

        loop {
            let left = self.left_child(index);
            let right = self.right_child(index);

            if left < heap_size && self.heap[left] > self.heap[largest] {
                largest = left;
            }

            if right < heap_size && self.heap[right] > self.heap[largest] {
                largest = right;
            }

            if largest == index {
                break;
            }

            self.heap.swap(largest, index);

            index = largest;
        }
    }

    fn min_heapify_down(&mut self, mut index: usize) {
        let mut smallest = index;
        let heap_size = self.heap.len();

        loop {
            let left = self.left_child(index);
            let right = self.right_child(index);

            if left < heap_size && self.heap[left] < self.heap[smallest] {
                smallest = left;
            }

            if right < heap_size && self.heap[right] < self.heap[smallest] {
                smallest = right;
            }

            if smallest == index {
                break;
            }

            self.heap.swap(smallest, index);

            index = smallest;
        }
    }

    pub fn build_queue(&mut self, array: &[T]) {
        self.heap = array.to_vec();

        let mut last_non_leaf = ((self.heap.len() / 2) - 1) as f64; // (n/2)-1 to get last non leaf node
        last_non_leaf = last_non_leaf.floor();

        for i in (0..=last_non_leaf as usize).rev() {
            match self.variant {
                Variant::Max => self.max_heapify_down(i),
                Variant::Min => self.min_heapify_down(i),
            };
        }
    }

    pub fn print_heap(&self) {
        println!("{:?}", &self.heap);
    }
}
