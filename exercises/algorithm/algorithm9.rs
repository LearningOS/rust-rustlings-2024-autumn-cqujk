/*
	heap
	This question requires you to implement a binary heap function
*/
// I AM NOT DONE

use std::cmp::Ord;
use std::default::Default;
use std::mem::replace;
pub struct Heap<T>
where
    T: Default+Ord,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default+Ord,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.items.push(value);
        self.count+=1;
        self.sift_up(self.count-1);
    }
    pub fn next(&mut self) -> Option<T> {  
        if self.is_empty() {  
            return None;  
        }  
        let last = self.items.pop().unwrap_or_default();  
        let first = std::mem::replace(&mut self.items[0], last);  
        let root = first;  // 如果需要的话，`root` 可以是 `first` 或 `last`，取决于你的逻辑需求  
        self.count -= 1;  
        self.sift_down(0);  
        Some(root)  
    }  
    fn parent_idx(&self, idx: usize) -> usize {
        (idx-1) / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2+1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);  
        let right = self.right_child_idx(idx);  
  
        if left >= self.count {  
            return idx; // No left child  
        }  
  
        let left_val = &self.items[left];  
        let mut smallest = left;  
  
        if right < self.count {  
            let right_val = &self.items[right];  
            if (self.comparator)(right_val, left_val) {  
                smallest = right;  
            }  
        }  
  
        smallest 
    }
    fn sift_up(&mut self, idx: usize) {  
        let parent = self.parent_idx(idx);  
  
        if idx > 0 && (self.comparator)(&self.items[idx], &self.items[parent]) {  
            self.items.swap(idx, parent);  
            self.sift_up(parent);  
        }  
    }  
  
    fn sift_down(&mut self, idx: usize) {  
        let smallest_child = self.smallest_child_idx(idx);  
  
        if smallest_child != idx {  
            self.items.swap(idx, smallest_child);  
            self.sift_down(smallest_child);  
        }  
    }  
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a <= b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a >= b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default+Ord,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.next()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a <= b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a >= b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}