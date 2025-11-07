/*
	heap
	This question requires you to implement a binary heap function
*/
use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    // Note: The first element (index 0) is a dummy value, so items at
    // indices 1 to count are the actual heap elements.
    items: Vec<T>,
    // Comparator returns true if the first argument should be "higher"
    // priority (closer to the root) than the second.
    // For MinHeap: |a, b| a < b
    // For MaxHeap: |a, b| a > b
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            // The 0-th index is unused to simplify parent/child index calculations.
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
        // 1. Insert the new value at the end of the items vector.
        self.items.push(value);
        self.count += 1;
        
        // 2. Bubble up the new element to its correct position.
        self.bubble_up(self.count);
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
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        
        // If there's no left child, we shouldn't have called this (checked by children_present)
        // If there's no right child (or it's beyond the count), the left child is the only one.
        if right > self.count {
            return left;
        }

        // Compare the children using the comparator.
        // Return the index of the child that is "higher priority" (closer to the root).
        if (self.comparator)(&self.items[left], &self.items[right]) {
            left
        } else {
            right
        }
    }

    // Helper to restore the heap property after an insertion.
    fn bubble_up(&mut self, mut idx: usize) {
        // Continue while we are not at the root (index 1) and the element 
        // is "higher priority" than its parent.
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            
            // Check if the current item is "higher priority" than its parent
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
                idx = parent_idx;
            } else {
                break; // Heap property satisfied
            }
        }
    }

    // Helper to restore the heap property after a removal (swapping the last element to the root).
    fn bubble_down(&mut self, mut idx: usize) {
        // Continue while the node has at least one child.
        while self.children_present(idx) {
            let child_idx = self.smallest_child_idx(idx);

            // Check if the child is "higher priority" than the current item
            if (self.comparator)(&self.items[child_idx], &self.items[idx]) {
                self.items.swap(idx, child_idx);
                idx = child_idx;
            } else {
                break; // Heap property satisfied
            }
        }
    }
}

// ---
// Implementations for Min/Max Heap Creation and Iteration
// ---

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
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // Check if the heap is empty (elements are at indices 1 to count)
        if self.count < 1 {
            return None;
        }

        // 1. Get the root element (index 1).
        let root = self.items.swap_remove(1);
        
        // 2. The element that was at the end of the vector is now at index 1.
        // The swap_remove operation correctly handles the removal and puts the
        // last element at the given index.
        
        // Decrement the count of actual heap elements.
        self.count -= 1;

        // 3. Restore the heap property by bubbling down the new root (at index 1), 
        // but only if there are still elements in the heap.
        if self.count > 0 {
            self.bubble_down(1);
        }

        // The root element is returned as the next item.
        Some(root)
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
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), None);
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
        assert_eq!(heap.next(), Some(1));
        assert_eq!(heap.next(), None);
    }
}