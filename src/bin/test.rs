use rust_heap::{binary_heap::BinaryHeap, Heap};

fn sort_fn(x: &i32, y: &i32) -> bool {
    x > y
}

fn main() {
    let mut t: Vec<i32> = vec![1, 2, 3];
    let mut x = BinaryHeap::new(t, &sort_fn);

    x.push_heap(1);

    x.print_stuff();
}
