/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

use std::cmp::PartialOrd;

fn sort<T: PartialOrd + Copy>(array: &mut [T]) {
    quick_sort(array, 0, array.len() as isize - 1);
}

fn quick_sort<T: PartialOrd + Copy>(array: &mut [T], low: isize, high: isize) {
    if low < high {
        let pivot_index = partition(array, low, high);
        
        quick_sort(array, low, pivot_index - 1);
        
        quick_sort(array, pivot_index + 1, high);
    }
}

fn partition<T: PartialOrd + Copy>(array: &mut [T], low: isize, high: isize) -> isize {
    let mid = (low + high) / 2;
    
    array.swap(mid as usize, high as usize);
    
    let pivot_value = array[high as usize];

    let mut i = low - 1;

    for j in low..high {
        if array[j as usize] <= pivot_value {
            i += 1;
            if i != j {
                array.swap(i as usize, j as usize);
            }
        }
    }

    array.swap((i + 1) as usize, high as usize);
    
    i + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
    #[test]
    fn test_sort_4_empty() {
        let mut vec: Vec<i32> = vec![];
        sort(&mut vec);
        assert_eq!(vec, vec![]);
    }
    #[test]
    fn test_sort_5_duplicates() {
        let mut vec = vec![5, 1, 5, 2, 8, 2, 5];
        sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 2, 5, 5, 5, 8]);
    }
}