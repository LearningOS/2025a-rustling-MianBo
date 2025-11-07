/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/
#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            // Vec::remove(0) 是 O(n) 操作，但在 Queue 的实现中这是必须的
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
    q1:Queue<T>,
    q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            q1:Queue::<T>::new(),
            q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        // 确定哪个队列包含所有数据（非空队列）
        let main_q = if !self.q1.is_empty() {
            &mut self.q1
        } else {
            &mut self.q2
        };

        // 如果两个都为空，默认使用 q1
        if main_q.is_empty() {
            self.q1.enqueue(elem);
        } else {
            main_q.enqueue(elem);
        }
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        if self.is_empty() {
            return Err("Stack is empty");
        }

        // 确定主队列和辅助队列
        let (main_q, aux_q) = if !self.q1.is_empty() {
            (&mut self.q1, &mut self.q2)
        } else {
            (&mut self.q2, &mut self.q1)
        };

        // 将所有元素（除了最后一个）从主队列转移到辅助队列
        while main_q.size() > 1 {
            // 元素出队并入队到辅助队列
            // 这里使用 unwrap() 是安全的，因为我们在循环前检查了 size() > 0
            if let Ok(elem) = main_q.dequeue() {
                aux_q.enqueue(elem);
            }
        }

        // 剩下的最后一个元素是栈顶元素，将其出队
        main_q.dequeue()
    }
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_queue(){
        let mut s = myStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}