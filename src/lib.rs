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
    FibonacciHeap,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    fn test_heap() {}
}
