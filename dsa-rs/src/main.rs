mod heap;

use heap::PriorityQueue;

fn main() {
    let mut min_queue: PriorityQueue<i32> = PriorityQueue::new(heap::Variant::Min);

    let arr = [1, 5, 6, 8, 9, 7, 3];

    min_queue.build_queue(&arr);

    min_queue.insert(90);

    let priority = min_queue.extract_root();

    println!("{:?}", priority);

    min_queue.print_heap();

    let mut max_queue: PriorityQueue<i32> = PriorityQueue::new(heap::Variant::Max);

    let arr = [16, 20, 5, 25, 12, 19, 3];

    max_queue.build_queue(&arr);

    max_queue.insert(90);

    let priority = max_queue.extract_root();

    println!("{:?}", priority);

    max_queue.print_heap();
}
