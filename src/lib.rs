pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub trait Heap {
    // todo: expeiment with using ItemType. Removing this for now.
    type ItemType;
    // finally got the right syntax. So apparently , there is no need to include generics in trait
    // declaration like Heap<T>. I will implement this associated function later in the impl of
    // this trait. Well, you can't.
    // fn new<T>(&mut self, array: &[T], sorting_fun: &dyn Fn(&T, &T) -> bool) -> dyn Heap;

    // investigate : will there be an issue if instead of transferring ownership, a reference to
    // the item is passed ?
    fn push_heap(&mut self, item: Self::ItemType);

    fn pop_heap(&mut self) -> Option<Self::ItemType>;

    fn get_max(&self) -> &Self::ItemType;

    fn is_empty(&self) -> bool;
}

// learning: modules should have snake case name , and the structs in CamelCase. Noice.
pub mod binary_heap;
pub mod fibonacci_heap;

pub enum HeapType {
    BinaryHeap,
    // FibonacciHeap,
}

fn less_than(item1: &String, item2: &String) -> bool {
    item1 < item2
}

#[cfg(test)]
mod tests {
    use binary_heap::BinaryHeap;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    fn test_heap_get_max(heap_type: HeapType) {
        let mut heap = match heap_type {
            HeapType::BinaryHeap => BinaryHeap::new(Vec::new(), &less_than),
        };
        let item1 = "item1".to_owned();
        let item2 = "item2".to_owned();

        heap.push_heap(item2.clone());
        heap.push_heap(item1.clone());

        assert_eq!(
            heap.get_max(),
            if less_than(&item1, &item2) {
                &item1
            } else {
                &item2
            }
        );
    }

    fn test_heap_pop(heap_type: HeapType) {
        let mut heap = match heap_type {
            HeapType::BinaryHeap => BinaryHeap::new(Vec::new(), &less_than),
        };
        let item1 = "item1".to_owned();
        let item2 = "item2".to_owned();
        let item3 = "item3".to_owned();

        heap.push_heap(item1.clone());
        heap.push_heap(item2.clone());
        heap.push_heap(item3.clone());

        assert_eq!(heap.pop_heap(), Some(item1));
        assert!(!heap.is_empty());
        assert_eq!(heap.get_max(), &item2);
    }

    fn test_empty_heap(heap_type: HeapType) {
        let mut heap = match heap_type {
            HeapType::BinaryHeap => BinaryHeap::new(Vec::new(), &less_than),
        };
        assert_eq!(heap.pop_heap(), None);
    }

    #[test]
    fn test_binary_heap_pop() {
        test_heap_pop(HeapType::BinaryHeap);
    }

    #[test]
    fn test_binary_heap_empty() {
        test_empty_heap(HeapType::BinaryHeap);
    }

    #[test]
    fn test_binary_heap_get_max() {
        test_heap_get_max(HeapType::BinaryHeap);
    }
}
