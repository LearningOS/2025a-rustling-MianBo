/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self
    where
        T: PartialOrd,
    {
        let mut merged_list = Self::new();
        
        let mut current_a = list_a.start.take();
        let mut current_b = list_b.start.take();

        while current_a.is_some() || current_b.is_some() {
            let mut next_node_ptr: Option<NonNull<Node<T>>> = None;
            
            match (current_a, current_b) {
                (Some(a_ptr), Some(b_ptr)) => unsafe {
                    let a_val = &(*a_ptr.as_ptr()).val;
                    let b_val = &(*b_ptr.as_ptr()).val;

                    if a_val <= b_val {
                        next_node_ptr = Some(a_ptr);
                        current_a = (*a_ptr.as_ptr()).next.take();
                        current_b = Some(b_ptr);
                    } else {
                        next_node_ptr = Some(b_ptr);
                        current_a = Some(a_ptr);
                        current_b = (*b_ptr.as_ptr()).next.take();
                    }
                },
                (Some(a_ptr), None) => {
                    next_node_ptr = Some(a_ptr);
                    current_a = unsafe { (*a_ptr.as_ptr()).next.take() };
                    current_b = None;
                },
                (None, Some(b_ptr)) => {
                    next_node_ptr = Some(b_ptr);
                    current_a = None;
                    current_b = unsafe { (*b_ptr.as_ptr()).next.take() };
                },
                (None, None) => break,
            };

            if let Some(mut node_ptr) = next_node_ptr {
                unsafe {
                    (*node_ptr.as_ptr()).next = None; 
                }

                let node_ptr_option = Some(node_ptr);
                
                match merged_list.end {
                    None => merged_list.start = node_ptr_option,
                    Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr_option },
                }
                merged_list.end = node_ptr_option;
                merged_list.length += 1;
            }
        }

        merged_list
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => write!(f, ""),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
        assert_eq!(Some(&1), list.get(0));
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
        assert_eq!(Some(&"A".to_string()), list_str.get(0));
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1,3,5,7];
        let vec_b = vec![2,4,6,8];
        let target_vec = vec![1,2,3,4,5,6,7,8];
        
        for i in 0..vec_a.len(){
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len(){
            list_b.add(vec_b[i]);
        }
        println!("list a: {} list b: {}", list_a,list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
        println!("merged List is: {}", list_c);
        assert_eq!(target_vec.len() as u32, list_c.length);
        for i in 0..target_vec.len(){
            assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
        }
    }
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11,33,44,88,89,90,100];
        let vec_b = vec![1,22,30,45];
        let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

        for i in 0..vec_a.len(){
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len(){
            list_b.add(vec_b[i]);
        }
        println!("list a: {} list b: {}", list_a,list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
        println!("merged List is: {}", list_c);
        assert_eq!(target_vec.len() as u32, list_c.length);
        for i in 0..target_vec.len(){
            assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
        }
    }
    
    #[test]
    fn test_merge_linked_list_empty() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_b = vec![1, 2, 3];
        let target_vec = vec![1, 2, 3];

        for i in 0..vec_b.len(){
            list_b.add(vec_b[i]);
        }

        println!("list a: {} list b: {}", list_a,list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
        println!("merged List is: {}", list_c);
        assert_eq!(target_vec.len() as u32, list_c.length);
        for i in 0..target_vec.len(){
            assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
        }
        
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![4, 5, 6];
        let target_vec = vec![4, 5, 6];

        for i in 0..vec_a.len(){
            list_a.add(vec_a[i]);
        }

        println!("list a: {} list b: {}", list_a,list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
        println!("merged List is: {}", list_c);
        assert_eq!(target_vec.len() as u32, list_c.length);
        for i in 0..target_vec.len(){
            assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
        }
    }
}