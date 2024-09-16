use crate::Heap;
use std::fmt::Debug;

pub struct BinaryHeap<'a, T>
where
    T: Sized + Debug,
{
    // when this was just [T] it showed error that size not known at compilation time. How is
    // this error gone now ?
    heap_array: Vec<T>,
    sorting_fn: &'a dyn Fn(&T, &T) -> bool,
}

impl<'a, T> BinaryHeap<'a, T>
where
    T: Sized + Debug,
{
    fn get_parent_index(child_index: usize) -> usize {
        (child_index - 1) / 2
    }

    fn get_children_indices(parent_index: usize) -> (usize, usize) {
        (2 * parent_index + 1, 2 * parent_index + 2)
    }

    fn le(&self, index1: usize, index2: usize) -> bool {
        if index1 >= self.heap_array.len() || index2 >= self.heap_array.len() {
            return true;
        }
        (self.sorting_fn)(
            self.heap_array.get(index1).unwrap(),
            self.heap_array.get(index2).unwrap(),
        )
    }

    fn heapify(&mut self, index: usize) {
        if index == 0 {
            return;
        }
        let parent_index = Self::get_parent_index(index);
        if self.le(parent_index, index) {
            return;
        }

        self.heap_array.swap(parent_index, index);
        self.heapify(parent_index);
    }

    fn sift_down(&mut self, index: usize) {
        if index * 2 > self.heap_array.len() {
            return;
        }
        let child_tuple = Self::get_children_indices(index);

        let min_child =
            if child_tuple.1 >= self.heap_array.len() || self.le(child_tuple.0, child_tuple.1) {
                child_tuple.0
            } else {
                child_tuple.1
            };

        if self.le(index, min_child) {
            return;
        }

        self.heap_array.swap(index, min_child);
        self.sift_down(min_child);
    }

    pub fn new(array: Vec<T>, sorting_fun: &'a dyn Fn(&T, &T) -> bool) -> BinaryHeap<'a, T> {
        let mut new_heap = Self {
            sorting_fn: sorting_fun,
            heap_array: array,
        };

        for i in (0..new_heap.heap_array.len()).rev() {
            new_heap.sift_down(i)
        }

        new_heap
    }

    pub fn print_stuff(&self) {
        println!("{:?}", &self.heap_array)
    }
}

impl<T> Heap for BinaryHeap<'_, T>
where
    T: Sized + Debug,
{
    type ItemType = T;

    fn push_heap(&mut self, item: T) {
        self.heap_array.push(item);

        self.heapify(self.heap_array.len() - 1);
    }

    fn pop_heap(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let last_pos = self.heap_array.len() - 1;
        self.heap_array.swap(0, last_pos);
        let ret_val = self.heap_array.pop();

        self.sift_down(0);

        return ret_val;
    }

    fn get_max(&self) -> &T {
        self.heap_array.get(0).unwrap()
    }

    fn is_empty(&self) -> bool {
        self.heap_array.is_empty()
    }
}
