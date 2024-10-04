/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;
use std::mem::replace;
pub struct Heap<T>
where
    T: Default+Ord+ std::clone::Clone,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T:Ord+Clone> Heap<T>
where
    T: Default+Ord + std::clone::Clone,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: Vec::with_capacity(1),
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
        self.items.push(value.clone());
        self.items[self.count]=value;
        println!("add_idx->{}",self.count);
        self.count+=1;
        if self.count!=1{
            self.sift_up(self.count-1);
        }
    }
    fn sift_up(&mut self,idx:usize){
        if(idx==0){return;}
        println!("idx->{}->have begin",idx);
        let parent_idx=self.parent_idx(idx);
       // println!("idx->{}->have get parent",idx);
        if idx>0&&(self.comparator)(&self.items[idx],&self.items[parent_idx]){
            println!("!!!!!!!idx->{} try to swap",idx);
            self.items.swap(idx,parent_idx);
            self.sift_up(parent_idx);
        }
        println!("idx->{}->have finish",idx);
    }
    fn parent_idx(&self, idx: usize) -> usize {
        (idx-1) / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) < self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        (idx * 2) + 1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }
    fn has_left_child(&self, idx: usize) -> bool {  
        self.left_child_idx(idx) < self.count  
    }  
    fn has_right_child(&self, idx: usize) -> bool {  
        self.right_child_idx(idx) < self.count  
    }  
    fn left_child(&self, idx: usize) -> &T {  
        &self.items[self.left_child_idx(idx)]  
    }  
    fn right_child(&self, idx: usize) -> &T {  
        &self.items[self.right_child_idx(idx)]  
    }  
    fn sift_down(&mut self,idx:usize)->usize{
        let left=self.left_child_idx(idx);
        let right=self.right_child_idx(idx);
        let smallest=if self.has_left_child(idx){
            let left_child=self.left_child(idx);
            if self.has_right_child(idx){
                let right_child=self.right_child(idx);
                if(self.comparator)(left_child,right_child){
                    left
                }else{
                    right
                }
            }else{
                left
            }
        }else if self.has_right_child(idx){
            right
        }else{
            return idx;
        };
        if (self.comparator)(&self.items[smallest],&self.items[idx]){
            self.items.swap(idx,smallest);
            self.sift_down(smallest)
        }else{
            idx
        }
    }
    fn extract_min(&mut self) -> T {  
        if self.is_empty() {  
            panic!("Cannot extract from an empty heap");  
        }  
        let value=self.items[0].clone();
        self.items[0]=self.items[self.count-1].clone();
        self.count-=1;
        self.sift_down(0);
        value
        //replace(&mut self.items[0], self.items.pop().unwrap_or_default()).sift_down(0)  
    }  
    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		0
    }
}

impl<T:Ord+Clone> Heap<T>
where
    T: Default + Ord + std::clone::Clone,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default+Ord+ std::clone::Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {  
            None  
        } else {  
            Some(self.extract_min())  
        }  
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord+ std::clone::Clone,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord+ std::clone::Clone,
    {
        Heap::new(|a, b| a > b)
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