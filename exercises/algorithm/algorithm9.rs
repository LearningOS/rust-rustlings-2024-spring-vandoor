/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

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
      self.count += 1;
      let mut idx = self.count;     
      self.items.push(value);
      while idx > 1 &&  (self.comparator)(&self.items[idx], &self.items[self.parent_idx(idx)]){
        let pa = self.parent_idx(idx);
        self.items.swap(idx, pa);
        idx = pa;
      }

    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
      let rid = self.right_child_idx(idx);
      if rid <= self.count && (self.comparator)(&self.items[rid], &self.items[self.left_child_idx(idx)]){
        rid
      }else{
        self.left_child_idx(idx)
      }
      
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
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
    T: Default+Ord,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
      if self.count == 0{
        None
      }
		  else{
        self.items.swap(1, self.count);
        self.count -= 1;
        let ret = self.items.pop().unwrap();
        let mut idx=1;
        while self.children_present(idx) {
          let cid = self.smallest_child_idx(idx);
          if((self.comparator)(&self.items[cid],&self.items[idx])) {self.items.swap(cid, idx)}
          else {break}
          idx = cid;
        }
        Some(ret)
      } 
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
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
        println!("!!!!");
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        println!("hh");
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        println!("aa");
        assert_eq!(heap.next(), Some(9));
        println!("bb");
        assert_eq!(heap.next(), Some(4));
        println!("cc");
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
        println!("dd");
    }
}